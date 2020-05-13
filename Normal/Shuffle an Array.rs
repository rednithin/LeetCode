mod rand {
    use std::time;

    #[derive(Debug)]
    pub struct XoShiro256SS {
        state: [u64; 4],
    }

    impl XoShiro256SS {
        pub fn new(seed: Option<u64>) -> Self {
            let seed: u64 = match seed {
                Some(x) => x,
                None => match time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
                    Ok(n) => n.as_secs(),
                    Err(_) => 0,
                },
            };

            let mut splitmix64 = SplitMix64 { state: seed };
            let mut state = [0; 4];

            let tmp = splitmix64.next();
            state[0] = tmp;
            state[1] = tmp >> 32;

            let tmp = splitmix64.next();
            state[2] = tmp;
            state[3] = tmp >> 32;
            Self { state }
        }

        fn rol64(x: u64, k: i32) -> u64 {
            (x << k) | (x >> (64 - k))
        }

        pub fn next(&mut self) -> u64 {
            let result = XoShiro256SS::rol64(self.state[1].wrapping_mul(5), 7).wrapping_mul(9);
            let t = self.state[1] << 17;

            self.state[2] ^= self.state[0];
            self.state[3] ^= self.state[1];
            self.state[1] ^= self.state[2];
            self.state[0] ^= self.state[3];

            self.state[2] ^= t;
            self.state[3] = XoShiro256SS::rol64(self.state[3], 45);

            return result;
        }
    }

    struct SplitMix64 {
        state: u64,
    }

    impl SplitMix64 {
        fn next(&mut self) -> u64 {
            let mut result: u64 = self.state;
            self.state = result.wrapping_add(0x9E3779B97f4A7C15);
            result = (result ^ (result >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            result = (result ^ (result >> 27)).wrapping_mul(0x94D049BB133111EB);
            return result ^ (result >> 31);
        }
    }
}

struct Solution {
    original: Vec<i32>,
    shuffle: Vec<i32>,
    prng: rand::XoShiro256SS,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Self {
            original: nums.clone(),
            shuffle: nums.clone(),
            prng: rand::XoShiro256SS::new(None)
        }
    }
    
    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.original.clone()
    }
    
    /** Returns a random shuffling of the array. */
    fn shuffle(&mut self) -> Vec<i32> {
        for i in 0..self.shuffle.len() {
            let random_index = self.prng.next() as usize % self.shuffle.len();
            self.shuffle.swap(i, random_index)
        }
        self.shuffle.clone()
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */