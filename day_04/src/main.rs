use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let out = part_2(include_str!("../input_1.txt"));
    println!("{}", out);
}

fn part_2(input: &str) -> u64 {
    let mut bingo = build_bingo(input);
    let mut score = 0;
    'outer: for val in &bingo.drawing.values {
        for board in &mut bingo.boards {
            if board.is_winner {
                continue;
            }

            board.mark_if(*val);

            if board.is_winner {
                score = board.score(*val);
                // break 'outer
            }
        }
    }
    score
}

struct Board {
    positions: [[u32; 5]; 5],
    marked: [[bool; 5]; 5],
    is_winner: bool,
}

struct Drawing {
    values: Vec<u32>,
}

struct Bingo {
    drawing: Drawing,
    boards: Vec<Board>,
}

impl Board {
    fn has_val(&self, val: u32) -> Option<(usize, usize)> {
        for i in 0..5 {
            for j in 0..5 {
                if self.positions[i][j] == val {
                    return Some((i, j));
                }
            }
        }

        None
    }

    fn score(&self, val: u32) -> u64 {
        let mut score = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    score += self.positions[i][j];
                }
            }
        }

        (score * val).into()
    }

    fn mark_if(&mut self, val: u32) {
        if let Some((i, j)) = self.has_val(val) {
            self.marked[i][j] = true;
            if self.marked[i].into_iter().all(|x| x) {
                self.is_winner = true;
            } else if self.marked[0][j]
                && self.marked[1][j]
                && self.marked[2][j]
                && self.marked[3][j]
                && self.marked[4][j]
            {
                self.is_winner = true;
            }
        }
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut array = [[0u32; 5]; 5];
        let re_spaces = Regex::new(r" +").unwrap();
        for (i, line) in string.trim().lines().enumerate() {
            for (j, val) in re_spaces.split(line.trim()).enumerate() {
                array[i][j] = val.parse()?
            }
        }

        let marked = [[false; 5]; 5];
        Ok(Board {
            positions: array,
            marked: marked,
            is_winner: false,
        })
    }
}

impl FromStr for Drawing {
    type Err = ParseIntError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string
            .trim()
            .split(",")
            .map(str::parse)
            .collect::<Result<_,_>()
            .map(|values| Drawing { values })
    }
}

fn build_bingo(input: &str) -> Bingo {
    let mut parts = input.trim().split("\n\n");
    let val = parts.next().unwrap();
    let drawing = Drawing::from_str(val).unwrap();

    let mut boards = Vec::new();
    for part in parts {
        boards.push(Board::from_str(part).unwrap())
    }

    Bingo { drawing, boards }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_from_str() {
        let mut board = Board::from_str(
            r#"
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
        "#,
        )
        .unwrap();

        assert_eq!(board.positions[0], [22, 13, 17, 11, 0]);
        assert_eq!(board.positions[1], [8, 2, 23, 4, 24]);

        assert_eq!(board.has_val(22), Some((0, 0)));
        assert_eq!(board.has_val(99), None);

        assert_eq!(board.marked[0][0], false);
        board.mark_if(22);
        assert_eq!(board.marked[0][0], true);
    }

    #[test]
    fn bingo_from_str() {
        let input = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
        "#;

        let bingo = build_bingo(input);
        assert_eq!(
            bingo.drawing.values,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        )
    }

    #[test]
    fn drawings_from_str() {
        let input = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
        "#;

        let drawing = Drawing::from_str(input).unwrap();
        assert_eq!(
            drawing.values,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        )
    }

    #[test]
    fn day_in_the_life_of_a_fish() {
        let input = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
        "#;

        assert_eq!(part_1(input), 1924);
    }
}
