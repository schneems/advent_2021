fn main() {
    let out = part_2(include_str!("../input_1.txt"), 256);
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
struct FishLifeTimer {
    day_array: [u64; 9]
}

impl FishLifeTimer {
    fn new(fishies: Vec<LanternFish>) -> Self {
        let mut array = [0; 9];
        for fish in fishies {
            array[fish.timer] += 1
        }

        FishLifeTimer {
            day_array: array
        }
    }

    fn day_tick(&mut self) -> & mut Self {
        let new_fish_count = self.day_array[0];
        let mut array = [0; 9];
        array[8] = new_fish_count;
        array[7] = self.day_array[8];
        array[6] = self.day_array[7] + new_fish_count;
        array[5] = self.day_array[6];
        array[4] = self.day_array[5];
        array[3] = self.day_array[4];
        array[2] = self.day_array[3];
        array[1] = self.day_array[2];
        array[0] = self.day_array[1];

        self.day_array = array;
        self
    }

    fn size(&self) -> u64 {
        let mut count = 0;
        for i in 0..self.day_array.len() {
            count += self.day_array[i]
        }
        count
    }
}

#[derive(Debug, PartialEq)]
struct LanternFish {
    timer: usize,
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

fn part_2(input: &str, days: u64) -> u64 {
    let mut population: Vec<LanternFish> = Vec::new();
    for value in input.trim().split(",") {
        population.push(LanternFish { timer: value.parse().unwrap() })
    }

    let mut fish_life = FishLifeTimer::new(population);

    for _ in 0..days {
        fish_life.day_tick();
    }

    fish_life.size()
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


        let fish = LanternFish { timer: 3 };
        let mut fish_life = FishLifeTimer::new(vec![fish]);
        assert_eq!(fish_life.day_array[3], 1);
        assert_eq!(fish_life.day_tick().size(), 1);
        assert_eq!(fish_life.day_array[2], 1);
        assert_eq!(fish_life.day_tick().size(), 1);
        assert_eq!(fish_life.day_array[1], 1);
        assert_eq!(fish_life.day_tick().size(), 1);
        assert_eq!(fish_life.day_array[0], 1);
        assert_eq!(fish_life.day_tick().size(), 2);
        assert_eq!(fish_life.day_array[6], 1);
        assert_eq!(fish_life.day_array[8], 1);

        assert_eq!(fish_life.day_tick().size(), 2);
        assert_eq!(fish_life.day_array[5], 1);
        assert_eq!(fish_life.day_array[7], 1);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_2("3,4,3,1,2", 18), 26);
        assert_eq!(part_2("3,4,3,1,2", 80), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2("3,4,3,1,2", 256), 26984457539);
    }
}
