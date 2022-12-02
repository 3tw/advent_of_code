// Day 2: Rock Paper Scissors
// https://adventofcode.com/2022/day/2

/* My initial attempt was to write control flows, which turned out to be very verbose.
Instead, I decided to explore modulo operator to establish cyclical dependency between Rock, Paper and Scissors.
I achieved this goal with the help of this article:
https://therenegadecoder.com/code/rock-paper-scissors-using-modular-arithmetic/
*/

pub fn run() -> (String, String) {
    fn map_sign(input: &str) -> u8 {
        // Map strings to enable modulo solution and to match the sign with the correct values
        return match input {
            "A" | "X" => 1, // Rock
            "B" | "Y" => 2, // Paper
            "C" | "Z" => 3, // Scissors
            _ => panic!("Could not parse the sign"),
        };
    }

    fn map_sign_part_2(me: &str, opponent: u8) -> u8 {
        return match me {
            "X" => {
                if opponent == 1 {
                    return 3;
                }
                return opponent - 1;
            }
            "Y" => opponent,
            "Z" => {
                if opponent == 3 {
                    return 1;
                }
                return opponent + 1;
            }
            _ => panic!("Could not parse the sign"),
        };
        // let sign = match input {
        //     "A" | "X" => Sign::from_str("R"),
        //     "B" | "Y" => Sign::from_str("P"),
        //     "C" | "Z" => Sign::from_str("S"),
        //     _ => panic!("Could not parse the sign"),
        // };
        // return sign.unwrap();
    }

    fn get_match_points(me: u8, opponent: u8) -> u8 {
        if me == opponent {
            return 3;
        }
        if (me + 1) % 3 == opponent % 3 {
            return 0;
        }
        return 6;
    }

    // Part 1
    let rounds: Vec<u8> = include_str!("input.txt")
        .lines()
        .map(|round| {
            let mut round = round.split(" ");
            let opponent = map_sign(round.next().unwrap());
            let me = map_sign(round.next().unwrap());
            return me + get_match_points(me, opponent);
        })
        .collect();

    let sum_of_rounds = rounds.into_iter().map(|i| i as u32).sum::<u32>();

    // Part 2
    let rounds_part_2: Vec<u8> = include_str!("input.txt")
        .lines()
        .map(|round| {
            let mut round = round.split(" ");
            let opponent = map_sign(round.next().unwrap());
            let me = map_sign_part_2(round.next().unwrap(), opponent);
            return me + get_match_points(me, opponent);
        })
        .collect();

    let sum_of_rounds_2 = rounds_part_2.into_iter().map(|i| i as u32).sum::<u32>();

    // let sum_of_rounds = "ss";
    return (sum_of_rounds.to_string(), sum_of_rounds_2.to_string());
}
