use std::collections::HashMap;

/// program logic:
/// take each line, separe each game and compare with max values
/// filter and sum values
#[allow(dead_code)]
pub fn solution(input: &str) -> i32 {
    fn game_is_possible(line: &str) -> Option<i32> {
        let max_values = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

        let game = line.split(':').last().unwrap();
        let sets = game.split(';');

        for set in sets {
            let colors = set.split(',');

            for color in colors {
                let value: i32 = color.split(' ').nth(1).unwrap().parse().unwrap();
                let name = color.split(' ').last().unwrap();

                if value > *max_values.get(name).unwrap() {
                    return None;
                }
            }
        }

        let game_name = line.split(':').next().unwrap();
        let game_number: i32 = game_name.split(' ').last().unwrap().parse().unwrap();

        Some(game_number)
    }

    input.lines().filter_map(|l| game_is_possible(l)).sum()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_solution() {
        const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        const EXPECTED: i32 = 8;

        assert!(solution(TEST_INPUT) == EXPECTED);
    }

    #[test]
    fn test_solution_final() {
        const INPUT: &str = include_str!("../../inputs/day02.txt");
        const EXPECTED: i32 = 2156;

        assert!(solution(INPUT) == EXPECTED);
    }
}
