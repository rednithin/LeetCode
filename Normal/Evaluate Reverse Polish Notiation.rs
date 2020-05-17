impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for token in tokens {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let mut answer = 0;
                match &token[..] {
                    "+" => answer = a + b,
                    "*" => answer = a * b,
                    "/" => answer = a / b,
                    "-" => answer = a - b,
                    _ => (),
                }
                stack.push(answer);
            }
        }
        stack[0]
    }
}
