impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for token in tokens {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let answer = match &token[..] {
                    "+" => a + b,
                    "*" => a * b,
                    "/" => a / b,
                    "-" => a - b,
                    _ => 0,
                };
                stack.push(answer);
            }
        }
        stack[0]
    }
}
