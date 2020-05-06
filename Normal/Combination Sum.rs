impl Solution {
    pub fn rec(
        candidates: &Vec<i32>,
        target: i32,
        current: i32,
        index: usize,
        answer: &mut Vec<i32>,
        answers: &mut Vec<Vec<i32>>,
    ) {
        // println!("{:?} {:?} {:?} {:?} {:?} {:?}", candidates, target, current, index, answer, answers);
        let mut current = current;
        if current == target {
            answers.push(answer.clone());
            return;
        }
        if current > target {
            return;
        }
        let mut times = 0;
        while index < candidates.len() {
            answer.push(candidates[index]);
            current += candidates[index];

            times += 1;

            Solution::rec(candidates, target, current, index + 1, answer, answers);
            if (current >= target) {
                for _ in 0..times {
                    answer.pop();
                    current -= candidates[index];
                }
                Solution::rec(candidates, target, current, index + 1, answer, answers);
                return;
            }
        }
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut answers = vec![];
        Solution::rec(&candidates, target, 0, 0, &mut vec![], &mut answers);
        return answers;
    }
}
