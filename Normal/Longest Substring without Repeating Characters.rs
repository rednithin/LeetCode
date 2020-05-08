use std::collections::{
    BTreeMap,   // Sorted Map
    BTreeSet,   // Sorted Set
    BinaryHeap, // Max Heap Implementation
    HashMap,    // Map
    HashSet,    // Set
    LinkedList, // Linked List not to be used anytime
    VecDeque,   // Deque
};

use std::cmp;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut i = 0;
        let mut j = 0;
        let mut set = HashSet::new();
        let chars: Vec<char> = s.chars().into_iter().collect();
        while j < chars.len() {
            // println!("\n{} {}", i, j);
            // println!("1: {:?}", set);
            if i == j && i == 0 {
                set.insert(chars[i]);
                j += 1;
                longest = cmp::max(longest, j - i);
                continue;
            }
            if let Some(x) = set.get(&chars[j]) {
                while chars[i] != chars[j] {
                    // println!("----2: {:?}", set);
                    // println!("----{} {}", chars[i], chars[j]);
                    set.remove(&chars[i]);
                    i += 1
                }
                i += 1;
                j += 1;
            } else {
                set.insert(chars[j]);
                j += 1;
            }
            longest = cmp::max(longest, j - i);
        }
        longest as i32
    }
}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut i = 0;
        let mut j = 0;
        let mut set = HashMap::new();
        let chars: Vec<char> = s.chars().into_iter().collect();
        while j < chars.len() {
            if let Some(x) = set.get_mut(&chars[j]) {
                i = cmp::max(i, *x);
            }
            longest = cmp::max(longest, j - i + 1);
            set.insert(chars[j], j + 1);
            j += 1;
            // println!("{} {} {:?}", i, j, set);
        }
        longest as i32
    }
}
