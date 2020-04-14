impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let total = shift.iter().fold(0, |acc, item| {
            let a = item[0];
            let b = item[1];
            if a == 1 {
                acc - b
            } else {
                acc + b
            }
        });
        let total = total.rem_euclid(s.len() as i32) as usize;
        return String::from(&s[total..]) + &s[..total];
    }
}
