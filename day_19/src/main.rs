use std::collections::HashMap;
use std::collections::HashSet;
// use std::str::FromStr;

fn main() {
    let out = part_1(include_str!("../input.txt"));
    println!("part_1: {}", out);

    let out = part_2(include_str!("../input.txt"));
    println!("part_2: {}", out);
}

fn part_1(input: &str) -> u64 {
    let scanners = parse(input);
    let scanners = fix_scanners(&scanners);
    unique_beacons(scanners).len().try_into().unwrap()
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

    let mut num = 0;
    let mut beacons = Vec::new();
    while let Some(line) = lines.next() {
        let mut chars = line.chars().peekable();
        if chars.peek().is_none() {
            continue;
        }
        if chars.last() == Some('-') {
            if beacons.len() > 0 {
                out.push(Scanner::new(beacons, num));
                beacons = Vec::new();
                num += 1;
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
    num += 1;
    out.push(Scanner::new(beacons, num));
    out[0].position = Some((0, 0, 0));

    out
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Scanner {
    num: i16,
    position: Option<Point>,
    beacons: Vec<Point>,
    pairs: HashMap<i16, (Point, Point)>,
}

impl Scanner {
    fn new(beacons: Vec<Point>, num: i16) -> Self {
        let mut pairs = HashMap::new();
        for one in beacons.iter() {
            for two in beacons.iter() {
                if one != two {
                    pairs.insert(distance(*one, *two), (*one, *two));
                }
            }
        }
        Scanner {
            num,
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

    fn fix_orientation(&mut self, target: &Self) {
        if target.position.is_none() {
            panic!("target position is not known {:?}", target);
        }

        let orientations = gen_orientations();
        let flips = gen_directions();

        let joint = self.find_overlap_becaon(&target, 4);
        let mut pairs = joint.iter().clone().peekable();
        let (self_one, target_one) = pairs.next().unwrap();
        let (self_two, target_two) = pairs.peek().unwrap();

        let b_direction = fdirection_between_points(*target_one, *target_two);
        let mut delta = (None, None);
        'outer: for (j, flip) in flips.iter().enumerate() {
            for (i, axis) in orientations.iter().enumerate() {
                let out = fdirection_between_points(
                    rotate_flip(*self_one, *axis, *flip),
                    rotate_flip(*self_two, *axis, *flip),
                );
                if out == b_direction {
                    delta = (Some(i), Some(j));
                    break 'outer;
                }
            }
        }

        if delta.0.is_none() {}

        let rotation = orientations[delta.0.unwrap()];
        let flip = flips[delta.1.unwrap()];
        let a_fixed = rotate_flip(*self_one, rotation, flip);

        let translation = (
            target_one.0 - a_fixed.0,
            target_one.1 - a_fixed.1,
            target_one.2 - a_fixed.2,
        );

        let mut new_beacons = self.beacons.clone();
        rotate_flip_translate_vec(&mut new_beacons, rotation, flip, translation);
        let scanner = Scanner::new(new_beacons, self.num);

        self.beacons = scanner.beacons;
        self.pairs = scanner.pairs;
        self.position = Some(translation);
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

fn rotate_flip_translate_vec(beacons: &mut Vec<Point>, axis: Point, flip: Point, trans: Point) {
    for i in 0..beacons.len() {
        beacons[i] = rotate_flip(beacons[i], axis, flip);
    }

    for beacon in beacons.iter_mut() {
        beacon.0 += trans.0;
        beacon.1 += trans.1;
        beacon.2 += trans.2;
    }
}

fn rotate_flip(input: Point, axis: Point, flip: Point) -> Point {
    let point = rotate(input, axis);
    (flip.0 * point.0, flip.1 * point.1, flip.2 * point.2)
}

// Figure out how to align

// fn direction_between_points(one: Point, two: Point) -> Point {
//     let dx = (two.0 - one.0) as f64;
//     let dy = (two.1 - one.1) as f64;
//     let dz = (two.2 - one.2) as f64;

//     let heading = dy.atan2(dx);
//     let heading2 = dz.atan2(dy);

//     let a = heading.cos();
//     let b = heading.sin();
//     let c = heading2.sin();
//     (a.signum() as i16, b.signum() as i16, c.signum() as i16)
// }

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

// fn cart_distance(one: Point, two: Point) -> f64 {
//     (((two.0 - one.0) as f64).powf(2.0)
//         + ((two.1 - one.1) as f64).powf(2.0)
//         + ((two.2 - one.2) as f64).powf(2.0))
//     .sqrt()
// }

fn unique_beacons(scanners: Vec<Scanner>) -> HashSet<Point> {
    scanners
        .iter()
        .flat_map(|scanner| scanner.beacons.clone())
        .collect::<HashSet<Point>>()
}

fn fix_scanners(scanners: &Vec<Scanner>) -> Vec<Scanner> {
    let mut not_fixed = scanners.clone();
    not_fixed.reverse();
    let mut fixed = vec![not_fixed.pop().unwrap()];
    while let Some(mut unknown) = not_fixed.pop() {
        let mut found = false;
        for known in fixed.iter() {
            let joint = unknown.find_overlap_becaon(known, 3);
            if joint.len() < 12 {
                continue;
            }
            found = true;

            unknown.fix_orientation(known);
        }

        if found {
            fixed.push(unknown);
        } else {
            not_fixed.insert(0, unknown);
        }
    }

    fixed.sort_by(|a, b| a.num.cmp(&b.num));
    fixed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blerg() {
        let scanners = parse(
            r#"
-- lol --
449,-685,-738
448,-1836,-1882
856,-1641,-752
1853,-1640,-472
1769,-548,-371
1913,-479,-334
1821,-476,-508
2086,-478,-1588
1215,-1201,-1158
1940,-1739,-465
605,-1792,-1821
1799,-1931,-1987
420,-759,-1871
2128,-521,-1771
946,-1646,-703
1694,-1948,-1914
379,-781,-1841
857,-1713,-640
1783,-1750,-508
2083,-601,-1682
472,-862,-1852
1714,-2086,-1984
585,-1856,-1822
431,-699,-732
388,-625,-587

-- lol --
-725,440,-714
634,594,661
732,307,-423
-751,-745,518
727,356,-333
-819,460,-721
623,551,504
-366,621,665
-53,-10,-32
-569,608,654
-656,-561,-730
-610,-809,608
-857,-622,-742
30,-172,47
537,-823,-831
733,587,574
517,-763,-694
-364,523,660
442,-928,520
-759,413,-862
581,-762,-674
-641,-653,496
287,-925,435
425,-855,415
-818,-632,-669
660,419,-422
"#,
        );

        let known = scanners[0].clone();
        let mut unknown = scanners[1].clone();
        unknown.fix_orientation(&known);
    }

    #[test]
    fn test_part_1_given() {
        let scanners = given_case();
        let scanners = fix_scanners(&scanners);
        assert_eq!(&scanners[0].position, &Some((0, 0, 0)));
        assert_eq!(&scanners[1].position, &Some((68, -1246, -43)));
        assert_eq!(&scanners[2].position, &Some((1105, -1205, 1229)));
        assert_eq!(&scanners[3].position, &Some((-92, -2380, -20)));
        assert_eq!(&scanners[4].position, &Some((-20, -1133, 1061)));

        assert_eq!(unique_beacons(scanners).len(), 79);
    }

    #[test]
    fn test_fix() {
        let scanners = given_case();
        let zero = scanners[0].clone();
        let mut one = scanners[1].clone();

        one.fix_orientation(&zero);

        assert_eq!(one.position.unwrap(), (68, -1246, -43));

        let joint = one.find_overlap_becaon(&zero, 3);

        for (a, b) in joint.iter() {
            if a != b {
                panic!("Expected {:?} to eq {:?} but it did not", a, b);
            }
        }
    }

    #[test]
    fn test_rotate_and_locate() {
        let scanners = given_case();
        let zero = scanners[0].clone();
        let one = scanners[1].clone();

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

        let orientations = gen_orientations();
        let flips = gen_directions();
        let mut pairs = joint.iter().clone().peekable();
        let (a_one, b_one) = pairs.next().unwrap();
        let (a_two, b_two) = pairs.peek().unwrap();

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

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
"#,
        )
    }
}
