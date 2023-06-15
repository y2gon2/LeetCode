impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
            let mut nums: Vec<String> = Vec::new();
        let mut chars: Vec<(String, String)> = Vec::new();
        
        for l in logs {
            for n in l.chars().rev() {
                if n.is_numeric() {
                    nums.push(l);
                    break;
                } else {
                    let mut iter = l.splitn(2, ' ');
                    chars.push((iter.next().unwrap().to_string(), iter.next().unwrap().to_string()));
                    break;
                }
            }
        }
        chars.sort_by(|(a0, _), (b0, _)| a0.cmp(b0));
        chars.sort_by(|(_, a1), (_, b1)| a1.cmp(b1));

        let mut result: Vec<String> = Vec::new();
        for (a0, a1) in chars.into_iter() {
            result.push(String::from(a0) + " " + a1.as_str());
        }
        for num in nums.into_iter() {
            result.push(num);
        }

        return result;
    }
}