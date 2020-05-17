impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for token in tokens {
            if ["+", "-", "/", "*"].iter().any(|&symbol| symbol == token) {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let mut answer = 0;
                match token.chars().nth(0).unwrap() {
                    _ if "+" == token => answer = a + b,
                    _ if "*" == token => answer = a * b,
                    _ if "/" == token => answer = a / b,
                    _ if "-" == token => answer = a - b,
                    _ => (),
                }
                stack.push(answer);
            } else {
                stack.push(token.parse::<i32>().unwrap());
            }
        }
        stack[0]
    }
}
