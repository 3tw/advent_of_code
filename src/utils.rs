use std::io;

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let input = match input.parse::<String>() {
        Ok(input) => input,
        Err(_) => String::from("Failed to parse user input as String."),
    };

    return input;
}
