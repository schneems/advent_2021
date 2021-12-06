fn main() {
    let out = part_1(include_str!("../input_1.txt"), 80);
    println!("{}", out);
}

fn pond_day(population: &mut Vec<LanternFish>) -> &mut Vec<LanternFish> {
    let population_len = population.len();
    for i in 0..population_len {
        let fish = &mut population[i];
        if let Some(baby_fishy) = fish.day_tick() {
            population.push(baby_fishy);
        }
    }
    population
}

#[derive(Debug, PartialEq)]
struct LanternFish {
    timer: u64,
}

impl LanternFish {
    fn day_tick(&mut self) -> Option<LanternFish> {
        if self.timer == 0 {
            self.timer = 6;

            Some(LanternFish { timer: 8 })
        } else {
            self.timer -= 1;
            None
        }
    }
}

fn part_1(input: &str, days: u64) -> u64 {
    let mut population: Vec<LanternFish> = Vec::new();
    for value in input.trim().split(",") {
        population.push(LanternFish { timer: value.parse().unwrap() })
    }

    for _ in 0..days {
        pond_day(&mut population);
    }

    population.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_in_the_life_of_a_fish() {
        let mut fish = LanternFish { timer: 3 };

        assert_eq!(fish.day_tick(), None);
        assert_eq!(fish.timer, 2);
        assert_eq!(fish.day_tick(), None);
        assert_eq!(fish.timer, 1);
        assert_eq!(fish.day_tick(), None);
        assert_eq!(fish.timer, 0);
        assert!(fish.day_tick().is_some());
        assert_eq!(fish.timer, 6);
    }

    #[test]
    fn big_fish_small_pond() {
        let fish = LanternFish { timer: 3 };
        let mut population = vec![fish];

        assert_eq!(pond_day(&mut population).len(), 1);
        assert_eq!(pond_day(&mut population).len(), 1);
        assert_eq!(pond_day(&mut population).len(), 1);
        assert_eq!(pond_day(&mut population).len(), 2);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("3,4,3,1,2", 18), 26);
        assert_eq!(part_1("3,4,3,1,2", 80), 5934);
    }
}
