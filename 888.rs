// https://leetcode.com/problems/fair-candy-swap/solutions/2888042/math-behind-lee215-s-solution/
use std::collections::HashSet;

impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let s1: i32 = a.iter().sum();        
        let s2: i32 = b.iter().sum();
        let diff: i32 = (s1 - s2) / 2;
        let mut set = HashSet::new();

        for i in 0..a.len() {
            set.insert(a[i]);
        }
        for i in 0..b.len() {
            if set.contains(&(b[i] + diff)) {
                return vec![b[i] + diff, b[i]]
            }
        }
        return vec![0, 0]
    }
}
