//! Multi-tier rate limiter, built up the way the Cloudflare interview stages it:
//!   1. global limit
//!   2. per-path limit
//!   3. per user + path
//!   4. per user + path + tenant
//!
//! Key design choice: every tier is just a `Rule` (data), so each new stage is a
//! new rule, not new `if` branches. A rule says which request fields form the
//! bucket key, optionally which path it applies to, and its limit/window.
//!
//! Algorithm: sliding-window log (exact). A request is allowed only if EVERY
//! matching rule has room; only then is it recorded in each rule's bucket, so
//! rejected requests don't eat quota.

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use std::time::Duration;

/// Request fields a rule can key its bucket on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Ip,
    Method,
    Path,
    User,
    Tenant,
}

#[derive(Debug, Clone)]
pub struct Request<'a> {
    pub ip: &'a str,
    pub method: &'a str,
    pub path: &'a str,
    pub user_id: &'a str,
    pub tenant_id: &'a str,
}

impl Request<'_> {
    fn get(&self, f: Field) -> &str {
        match f {
            Field::Ip => self.ip,
            Field::Method => self.method,
            Field::Path => self.path,
            Field::User => self.user_id,
            Field::Tenant => self.tenant_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    /// `None` = applies to every path.
    pub path: Option<String>,
    /// Fields that make up the bucket key. Empty = one global bucket.
    pub key_fields: Vec<Field>,
    pub limit: usize,
    pub window: Duration,
}

impl Rule {
    fn matches(&self, req: &Request) -> bool {
        self.path.as_deref().is_none_or(|p| p == req.path)
    }

    fn bucket_key(&self, req: &Request) -> String {
        // '\u{1f}' (unit separator) can't collide with normal field contents,
        // unlike ':' which could appear in an IPv6 address or a path.
        self.key_fields
            .iter()
            .map(|f| req.get(*f))
            .collect::<Vec<_>>()
            .join("\u{1f}")
    }
}

pub struct RateLimiter {
    rules: Vec<Rule>,
    /// (rule index, bucket key) -> timestamps (ms) of allowed requests in window.
    buckets: Mutex<HashMap<(usize, String), VecDeque<u64>>>,
}

impl RateLimiter {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules, buckets: Mutex::new(HashMap::new()) }
    }

    /// The interview signature. Uses wall-clock time.
    pub fn is_rate_limited(
        &self,
        ip: &str,
        method: &str,
        path: &str,
        user_id: &str,
        tenant_id: &str,
    ) -> bool {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock before epoch")
            .as_millis() as u64;
        let req = Request { ip, method, path, user_id, tenant_id };
        self.is_rate_limited_at(&req, now_ms)
    }

    /// Same logic with an injected clock, so tests are deterministic.
    pub fn is_rate_limited_at(&self, req: &Request, now_ms: u64) -> bool {
        // One lock around check-then-record so concurrent requests can't both
        // see the last free slot. (Per-bucket locks would scale better.)
        let mut buckets = self.buckets.lock().unwrap();

        let matching: Vec<(usize, String)> = self
            .rules
            .iter()
            .enumerate()
            .filter(|(_, r)| r.matches(req))
            .map(|(i, r)| (i, r.bucket_key(req)))
            .collect();

        // Phase 1: check every matching rule, evicting expired timestamps.
        for (i, key) in &matching {
            let rule = &self.rules[*i];
            let window_ms = rule.window.as_millis() as u64;
            if let Some(q) = buckets.get_mut(&(*i, key.clone())) {
                while q.front().is_some_and(|&t| now_ms.saturating_sub(t) >= window_ms) {
                    q.pop_front();
                }
                if q.len() >= rule.limit {
                    return true; // limited; record nothing
                }
            }
        }

        // Phase 2: all rules have room, so count this request against each.
        for (i, key) in matching {
            buckets.entry((i, key)).or_default().push_back(now_ms);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req<'a>(path: &'a str, user: &'a str, tenant: &'a str) -> Request<'a> {
        Request { ip: "1.2.3.4", method: "GET", path, user_id: user, tenant_id: tenant }
    }

    fn rule(name: &str, path: Option<&str>, keys: &[Field], limit: usize, secs: u64) -> Rule {
        Rule {
            name: name.into(),
            path: path.map(String::from),
            key_fields: keys.to_vec(),
            limit,
            window: Duration::from_secs(secs),
        }
    }

    #[test]
    fn stage1_global_limit() {
        let rl = RateLimiter::new(vec![rule("global", None, &[], 3, 1)]);
        for _ in 0..3 {
            assert!(!rl.is_rate_limited_at(&req("/a", "u1", "t1"), 0));
        }
        // 4th request from anyone, anywhere, is limited.
        assert!(rl.is_rate_limited_at(&req("/b", "u2", "t2"), 500));
        // Window slides: at t=1000 the first three have expired.
        assert!(!rl.is_rate_limited_at(&req("/a", "u1", "t1"), 1000));
    }

    #[test]
    fn stage2_path_limit_only_affects_that_path() {
        let rl = RateLimiter::new(vec![rule("login", Some("/login"), &[], 2, 60)]);
        assert!(!rl.is_rate_limited_at(&req("/login", "u1", "t1"), 0));
        assert!(!rl.is_rate_limited_at(&req("/login", "u2", "t1"), 0));
        assert!(rl.is_rate_limited_at(&req("/login", "u3", "t1"), 0));
        // Other paths are unaffected.
        assert!(!rl.is_rate_limited_at(&req("/home", "u1", "t1"), 0));
    }

    #[test]
    fn stage3_user_plus_path() {
        let rl = RateLimiter::new(vec![rule(
            "user-login", Some("/login"), &[Field::User, Field::Path], 2, 60,
        )]);
        assert!(!rl.is_rate_limited_at(&req("/login", "alice", "t1"), 0));
        assert!(!rl.is_rate_limited_at(&req("/login", "alice", "t1"), 0));
        assert!(rl.is_rate_limited_at(&req("/login", "alice", "t1"), 0));
        // Bob has his own bucket.
        assert!(!rl.is_rate_limited_at(&req("/login", "bob", "t1"), 0));
    }

    #[test]
    fn stage4_user_path_tenant() {
        let rl = RateLimiter::new(vec![rule(
            "user-path-tenant", Some("/api"), &[Field::User, Field::Path, Field::Tenant], 1, 60,
        )]);
        assert!(!rl.is_rate_limited_at(&req("/api", "alice", "acme"), 0));
        assert!(rl.is_rate_limited_at(&req("/api", "alice", "acme"), 0));
        // Same user, different tenant: separate bucket.
        assert!(!rl.is_rate_limited_at(&req("/api", "alice", "globex"), 0));
    }

    #[test]
    fn all_tiers_together_strictest_wins_and_rejects_dont_consume() {
        let rl = RateLimiter::new(vec![
            rule("global", None, &[], 100, 60),
            rule("login", Some("/login"), &[], 10, 60),
            rule("user-login", Some("/login"), &[Field::User, Field::Path], 2, 60),
        ]);
        assert!(!rl.is_rate_limited_at(&req("/login", "alice", "t"), 0));
        assert!(!rl.is_rate_limited_at(&req("/login", "alice", "t"), 0));
        // Alice is blocked by the per-user rule...
        for _ in 0..5 {
            assert!(rl.is_rate_limited_at(&req("/login", "alice", "t"), 0));
        }
        // ...and her rejected attempts did NOT use up the shared /login quota:
        // 2 used so far, so 8 more users fit before /login (limit 10) is full.
        for u in 0..8 {
            assert!(!rl.is_rate_limited_at(&req("/login", &format!("user{u}"), "t"), 0));
        }
        assert!(rl.is_rate_limited_at(&req("/login", "zed", "t"), 0));
    }

    #[test]
    fn public_signature_works() {
        let rl = RateLimiter::new(vec![rule("global", None, &[], 1, 60)]);
        assert!(!rl.is_rate_limited("1.2.3.4", "GET", "/", "u", "t"));
        assert!(rl.is_rate_limited("1.2.3.4", "GET", "/", "u", "t"));
    }
}