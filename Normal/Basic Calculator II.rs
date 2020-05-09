impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut num = 0;
        let mut stack = vec![];
        let mut last_operator = '+';
        for c in (s + "+").chars() {
            match c {
                ' ' => continue,
                ('0'..='9') => num = num * 10 + (c as i32 - '0' as i32),
                _ => {
                    match last_operator {
                        '+' => stack.push(num),
                        '-' => stack.push(-num),
                        '*' => *stack.last_mut().unwrap() *= num,
                        '/' => *stack.last_mut().unwrap() /= num,
                        _ => (),
                    };
                    last_operator = c;
                    num = 0;
                }
            };
        }
        stack.iter().sum()
    }
}
