use std::ops::RangeInclusive;

// use std::collections::HashMap;
// use std::str::FromStr;

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
struct Cube {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

impl Cube {
    fn count(&self) -> i64 {
        let x = (self.x.end() + 1 - self.x.start()).abs();
        let y = (self.y.end() + 1 - self.y.start()).abs();
        let z = (self.z.end() + 1 - self.z.start()).abs();
        x * y * z
    }

    fn intersect(&self, other: &Cube) -> Option<Cube> {
        if self.x.start() > other.x.end() || self.x.end() < other.x.start() || // prserve format
           self.y.start() > other.y.end() || self.y.end() < other.y.start() || // preserve format
           self.z.start() > other.z.end() || self.z.end() < other.z.start()
        {
            return None;
        }
        let x_min;
        let x_max;
        let y_min;
        let y_max;
        let z_min;
        let z_max;

        if self.x.start() > other.x.start() {
            x_min = self.x.start();
        } else {
            x_min = other.x.start();
        }

        if self.x.end() > other.x.end() {
            x_max = other.x.end();
        } else {
            x_max = self.x.end();
        }

        if self.y.start() > other.y.start() {
            y_min = self.y.start();
        } else {
            y_min = other.y.start();
        }

        if self.y.end() > other.y.end() {
            y_max = other.y.end();
        } else {
            y_max = self.y.end();
        }

        if self.z.start() > other.z.start() {
            z_min = self.z.start();
        } else {
            z_min = other.z.start();
        }

        if self.z.end() > other.z.end() {
            z_max = other.z.end();
        } else {
            z_max = self.z.end();
        }

        Some(Cube {
            x: *x_min..=*x_max,
            y: *y_min..=*y_max,
            z: *z_min..=*z_max,
        })
    }
}
fn main() {
    let out = part_2(include_str!("../input.txt"));
    println!("part_2: {}", out);
}

fn part_2(input: &str) -> i64 {
    let mut output: Vec<Cuboid> = Vec::new();
    let commands = parse(input).into_iter();

    // for x in &mut nc {
    //     x.remove(c);
    // }
    // if c.on {
    //     nc.push(c.clone());
    // }

    for command in commands {
        let mut cuboid = command.cuboid.clone();
        for c in &mut output {
            c.chomp_overlap(&cuboid);
        }
        if command.on {
            output.push(cuboid);
        }

        // if command.on {
        //     let mut additions: Vec<Cuboid> = Vec::new();
        //     for c in &output {
        //         additions.extend(cuboid.add(c));
        //     }
        //     output.extend(additions);
        //     output.push(cuboid);
        // } else {
        //     for mut c in output.iter_mut() {
        //         c.subtract(&cuboid);
        //     }
        // }
    }

    output.iter().map(|c| c.count()).sum::<i64>()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Command {
    cuboid: Cuboid,
    on: bool,
}

fn parse(input: &str) -> Vec<Command> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let on = match parts.next() {
                Some("on") => true,
                Some("off") => false,
                _ => panic!("lol"),
            };

            let mut parts = parts.next().unwrap().split(",");
            let (x_str, y_str, z_str) = (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            );

            Command {
                cuboid: Cuboid {
                    overlap: vec![],
                    body: Cube {
                        x: str_into_range(x_str),
                        y: str_into_range(y_str),
                        z: str_into_range(z_str),
                    },
                },
                on: on,
            }
        })
        .collect::<Vec<Command>>()
}

fn str_into_range(input: &str) -> RangeInclusive<i64> {
    let mut parts = input.split("..");
    parts.next().unwrap().parse().unwrap()..=parts.next().unwrap().parse().unwrap()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Cuboid {
    body: Cube,
    overlap: Vec<Cuboid>,
}

impl Cuboid {
    fn count(&self) -> i64 {
        let body = self.body.count();
        let overlap: i64 = self.overlap.iter().map(|c| c.count()).sum::<i64>();

        let count = body - (overlap);

        count
    }

    fn intersect(&self, other: &Cuboid) -> Option<Cuboid> {
        if let Some(body) = self.body.intersect(&other.body) {
            Some(Cuboid {
                body,
                overlap: vec![],
            })
        } else {
            None
        }
    }

    fn chomp_overlap(&mut self, other: &Cuboid) {
        if let Some(cuboid) = self.intersect(other) {
            for r in &mut self.overlap {
                r.chomp_overlap(&cuboid);
            }
            self.overlap.push(cuboid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hahahahaha() {
        let input = r#"
on -20..26,-36..17,-47..7
on -20..33,-21..23,-26..28
"#;
        assert_eq!(part_2(input), 139590 + 133650 - 62322);

        let input = r#"
on -20..26,-36..17,-47..7
on -20..33,-21..23,-26..28
on -22..28,-29..23,-38..16
"#;

        assert_eq!(part_2(input), 225476);

        let input = r#"
on -20..26,-36..17,-47..7
on -20..33,-21..23,-26..28
on -22..28,-29..23,-38..16
on -46..7,-6..46,-50..-1
"#;

        let a = Cube {
            x: -20..=26,
            y: -36..=17,
            z: -47..=7,
        };

        let b = Cube {
            x: -20..=33,
            y: -21..=23,
            z: -26..=28,
        };

        let c = Cube {
            x: -22..=28,
            y: -29..=23,
            z: -38..=16,
        };

        let d = Cube {
            x: -46..=7,
            y: -6..=46,
            z: -50..=-1,
        };
        let one = d.intersect(&a).unwrap();
        let two = d.intersect(&b).unwrap();
        let three = d.intersect(&c).unwrap();
        let foo = one.intersect(&two).unwrap().count()
            + two.intersect(&three).unwrap().count()
            + one.intersect(&three).unwrap().count();
        assert_eq!(foo, 64848);

        assert_eq!(part_2(input), 328328);

        let mut a = Cuboid {
            body: Cube {
                x: 11..=13,
                y: 11..=13,
                z: 11..=13,
            },
            overlap: vec![],
        };

        let b = Cuboid {
            body: Cube {
                x: 11..=11,
                y: 11..=11,
                z: 11..=11,
            },
            overlap: vec![],
        };

        // assert_eq!(a.count(), 27);
        // a.subtract(&b);
        // assert_eq!(a.count(), 26);
        // a.subtract(&b);
        // assert_eq!(a.count(), 26);

        // a.add(&b);
        // assert_eq!(a.count(), 27);
    }

    #[test]
    fn test_blerg() {
        let input = r#"
on 10..12,10..12,10..12
on 11..13,11..13,11..13
off 9..11,9..11,9..11
on 10..10,10..10,10..10
"#;
        let mut cuboids = parse(input)
            .iter()
            .map(|x| x.cuboid.clone())
            .collect::<Vec<Cuboid>>();

        let mut d = cuboids.pop().unwrap();
        let c = cuboids.pop().unwrap();
        let mut b = cuboids.pop().unwrap();
        let mut a = cuboids.pop().unwrap();

        a.chomp_overlap(&b);
        assert_eq!(a.count() + b.count(), 46);

        a.chomp_overlap(&c);
        b.chomp_overlap(&c);

        assert_eq!(a.count() + b.count(), 38);

        // let mut blerg = Vec::new();
        // blerg.extend(d.add(&a));
        // blerg.extend(d.add(&b));

        // blerg.push(a);
        // blerg.push(b);
        // blerg.push(d);

        // assert_eq!(blerg.iter().map(|x| x.count()).sum::<i64>(), 39);

        // assert_eq!(part_2(input), 39);
    }

    #[test]
    fn test_lol() {
        let a = Cube {
            x: 11..=13,
            y: 11..=13,
            z: 11..=13,
        };

        let b = Cube {
            x: 11..=11,
            y: 11..=11,
            z: 11..=11,
        };

        let out = a.intersect(&b);
        assert_eq!(
            out.unwrap(),
            Cube {
                x: 11..=11,
                y: 11..=11,
                z: 11..=11,
            }
        );

        let a = Cube {
            x: 13..=14,
            y: 13..=14,
            z: 13..=14,
        };

        assert_eq!(a.count(), 8);

        let b = Cube {
            x: 11..=13,
            y: 11..=13,
            z: 11..=13,
        };
        assert_eq!(b.count(), 27);

        assert_eq!(
            a.intersect(&b).unwrap(),
            Cube {
                x: 13..=13,
                y: 13..=13,
                z: 13..=13
            }
        );

        let a = Cube {
            x: 10..=12,
            y: 10..=12,
            z: 10..=12,
        };

        let b = Cube {
            x: 11..=13,
            y: 11..=13,
            z: 11..=13,
        };

        assert_eq!(
            a.intersect(&b).unwrap(),
            Cube {
                x: 11..=12,
                y: 11..=12,
                z: 11..=12
            }
        );
    }

    #[test]
    fn test_parts() {
        //         let input = r#"
        // on -20..26,-36..17,47..7
        // on -20..33,-21..23,26..28
        // on -22..28,-29..23,38..16
        // on -46..7,-6..46,50..-1
        // on -49..1,-3..46,24..28
        // on 2..47,-22..22,23..27
        // on -27..23,-28..26,21..29
        // on -39..5,-6..47,3..44
        // on -30..21,-8..43,13..34
        // on -22..26,-27..20,29..19
        // off -48..-32,26..41,47..-37
        // on -12..35,6..50,50..-2
        // off -48..-32,-32..-16,15..-5
        // on -18..26,-33..15,7..46
        // off -40..-22,-38..-28,3..41
        // on -16..35,-41..10,47..6
        // off -32..-23,11..30,14..3
        // on -49..-5,-3..45,29..18
        // off 18..30,-20..-8,3..13
        // on -41..9,-7..43,33..15
        // "#;
        //         assert_eq!(part_2(input), 590784);

        let input = r#"
on -5..47,-31..22,-19..33
on -44..5,-27..21,-14..35
on -49..-1,-11..42,-10..38
on -20..34,-40..6,-44..1
off 26..39,40..50,-2..11
on -41..5,-41..6,-36..8
off -43..-33,-45..-28,7..25
on -33..15,-32..19,-34..11
off 35..47,-46..-34,-11..5
on -14..36,-6..44,-16..29
on -57795..-6158,29564..72030,20435..90618
on 36731..105352,-21140..28532,16094..90401
on 30999..107136,-53464..15513,8553..71215
on 13528..83982,-99403..-27377,-24141..23996
on -72682..-12347,18159..111354,7391..80950
on -1060..80757,-65301..-20884,-103788..-16709
on -83015..-9461,-72160..-8347,-81239..-26856
on -52752..22273,-49450..9096,54442..119054
on -29982..40483,-108474..-28371,-24328..38471
on -4958..62750,40422..118853,-7672..65583
on 55694..108686,-43367..46958,-26781..48729
on -98497..-18186,-63569..3412,1232..88485
on -726..56291,-62629..13224,18033..85226
on -110886..-34664,-81338..-8658,8914..63723
on -55829..24974,-16897..54165,-121762..-28058
on -65152..-11147,22489..91432,-58782..1780
on -120100..-32970,-46592..27473,-11695..61039
on -18631..37533,-124565..-50804,-35667..28308
on -57817..18248,49321..117703,5745..55881
on 14781..98692,-1341..70827,15753..70151
on -34419..55919,-19626..40991,39015..114138
on -60785..11593,-56135..2999,-95368..-26915
on -32178..58085,17647..101866,-91405..-8878
on -53655..12091,50097..105568,-75335..-4862
on -111166..-40997,-71714..2688,5609..50954
on -16602..70118,-98693..-44401,5197..76897
on 16383..101554,4615..83635,-44907..18747
off -95822..-15171,-19987..48940,10804..104439
on -89813..-14614,16069..88491,-3297..45228
on 41075..99376,-20427..49978,-52012..13762
on -21330..50085,-17944..62733,-112280..-30197
on -16478..35915,36008..118594,-7885..47086
off -98156..-27851,-49952..43171,-99005..-8456
off 2032..69770,-71013..4824,7471..94418
on 43670..120875,-42068..12382,-24787..38892
off 37514..111226,-45862..25743,-16714..54663
off 25699..97951,-30668..59918,-15349..69697
off -44271..17935,-9516..60759,49131..112598
on -61695..-5813,40978..94975,8655..80240
off -101086..-9439,-7088..67543,33935..83858
off 18020..114017,-48931..32606,21474..89843
off -77139..10506,-89994..-18797,-80..59318
off 8476..79288,-75520..11602,-96624..-24783
on -47488..-1262,24338..100707,16292..72967
off -84341..13987,2429..92914,-90671..-1318
off -37810..49457,-71013..-7894,-105357..-13188
off -27365..46395,31009..98017,15428..76570
off -70369..-16548,22648..78696,-1892..86821
on -53470..21291,-120233..-33476,-44150..38147
off -93533..-4276,-16170..68771,-104985..-24507
"#;
        assert_eq!(part_2(input), 2758514936282235);
    }
}
