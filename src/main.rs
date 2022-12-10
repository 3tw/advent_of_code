mod days;
mod utils;
use crate::days::AoC;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // Greeting
    println!("\nGo ahead, choose the day!  \n\nHave fun ! \n");

    loop {
        // Get user input
        let user_input = utils::get_input();

        /* Check if the input is a number that corresponds to the possible solutions
        This number can never  n < 1 or 25 < n */
        match user_input.trim().parse::<u32>() {
            Ok(number) => {
                // if number > 0 {
                if let Ok(value) = AoC::run(number as usize) {
                    let (part_one, part_two) = value;
                     println!(
                        "Solutions to day {} \nPart one --> {} \nPart two --> {}",
                        number, part_one, part_two
                    );
                } else {
                    println!("\nTry again. Pick a day between 1 and 25.");
                    continue;
                }
            }
            Err(_) => {
                println!("To select a day you must type in a number.")
            }
        };
    }
}
