// use std::collections::HashMap;
// use std::str::FromStr;

fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);

    let out = part_2(include_str!("../input.txt"));
    println!("part_2: {}", out);
}

fn part_1(input: &str) -> u64 {
    let _thing = parse(input);
    unimplemented!()
}

fn part_2(input: &str) -> u64 {
    let _thing = parse(input);
    unimplemented!()
}

fn parse(input: &str) {
    unimplemented!()
}

fn add(left: Cell, right: Cell) -> Cell {
    Cell::Vec(vec![left, right])
}

// fn reduce_n(cell: Vec<Cell>, n: u32) -> Cell {
//     if n == 4 {
//         let mut left = cell.get(0).expect("Cell::Vec cannot be empty").clone();
//         let mut right = cell.get(1).expect("Cell::Vec must have 2 elements").clone();
//         match (left, right) {
//             (Cell::Vec(_), Cell::Num(_)) => {}
//             _ => {} // Cell::Vec(vec![Cell::Num(a), Cell::Num(b)]) => {}
//         }
//     };

//     Cell::Vec(vec![])
// }

// fn reduce(cell: Cell) -> Cell {
//     match cell {
//         Cell::Vec(Cell::Vec(Cell::Vec(Cell::Vec(inner)))) => {
//             println!("{:?}", inner);
//             cell
//         }
//         Cell::Num(inner) => cell,
//         _ => panic!("lol"),
//     }
// }

#[derive(Clone, Eq, PartialEq, Debug)]
enum Cell {
    Num(u32),
    Vec(Vec<Cell>),
}

// [4,[3,2]] => [7,0]
fn explode(cell: Vec<Cell>) -> Vec<Cell> {
    let mut iter = cell.into_iter();
    match (iter.next().unwrap(), iter.next().unwrap()) {
        (Cell::Vec(vec), Cell::Num(r_num)) => {
            if let Cell::Num(l_num) = vec[1] {
                vec![Cell::Vec(vec![Cell::Num(0), Cell::Num(l_num + r_num)])]
            } else {
                panic!("")
            }
        }
        (Cell::Num(l_num), Cell::Vec(vec)) => {
            if let Cell::Num(r_num) = vec[0] {
                vec![Cell::Vec(vec![Cell::Num(l_num + r_num), Cell::Num(0)])]
            } else {
                panic!("")
            }
        }
        _ => panic!(""),
    }
}

fn reduce(cell: Cell) -> Cell {
    // let mut cell = cell.clone();
    // match cell {
    //     Cell::Vec(inner) => {
    //         for cell in inner.into_iter() {
    //             let reduce(reduce(reduce(cell.clone())));
    //         }
    //     }
    //     Cell::Num(inner) => {}
    // }
    cell
}

// fn a_to_cell()

// [[1,2],[[3,4],5]] => [1,2] + [[3,4],5]

fn break(cell: Vec<Cell>) -> Vec<Cell> {
    let mut out = Vec::new();

    // let mut stack = Vec::new();
    // let mut frontier = Vec::new();
    // for c in cell.clone() {
    //     frontier.push(c.clone());
    // }
    // while let Some(c) = frontier.pop() {
    //     match c {
    //         Cell::Num(_) => stack.push(c.clone()),
    //         Cell::Vec(cell) => {
    //             for c in cell.clone() {
    //                 frontier.push(c.clone());
    //             }
    //         }
    //     }
    // }
    // stack.reverse();
    // stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrap() {
        // [[6,[5,[4,[3,2]]]],1]
        let input = vec![Cell::Vec(vec![
            Cell::Num(6),
            Cell::Vec(vec![
                Cell::Num(5),
                Cell::Vec(vec![
                    Cell::Num(4),
                    Cell::Vec(vec![Cell::Num(3), Cell::Num(2)]),
                ]),
                Cell::Num(1),
            ]),
        ])];

        let out = unwrap(input);
        println!("{:?}", out);
        panic!("lol");
    }

    #[test]
    fn test_explode() {
        // [[9,8],1] => // [0,9]
        let input = vec![Cell::Vec(vec![Cell::Num(9), Cell::Num(8)]), Cell::Num(1)];
        let expected = vec![Cell::Vec(vec![Cell::Num(0), Cell::Num(9)])];
        assert_eq!(explode(input), expected);
        // [4,[3,2]] => [0,9]
        let input = vec![Cell::Num(4), Cell::Vec(vec![Cell::Num(3), Cell::Num(2)])];
        let expected = vec![Cell::Vec(vec![Cell::Num(7), Cell::Num(0)])];
        assert_eq!(explode(input), expected);
    }

    #[test]
    fn test_reduce() {
        // [
        // [
        // [
        // [
        //[9,8],
        // 1],2],3],4]
        let input = Cell::Vec(vec![
            Cell::Vec(vec![
                Cell::Vec(vec![
                    Cell::Vec(vec![
                        Cell::Vec(vec![Cell::Num(9), Cell::Num(8)]),
                        Cell::Num(1),
                    ]),
                    Cell::Num(2),
                ]),
                Cell::Num(3),
            ]),
            Cell::Num(4),
        ]);
        println!("{:?}", input);
        let expected = Cell::Vec(vec![
            Cell::Vec(vec![
                Cell::Vec(vec![
                    Cell::Vec(vec![Cell::Num(0), Cell::Num(9)]),
                    Cell::Num(2),
                ]),
                Cell::Num(3),
            ]),
            Cell::Num(4),
        ]);
        println!("{:?}", expected);
        let out = reduce(input);
        assert_eq!(out, expected);
    }

    #[test]
    fn test_add() {
        let left = Cell::Vec(vec![Cell::Num(1), Cell::Num(2)]);
        let right = Cell::Vec(vec![
            Cell::Vec(vec![Cell::Num(3), Cell::Num(4)]),
            Cell::Num(5),
        ]);

        let out = add(left, right);
        let expected = Cell::Vec(vec![
            Cell::Vec(vec![Cell::Num(1), Cell::Num(2)]),
            Cell::Vec(vec![
                Cell::Vec(vec![Cell::Num(3), Cell::Num(4)]),
                Cell::Num(5),
            ]),
        ]);

        assert_eq!(out, expected);
    }

    #[test]
    fn test_parts() {
        let input = r#""#;
        // assert_eq!(part_1(input), 99);
        // assert_eq!(part_2(input), 99);
    }
}
