impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut answer = vec![];
        for num in &nums {
            match answer.binary_search(num) {
                Ok(_) => (),
                Err(x) => {
                    if x == answer.len() {
                        answer.push(*num);
                    } else {
                        answer[x] = *num;
                    }
                    if answer.len() >= 3 {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}
