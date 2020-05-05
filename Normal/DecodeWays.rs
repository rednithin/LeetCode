impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        if s.len() > 0 {
            dp[1] = if s[0..1].parse::<i32>().expect("Fail") > 0 {
                1
            } else {
                0
            }
        }

        for i in 1..s.len() {
            let one = s[i..i + 1].parse::<i32>().expect("Fail 2");
            if one > 0 {
                dp[i + 1] += dp[i];
            }
            let two = s[i - 1..i + 1].parse::<i32>().expect("Fail 3");
            if two >= 10 && two <= 26 {
                dp[i + 1] += dp[i - 1];
            }
            // println!("{} {} {}", i, one, two);
        }
        // println!("{:?}", dp);
        return dp[s.len()];
    }
}
