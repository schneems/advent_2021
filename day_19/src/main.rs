use std::collections::HashMap;
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

type Point = (i16, i16, i16);

fn distance(a: Point, b: Point) -> i16 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}

fn parse(input: &str) -> Vec<Scanner> {
    let mut out = Vec::new();
    let mut lines = input.trim().lines().into_iter();

    let mut beacons = Vec::new();
    while let Some(line) = lines.next() {
        let mut chars = line.chars().peekable();
        if chars.peek().is_none() {
            continue;
        }
        if chars.last() == Some('-') {
            if beacons.len() > 0 {
                out.push(Scanner::new(beacons));
                beacons = Vec::new();
            }
            continue;
        }

        let mut nums = line.split(",").into_iter();
        beacons.push((
            nums.next().unwrap().parse().unwrap(),
            nums.next().unwrap().parse().unwrap(),
            nums.next().unwrap().parse().unwrap(),
        ))
    }
    out.push(Scanner::new(beacons));
    out[0].position = Some((0, 0, 0));

    out
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Scanner {
    position: Option<Point>,
    beacons: Vec<Point>,
    pairs: HashMap<i16, (Point, Point)>,
}

impl Scanner {
    fn new(beacons: Vec<Point>) -> Self {
        let mut pairs = HashMap::new();
        for one in beacons.iter() {
            for two in beacons.iter() {
                if one != two {
                    pairs.insert(distance(*one, *two), (*one, *two));
                }
            }
        }
        Scanner {
            beacons,
            pairs,
            position: None,
        }
    }

    fn find_overlap_becaon(&self, other: &Self) -> Vec<(Point, Point)> {
        let mut maybe: HashMap<Point, HashMap<Point, usize>> = HashMap::new();
        for (dist, one) in &self.pairs {
            if let Some(two) = other.pairs.get(&dist) {
                let matches = maybe.entry(one.0.clone()).or_insert(HashMap::new());
                *matches.entry(two.0).or_insert(0) += 1;
                *matches.entry(two.1).or_insert(0) += 1;

                let matches = maybe.entry(one.1.clone()).or_insert(HashMap::new());
                *matches.entry(two.0).or_insert(0) += 1;
                *matches.entry(two.1).or_insert(0) += 1;
            }
        }

        let mut matches = Vec::new();
        for (one, lookup) in maybe {
            if let Some((two, count)) = lookup.iter().max_by_key(|(_, count)| *count) {
                if count >= &2 {
                    matches.push((one, *two));
                }
            }
        }
        // for m
        matches.sort();
        matches
    }
}

fn rotate(point: Point, axis: Point) -> Point {
    let mut out = point.clone();
    for a in [(axis.0, 0, 0), (0, axis.1, 0), (0, 0, axis.2)] {
        if a == (0, 0, 0) {
            continue;
        }
        out = rotate_on_axis(out, a);
    }
    out
}

// https://www.mathworks.com/matlabcentral/answers/123763-how-to-rotate-entire-3d-data-with-x-y-z-values-along-a-particular-axis-say-x-axis
fn rotate_on_axis(point: Point, axis: Point) -> Point {
    // apparently i didn't even need this
    let x = point.0;
    let y = point.1;
    let z = point.2;
    match axis {
        (n, 0, 0) => {
            let degrees = std::f64::consts::FRAC_PI_2 * n as f64;
            let cos = degrees.cos() as i16;
            let sin = degrees.sin() as i16;
            (x, y * cos - z * sin, y * sin + z * cos)
        }
        (0, n, 0) => {
            let degrees = std::f64::consts::FRAC_PI_2 * n as f64;
            let cos = degrees.cos() as i16;
            let sin = degrees.sin() as i16;
            (x * cos + z * sin, y, z * cos - x * sin)
        }
        (0, 0, n) => {
            let degrees = std::f64::consts::FRAC_PI_2 * n as f64;
            let cos = degrees.cos() as i16;
            let sin = degrees.sin() as i16;
            (x * cos - y * sin, x * sin + y * cos, z)
        }
        _ => panic!("Not supported {:?}", axis),
    }
}

// use std::collections::HashSet;

fn gen_orientations() -> Vec<Point> {
    let mut out = Vec::with_capacity(96);
    for x in [1, 2, 3, 4] {
        for z in [1, 2, 3, 4] {
            out.push((x, 0, z));
            out.push((x, 1, z));
            out.push((x, 2, z));
            out.push((x, 3, z));
        }
    }

    for y in [1, 3] {
        for z in [1, 2, 3, 4] {
            out.push((0, y, z));
            out.push((1, y, z));
            out.push((2, y, z));
            out.push((3, y, z));
        }
    }
    out
}

fn rotate_vec(beacons: &mut Vec<Point>, axis: Point) {
    for i in 0..beacons.len() {
        beacons[i] = rotate(beacons[i], axis);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_orientations() {
        let out = rotate((8, 0, 7), (1, 0, 0));
        assert_eq!(out, (8, -7, 0));
        // let out = rotate((8, -7, 0), (0, 1, 0));
        // assert_eq!(out, (0, -7, -8));
        let out = rotate((8, -7, 0), (0, 2, 0));
        assert_eq!(out, (-8, -7, 0));

        let out = rotate((8, 0, 7), (1, 2, 0));
        assert_eq!(out, (-8, -7, 0));

        let out = gen_orientations();
        assert_eq!(out.len(), 96);

        //         let out = &mut parse(
        //             r#"
        // --- scanner 0 ---
        // -1,-1,1
        // -2,-2,2
        // -3,-3,3
        // -2,-3,1
        // 5,6,-4
        // 8,0,7
        //         "#,
        //         )[0];

        // for axis in gen_orientations().iter() {
        //     rotate_vec(&mut out.beacons, *axis);
        //     println!("====");
        //     for b in out.beacons.iter() {
        //         println!("{},{},{}", b.0, b.1, b.2);
        //     }
        // }
    }

    #[test]
    fn test_rotate() {
        assert_eq!(rotate((0, 1, 0), (0, 0, 1)), (-1, 0, 0));
        assert_eq!(rotate((4, 3, 0), (0, 0, 1)), (-3, 4, 0));
        let mut out = (4, 3, 0);
        for _ in 0..4 {
            out = rotate(out, (0, 0, 1))
        }

        assert_eq!(out, (4, 3, 0));
        assert_eq!(rotate((4, 3, 0), (0, 0, 4)), (4, 3, 0));
    }

    #[test]
    fn test_match_points() {
        let scanners = parse(
            r#"
--- scanner 0 ---
0,2,0
4,1,0
3,3,0

--- scanner 1 ---
-1,-1,0
-5,0,0
-2,1,0
"#,
        );

        let matches = scanners[0].find_overlap_becaon(&scanners[1]);
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0], ((0, 2, 0), (-5, 0, 0)));
        assert_eq!(matches[1], ((3, 3, 0), (-2, 1, 0)));
        assert_eq!(matches[2], ((4, 1, 0), (-1, -1, 0)));
    }

    #[test]
    fn test_blerf() {
        let scanners = parse(
            r#"
--- scanner 0 ---
0,2,0
4,1,0
3,3,0

--- scanner 1 ---
-1,-1,0
-5,0,0
-2,1,0
"#,
        );

        assert_eq!(distance((0, 2, 0), (4, 1, 0)), 5);
        assert_eq!(scanners.len(), 2);
        assert_eq!(scanners[0].pairs.get(&5).unwrap(), &((4, 1, 0), (0, 2, 0)));
        assert_eq!(scanners[0].pairs.len(), 3);

        assert_eq!(scanners[1].beacons.len(), 3);
        assert_eq!(scanners[1].pairs.len(), 3);
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance((1, 0, 0), (2, 0, 0)), 1);
        assert_eq!(distance((2, 0, 0), (2, 0, 0)), 0);
    }
}
