
// Arrays and strings
mod question1;
mod question392;
mod question344;
mod question977;
mod question713;


mod question100;
mod question643;
mod question1426;
mod question509;

mod k_distinct_char;
mod question3;
mod question1004;
mod question268;
mod questin49;

mod question104;
mod question1047;
mod question1133;
mod question1189;
mod question1248;
mod question125;
mod question14;
mod question111;
mod question1438;
mod question1544;
mod question1941;
mod question20;
mod question543;
mod question206;
mod question2225;
mod question2248;
mod question2260;
mod question2270;
mod question231;
mod question2342;
mod question2351;
mod question2352;
mod question239;
mod question1448;
mod question24;
mod question26;
mod question27;
mod question28;
mod question346;
mod question35;
mod question383;
mod question49;
mod question496;
mod question525;
mod question560;
mod question66;
mod question71;
mod question739;
mod question771;
mod question83;
mod question844;
mod question876;
mod question9;
mod question901;
mod question933;


mod question1413;
mod question1832;
mod question2090;

mod question1480;
mod question112;
mod question236;
mod question1026;
// Heap
mod question1046;
mod question2208;
mod question480;
mod question1962;
mod question1167;
mod question347;
mod question658;
mod question215;
mod question703;
mod question973;
// Greedy
mod question2126;
mod question2294;
mod question1323;
mod question1710;
mod question1196;
mod question1338;
mod question1481;
mod question881;
// binary search
mod question704;
mod question2389;
mod question74;
mod question2300;
mod question875;
mod question1283;
mod question1631;
// backtracking
mod question46;
mod question77;
mod question78;
mod question17;
mod question63;
mod question64;
mod question198;
// dynamic programming
mod question62;
mod question746;
mod question300;
mod question70;
mod question322;
// graph
mod question547;
mod question200;
mod question1971;
mod question323;
mod question695;
mod question1466;
mod question2368;
mod question797;
mod question1091;

// for interview

mod question53;
mod question121;
mod question217;
mod question238;
mod question242;
mod palindrome;
mod tree_practice;
mod recursion_practice;

mod two_lists;

// Cloudflare interview
mod question15;
mod question18;
mod question622;
mod question289;
mod question1381;
mod question227;


fn main() {
    // let nums = [0,0,1,1,1,2,2,3,3,4]
    // let result = question26::remove_duplicates();
    // println!("{result}");
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tree<T> {
    value: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

impl<T> Tree<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}
