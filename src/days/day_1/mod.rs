// Day 1: Calorie Counting
// https://adventofcode.com/2022/day/1

pub fn run() -> (String, String) {
    let mut top_three_sums: [u32; 3] = [0; 3];

    include_str!("input.txt").split("\n\n").for_each(|e| {
        // Get sum of all values in a chunk
        let sum_of_chunk: u32 = e.lines().map(|c| c.parse::<u32>().unwrap()).sum();
        // Get max values while iterating to avoid more costly sorting algorithms
        for item in top_three_sums.into_iter().enumerate() {
            let (i, sum) = item;
            if sum < sum_of_chunk {
                let mut cached = sum;
                top_three_sums[i] = sum_of_chunk;
                for index in i + 1..3 {
                  let temp = top_three_sums[index];
                    top_three_sums[index] = cached;
                    cached = temp;
                }
                break;
            }
        }
    });
    
    return (top_three_sums[0].to_string(),  top_three_sums.into_iter().sum::<u32>().to_string());
}
