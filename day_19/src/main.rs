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

    fn find_overlap_becaon(&self, other: &Self, threshold: usize) -> Vec<(Point, Point)> {
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
                if count >= &threshold {
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

fn gen_directions() -> Vec<Point> {
    vec![(1, 1, 1), (-1, 1, 1), (1, -1, 1), (1, 1, -1)]
}

fn gen_orientations() -> Vec<Point> {
    let mut out = Vec::with_capacity(96);
    for x in [0, 1, 2, 3] {
        for z in [0, 1, 2, 3] {
            out.push((x, 0, z));
        }
    }

    for y in [1, 3] {
        for z in [0, 1, 2, 3] {
            out.push((0, y, z));
        }
    }
    out
}

fn rotate_vec(beacons: &mut Vec<Point>, axis: Point) {
    for i in 0..beacons.len() {
        beacons[i] = rotate(beacons[i], axis);
    }
}

fn rotate_flip(input: Point, axis: Point, flip: Point) -> Point {
    let point = rotate(input, axis);
    (flip.0 * point.0, flip.1 * point.1, flip.2 * point.2)
}

// Figure out how to align

fn direction_between_points(one: Point, two: Point) -> Point {
    let dx = (two.0 - one.0) as f64;
    let dy = (two.1 - one.1) as f64;
    let dz = (two.2 - one.2) as f64;

    let heading = dy.atan2(dx);
    let heading2 = dz.atan2(dy);

    let a = heading.cos();
    let b = heading.sin();
    let c = heading2.sin();
    (a.signum() as i16, b.signum() as i16, c.signum() as i16)
}

fn fdirection_between_points(one: Point, two: Point) -> (f64, f64, f64) {
    let dx = (two.0 - one.0) as f64;
    let dy = (two.1 - one.1) as f64;
    let dz = (two.2 - one.2) as f64;

    let heading = dy.atan2(dx);
    let heading2 = dz.atan2(dy);

    let a = heading.cos();
    let b = heading.sin();
    let c = heading2.sin();
    (a, b, c)
}

fn cart_distance(one: Point, two: Point) -> f64 {
    (((two.0 - one.0) as f64).powf(2.0)
        + ((two.1 - one.1) as f64).powf(2.0)
        + ((two.2 - one.2) as f64).powf(2.0))
    .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rotate_and_locate() {
        let scanners = given_case();
        let zero = scanners[0].clone();
        let one = scanners[1].clone();

        let orientations = gen_orientations();
        let joint = one.find_overlap_becaon(&zero, 3);

        assert_eq!(joint.len(), 12);

        let expected = vec![
            ((686, 422, 578), (-618, -824, -621)),
            ((605, 423, 415), (-537, -823, -458)),
            ((515, 917, -361), (-447, -329, 318)),
            ((-336, 658, 858), (404, -588, -901)),
            ((-476, 619, 847), (544, -627, -890)),
            ((-460, 603, -452), (528, -643, 409)),
            ((729, 430, 532), (-661, -816, -575)),
            ((-322, 571, 750), (390, -675, -793)),
            ((-355, 545, -477), (423, -701, 434)),
            ((413, 935, -424), (-345, -311, 381)),
            ((-391, 539, -444), (459, -707, 401)),
            ((553, 889, -390), (-485, -357, 347)),
        ]
        .into_iter()
        .collect::<HashMap<(i16, i16, i16), (i16, i16, i16)>>();

        for (a, b) in joint.iter() {
            if expected.get(&a) != Some(b) {
                panic!(
                    "Expected mapping {:?} to {:?} to be in {:?} but it was not",
                    a, b, expected
                )
            }
        }

        let mut min_distance = (f64::MAX, 0);
        for (i, axis) in orientations.iter().enumerate() {
            let mut dist = 0.0;
            for (a, b) in &joint {
                dist += cart_distance(rotate(*a, *axis), *b) as f64;
            }

            if (dist, i) < min_distance {
                min_distance = (dist, i);
            }
        }
        let rotation = orientations[min_distance.1];

        let mut pairs = joint.iter().clone().peekable();
        let (a_one, b_one) = pairs.next().unwrap();
        let (a_two, b_two) = pairs.peek().unwrap();
        let a_sig = direction_between_points(*a_one, *a_two);

        let b_sig = direction_between_points(*b_one, *b_two);

        let flips = gen_directions();
        let b_direction = fdirection_between_points(*b_one, *b_two);
        let mut delta = (None, None);
        'outer: for (j, flip) in flips.iter().enumerate() {
            for (i, axis) in orientations.iter().enumerate() {
                let out = fdirection_between_points(
                    rotate_flip(*a_one, *axis, *flip),
                    rotate_flip(*a_two, *axis, *flip),
                );
                if out == b_direction {
                    delta = (Some(i), Some(j));
                    break 'outer;
                }
            }
        }

        let rotation = orientations[delta.0.unwrap()];
        let flip = flips[delta.1.unwrap()];

        let a_fixed = rotate_flip(*a_one, rotation, flip);
        assert_eq!(
            (
                b_one.0 - a_fixed.0,
                b_one.1 - a_fixed.1,
                b_one.2 - a_fixed.2
            ),
            (68, -1246, -43)
        );
    }

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
        assert_eq!(out.len(), 24);

        let mut foo = HashMap::new();
        for axis in out.iter() {
            foo.insert(rotate((1, 2, 3), *axis), 1);
        }

        assert_eq!(foo.len(), 24);
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

        let matches = scanners[0].find_overlap_becaon(&scanners[1], 2);
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

    fn given_case() -> Vec<Scanner> {
        parse(
            r#"
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390
"#,
        )
    }
}
