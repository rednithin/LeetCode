// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
struct NestedIterator {
    flattened_list: Vec<i32>,
    current: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn flatten(nestedList: &Vec<NestedInteger>, flattened_list: &mut Vec<i32>) {
        for item in nestedList {
            match item {
                NestedInteger::Int(x) => flattened_list.push(*x),
                NestedInteger::List(x) => Self::flatten(&x, flattened_list),
            }
        }
    }
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut flattened_list = vec![];
        Self::flatten(&nestedList, &mut flattened_list);
        Self {
            flattened_list,
            current: 0,
        }
    }

    fn next(&mut self) -> i32 {
        self.current += 1;
        return self.flattened_list[self.current - 1];
    }

    fn has_next(&self) -> bool {
        self.current < self.flattened_list.len()
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */