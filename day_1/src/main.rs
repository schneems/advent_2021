use itertools::Itertools;

fn main() {
    let mut count = 0;
    let mut windows = vec![Window {
        sum: std::u32::MAX,
        count: 3,
    }];

    for line in include_str!("../input.txt").lines() {
        let num: u32 = line.parse().unwrap();
        windows.push(Window { sum: 0, count: 0 });

        for window in windows.iter_mut().rev().take(3) {
            window.add(num);
        }

        if let Some((current, last)) = windows
            .iter()
            .rev()
            .filter(|x| x.is_full())
            .take(2)
            .next_tuple()
        {
            if current > last {
                count += 1
            }
        }
    }
    println!("{}", count);
}

#[derive(PartialEq, PartialOrd)]
struct Window {
    sum: u32,
    count: usize,
}

impl Window {
    fn is_full(&self) -> bool {
        self.count == 3
    }

    fn add(&mut self, val: u32) -> &mut Self {
        if !self.is_full() {
            self.sum += val;
            self.count += 1;
        }
        self
    }
}
