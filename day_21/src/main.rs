// use std::collections::HashMap;
// use std::str::FromStr;

fn main() {
    let out = part_1(7, 4);
    println!("part_1: {}", out);

    let out = part_2(7, 4);
    println!("part_2: {}", out);
}

struct Player {
    at: u64,
    score: u64,
}

fn deterministic_roll_three(at: &mut u64, dice: &mut u64) {
    *dice = 1 + ((*dice + 1) - 1) % 100;
    *at += *dice;

    *dice = 1 + ((*dice + 1) - 1) % 100;
    *at += *dice;

    *dice = 1 + ((*dice + 1) - 1) % 100;
    *at += *dice;

    *at = num_to_score(*at);
}

fn part_1(one: u64, two: u64) -> u64 {
    play(one, two)
}

fn part_2(one: u64, two: u64) -> u64 {
    unimplemented!()
}

fn num_to_score(num: u64) -> u64 {
    1 + (num - 1) % 10
}

fn deterministic_roll_three_player(player: &mut Player, dice: &mut u64) {
    deterministic_roll_three(&mut player.at, dice);
    player.score += player.at;
}

fn play(one: u64, two: u64) -> u64 {
    let mut count = 0;
    let mut one = Player { at: one, score: 0 };
    let mut two = Player { at: two, score: 0 };
    let mut dice = 0;
    loop {
        count += 3;
        deterministic_roll_three_player(&mut one, &mut dice);
        // println!("{:?} {}", one.score, dice);
        if one.score >= 1000 {
            break;
        }

        count += 3;
        deterministic_roll_three_player(&mut two, &mut dice);

        if two.score >= 1000 {
            break;
        }
    }
    std::cmp::min(one.score, two.score) * count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_roll_three() {
        let mut one = Player { at: 4, score: 0 };
        let mut two = Player { at: 8, score: 0 };
        let mut dice = 0;

        deterministic_roll_three_player(&mut one, &mut dice);
        assert_eq!(one.score, 10);
        deterministic_roll_three_player(&mut two, &mut dice);
        assert_eq!(two.score, 3);

        deterministic_roll_three_player(&mut one, &mut dice);
        assert_eq!(one.score, 14);
        deterministic_roll_three_player(&mut two, &mut dice);
        assert_eq!(two.score, 9);
    }

    #[test]
    fn test_num_to_score() {
        assert_eq!(num_to_score(12), 2);
        assert_eq!(num_to_score(1), 1);
        assert_eq!(num_to_score(10), 10);
        assert_eq!(num_to_score(22), 2);
    }

    #[test]
    fn test_parts() {
        let input = r#""#;
        assert_eq!(part_1(4, 8), 739785);
        // assert_eq!(part_2(input), 99);
    }
}
