pub fn parse_columns(mut columns: Vec<&str>) -> Vec<Vec<char>> {
    let number_of_columns = columns
        .pop()
        .unwrap()
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();

    let mut columns_parsed: Vec<Vec<char>> = vec![vec![]; number_of_columns as usize];

    columns.into_iter().for_each(|line| {
        (1..line.len())
            .map(|index| line.chars().nth(index).unwrap())
            .step_by(4)
            .enumerate()
            .filter(|(_, item)| *item != ' ')
            .for_each(|(column, item)| columns_parsed[column].insert(0, item));
    });
    return columns_parsed;
}

pub fn parse_instructions(instructions: Vec<&str>) -> Vec<[usize; 3]> {
    instructions
        .iter()
        .map(|instruction| parse_instruction(*instruction))
        .map(|instruction| [instruction[0], instruction[1], instruction[2]])
        .collect()
}

pub fn parse_instruction(instruction: &str) -> Vec<usize> {
    return instruction
        .split(" ")
        .map(|word| match word.parse::<usize>() {
            Ok(value) => return value,
            Err(_) => return 0,
        })
        .filter(|value| *value > 0)
        .collect::<Vec<usize>>();
}
