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
    play_nice(one, two)
}

fn part_2(start_one: u64, start_two: u64) -> u64 {
    let (one, two) = play_lol(start_one, 0, start_two, 0);
    println!("{} {}", one, two);
    std::cmp::max(one, two)
}

fn play_lol(one_at: u64, one_score: u64, two_at: u64, two_score: u64) -> (u64, u64) {
    if two_score >= 21 {
        return (0, 1);
    }

    if one_score >= 21 {
        return (1, 0);
    }

    let mut one_count = 0;
    let mut two_count = 0;
    for (num, multiplier) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let new_one_at = 1 + (num + one_at - 1) % 10;
        let new_one_score = one_score + new_one_at;

        let (out_two, out_one) = play_lol(two_at, two_score, new_one_at, new_one_score);
        one_count += out_one * multiplier;
        two_count += out_two * multiplier;
    }

    (one_count, two_count)
}

fn num_to_score(num: u64) -> u64 {
    1 + (num - 1) % 10
}

fn deterministic_roll_three_player(player: &mut Player, dice: &mut u64) {
    deterministic_roll_three(&mut player.at, dice);
    player.score += player.at;
}

fn play_nice(one: u64, two: u64) -> u64 {
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
    fn test_omfg() {
        // let (one, two) = play_lol(1, 21, 1, 20);
        // assert_eq!(one, 1);
        // assert_eq!(two, 0);

        // let (one, two) = play_lol(1, 20, 1, 21);
        // assert_eq!(one, 0);
        // assert_eq!(two, 1);

        // let (one, two) = play_lol(1, 20, 4, 20);
        // assert_eq!(one, 0);
        // assert_eq!(two, 27);

        println!("============");
        let (one, two) = play_lol(4, 20, 1, 20);
        assert_eq!(one, 27);
        assert_eq!(two, 0);
    }

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
        assert_eq!(part_1(4, 8), 739785);
        assert_eq!(part_2(4, 8), 444356092776315);
    }
}
