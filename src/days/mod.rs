pub mod day_1;
pub mod day_2;
use crate::constants::DAYS_SOLVED;

pub struct AoC;
impl AoC {
    pub fn count_days() -> usize {
        return DAYS_SOLVED;
    }
    pub fn run(index: usize) -> (String, String) {
        let days: [fn() -> (String, String); DAYS_SOLVED] = [day_1::run, day_2::run];
        match days.get(index - 1) {
            Some(problem) => {
                problem()
            }
            None => panic!("This day doesn't exist."),
        }
    }
}
