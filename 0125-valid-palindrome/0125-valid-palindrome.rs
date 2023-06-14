impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut result: bool = true;

        let mut st = String::new();

        for c in s.chars() {
            if c.is_alphanumeric() {
                st.push(c.to_ascii_lowercase());
            }
        }

        while st.len() > 1 {
            let back = st.pop().unwrap();
            let front = st.remove(0);

            if back != front {
                result = false;
                // println!("{}", st);
                break;
            }
        }

        return result;
    }
}