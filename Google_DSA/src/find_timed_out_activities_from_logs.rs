/*
You are given a set of activity log records and need to find all activity IDs that have timed out.

Design appropriate classes/data structures (OO design) and implement a function that returns the timed-out activityIds.

Input
A list of log records logs, each containing at least:
- activityId: unique id (string or int)
- timestamp: comparable timestamp (int)
- eventType: e.g. START / END (or equivalent)
- A timeout threshold timeout (e.g., seconds/milliseconds), the maximum allowed duration from start to end.

Task
Return all activityIds whose duration exceeds timeout.

Constraints / edge cases to clarify
logs may be unsorted; if already sorted, discuss optimizations.
There may be missing START or END; state your handling policy.
Clarify whether an activityId can have multiple start/end segments.
Example (illustrative)
timeout = 5
Logs: (A,0,START), (A,10,END), (B,1,START), (B,3,END)
Output: ["A"]
*/



