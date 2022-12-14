use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Monkey {
    pub items: VecDeque<u64>,
    pub inspect_args: (String, String),
    pub divisor: u64,
    pub target: (u8, u8),
    pub inspect_count: u64,
}

impl Monkey {
    pub fn new(
        items: VecDeque<u64>,
        inspect_args: (String, String),
        divisor: u64,
        target: (u8, u8),
    ) -> Self {
        Monkey {
            items,
            inspect_args,
            divisor,
            target,
            inspect_count: 0,
        }
    }

    pub fn push(&mut self, item: u64) {
        self.items.push_back(item)
    }

    /*
     * Process each item and return a vector with moving instructions
     *
     * Each instruction is a tuple (u8, u64) with
     * the number of target monkey (u8)
     * and the item itself (u32)
     *
     * Because numbers get very large in part 2, we divide the number by the largest common denominator,
     * which we get as a product of monkey's divisors used in "match item % self.divisor" (line 51)
     */
    pub fn process(&mut self, operation: &str, n: u64) -> Vec<(u8, u64)> {
        let mut move_queue: Vec<(u8, u64)> = vec![];
        for _ in 0..self.items.len() {
            let item = self.items.pop_front().unwrap();
            let item: u64 = match operation {
                "divide" => self.inspect(item) / n,
                _ => self.inspect(item) % n,
            };

            match item % self.divisor {
                0 => move_queue.push((self.target.0, item)),
                _ => move_queue.push((self.target.1, item)),
            }
            self.inspect_count += 1
        }
        return move_queue;
    }

    fn inspect(&mut self, item: u64) -> u64 {
        let b = self.inspect_args.1.clone();
        let b = match b {
            b if b == "old" => item,
            _ => b.parse::<u64>().unwrap(),
        };

        let operation = self.inspect_args.0.clone();
        match operation.as_str() {
            "*" => return item * b,
            "/" => return item / b,
            "+" => return item + b,
            _ => return item - b,
        }
    }
}
