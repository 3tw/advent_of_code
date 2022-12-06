pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
use crate::constants::DAYS_SOLVED;

pub struct AoC;
impl AoC {
    pub fn run(index: usize) -> (String, String) {
        let days: [fn() -> (String, String); DAYS_SOLVED] = [
            day_1::run,
            day_2::run,
            day_3::run,
            day_4::run,
            day_5::run,
            day_6::run,
        ];
        match days.get(index - 1) {
            Some(problem) => problem(),
            None => panic!("This day doesn't exist."),
        }
    }
}
