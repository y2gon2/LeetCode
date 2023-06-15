impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        *s = s.iter().rev().map(|c| *c).collect(); 
        
    }
}