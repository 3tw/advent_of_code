pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_10;
pub mod day_11;
pub mod day_12;

pub struct AoC;
impl AoC {
    pub fn run(index: usize) -> Result<(String, String), &'static str> {
        let days: Vec<fn() -> (String, String)> = vec![
            day_1::run,
            day_2::run,
            day_3::run,
            day_4::run,
            day_5::run,
            day_6::run,
            day_7::run,
            day_8::run,
            day_9::run,
            day_10::run,
            day_11::run,
            day_12::run,
        ];
        match days.get(index - 1) {
            Some(problem) => Ok(problem()),
            None => Err("This day doesn't exist."),
        }
    }
}
