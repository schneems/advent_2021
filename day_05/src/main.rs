fn main() {
    let out = part_1(include_str!("../input_1.txt"));
    println!("{}", out);
}

#[derive(PartialEq, Eq,Copy, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(PartialEq, Eq,Copy, Clone, Debug)]
struct Point {
    x: i16,
    y: i16,
}

impl Line {
    fn from_str(line: &str) -> Self {
        if let Some((Some(start), Some(end))) = line.split_once("->").map(|(start, end)| {
            (
                start
                    .trim()
                    .split_once(",")
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .map(|(x, y)| Point { x, y }),
                end.trim()
                    .split_once(",")
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .map(|(x, y)| Point { x, y }),
            )
        }) {
            Line { start, end }
        } else {
            panic!("nope")
        }
    }

    fn from_input(input: &str) -> Vec<Self> {
        let mut out = Vec::new();
        for line in input.trim().lines() {
            out.push(Line::from_str(line));
        }
        out
    }
}

fn apply(line: &Line, grid: &mut Vec<Vec<i32>>) {
    // unimplemented!();
    let x_dir: i16 = match line.start.x.cmp(&line.end.x) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    };
    let y_dir: i16 = match line.start.y.cmp(&line.end.y) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    };


    let mut point = line.start.clone();
    while point != line.end {
        // println!("  at        {:?}", point);
        // println!("  going to  {:?}", line.end);

        grid[point.x as usize][point.y as usize] += 1;
        if point.x != line.end.x {
            point.x += x_dir;
        }
        if point.y != line.end.y {
            point.y += y_dir;
        }
    }

    grid[line.end.x as usize][line.end.y as usize] += 1;
}

fn part_1(input: &str) -> u32 {
    let lines = Line::from_input(input);
    let mut grid = vec![vec![0; 1000]; 1000];

    for line in lines {
        // if line.start.x == line.end.x || line.start.y == line.end.y {
            apply(&line, &mut grid);
        // }
    }
    // debug_grid(&grid);

    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] >= 2{
                count+=1;
            };
        }
    }
    count
}

fn debug_grid(grid: &Vec<Vec<i32>>) {
    println!("");
    for x in 0..grid.len() {
        println!("{:?}", grid[x])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lol() {
        let line = Line::from_str("403,277 -> 403,802");
        assert_eq!(line.start, Point { x: 403, y: 277 });
        assert_eq!(line.end, Point { x: 403, y: 802 });
    }

    #[test]
    fn lulz_full() {
        let input = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
        "#;

        // let lines = Line::from_input(input);
        // assert_eq!(lines.len(), 10);

        // let mut grid = vec![vec![0; 10]; 10];
        // debug_grid(&grid);

        // for line in lines {
        //     // println!("\n{:?}", line);
        //     // apply(&line, &mut grid);
        //     debug_grid(&grid);
        // }
        // debug_grid(&grid);
        assert_eq!(part_1(input), 12);
    }

    #[test]
    fn horizontal() {
        let input = r#"
1,1 -> 1,3
        "#;

        let lines = Line::from_input(input);
        assert_eq!(lines.len(), 1);

        let mut grid = vec![vec![0; 10]; 10];

        // debug_grid(&grid);

        for line in lines {
            apply(&line, &mut grid);
        }
        // debug_grid(&grid);

        assert_eq!(grid[1][1], 1);
        assert_eq!(grid[1][2], 1);
        assert_eq!(grid[1][3], 1);
    }
    #[test]
    fn vertical() {
        let input = r#"
9,7 -> 7,7
        "#;

        let lines = Line::from_input(input);
        assert_eq!(lines.len(), 1);

        let mut grid = vec![vec![0; 10]; 10];

        // debug_grid(&grid);

        for line in lines {
            apply(&line, &mut grid);
        }
        assert_eq!(grid[9][7], 1);
        assert_eq!(grid[8][7], 1);
        assert_eq!(grid[7][7], 1);
    }

}
