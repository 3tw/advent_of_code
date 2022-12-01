pub mod day_1;

pub struct AoC;
impl AoC {
    pub fn count_days() -> usize {
        return 1;
    }

    pub fn run(index: usize) -> (String, String) {
        let days: [fn() -> (String, String); 1] = [day_1::run];
        match days.get(index - 1) {
            Some(problem) => {
                println!("\nYou have chosen day {}.", index);
                println!("Don't forget to type 'run' to confirm the input for the puzzle");
                problem()
            }
            None => panic!("This day doesn't exist."),
        }
    }
}
