use std::collections::HashMap;

impl Solution {
    pub fn rec(
        map: &HashMap<i32, i32>,
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
        if index >= candidates.len() {
            return;
        }
        for _ in 0..*map.get(&candidates[index]).unwrap() {
            answer.push(candidates[index]);
            current += candidates[index];

            times += 1;

            Solution::rec(map, candidates, target, current, index + 1, answer, answers);
            if (current >= target) {
                break;
            }
        }
        for _ in 0..times {
            answer.pop();
            current -= candidates[index];
        }
        Solution::rec(map, candidates, target, current, index + 1, answer, answers);
    }
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut answers = vec![];
        let mut map = HashMap::new();
        for num in &candidates {
            if let Some(x) = map.get_mut(num) {
                *x = *x + 1;
            } else {
                map.insert(*num, 1 as i32);
            }
        }
        let candidates: Vec<i32> = map.keys().into_iter().map(|a| *a).collect();
        // println!("{:?}", candidates);
        Solution::rec(&map, &candidates, target, 0, 0, &mut vec![], &mut answers);
        return answers;
    }
}
