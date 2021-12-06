fn main() {
    println!("Hello, world!");
}

struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl Line {
    fn from(line: &str) -> Self {
        if let Some((Some(start), Some(end))) = line.split_once("->").map(|(start, end)| {
            (
                start
                    .trim()
                    .split_once(",")
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())),
                end.trim()
                    .split_once(",")
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())),
            )
        }) {
            Line { start, end }
        } else {
            panic!("nope")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lol() {
        let line = Line::from("403,277 -> 403,802");
        assert_eq!(line.start, (403, 277));
        assert_eq!(line.end, (403, 802));
    }
}
