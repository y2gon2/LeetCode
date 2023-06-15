impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        for i in 0..len / 2 {
            let temp = s[i];
            s[i] = s[len - 1 - i];
            s[len - 1 - i] = temp;
        }
        
    }
}