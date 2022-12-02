mod constants;
mod days;
mod utils;
use crate::days::AoC;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let days_total = AoC::count_days();

    // Greeting
    println!(
        "\nGo ahead, choose the day! \nThere are {} days available. \n\nHave fun ! \n",
        days_total
    );

    loop {
        // Get user input
        let user_input = utils::get_input();

        /* Check if the input is a number that corresponds to the possible solutions
        This number can never  n < 1 or 25 < n */
        match user_input.trim().parse::<u32>() {
            Ok(number) => {
                if number > 0 && number <= days_total as u32 {
                    let (part_one, part_two) = AoC::run(number as usize);
                    println!(
                        "Solutions to day {} \nPart one --> {} \nPart two --> {}",
                        number, part_one, part_two
                    );
                } else {
                    println!("\nTry again. Pick a number between 1 and {}", days_total);
                    continue;
                }
            }
            Err(_) => {
                println!("To select a day you must type in a number.")
            }
        };
    }
}
