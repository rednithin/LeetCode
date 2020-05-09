impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut present = false;
        let mut positive = true;
        let mut number: i32 = 0;

        for c in str.chars() {
            match c {
                '+' if !present => {
                    present = true;
                    continue;
                }
                '-' if !present => {
                    present = true;
                    positive = false;
                    continue;
                }
                ' ' if !present => continue,
                '0'..='9' => {
                    present = true;

                    if let Some(x) = number
                        .checked_mul(10)
                        .and_then(|x| x.checked_add(c as i32 - '0' as i32))
                    {
                        number = x;
                    } else {
                        return if positive {
                            i32::max_value()
                        } else {
                            i32::min_value()
                        };
                    }
                }
                _ => break,
            }
        }
        if positive {
            return number;
        } else {
            return -number;
        }
    }
}
