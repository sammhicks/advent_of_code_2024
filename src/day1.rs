use std::collections::BTreeMap;

pub struct MultiSet<T>(BTreeMap<T, usize>);

impl<T: Ord> MultiSet<T> {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn insert(&mut self, value: T) {
        *self.0.entry(value).or_insert(0) += 1
    }

    fn get(&self, value: &T) -> usize {
        self.0.get(value).copied().unwrap_or(0)
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.0
            .iter()
            .flat_map(|(value, count)| std::iter::repeat_n(value, *count))
    }
}

#[aoc_runner_derive::aoc_generator(day1)]
pub fn parse(input: &str) -> (MultiSet<i32>, MultiSet<i32>) {
    let mut firsts = MultiSet::new();
    let mut seconds = MultiSet::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        let (first, second) = line.split_once(' ').expect("Invalid line");

        firsts.insert(first.trim().parse::<i32>().unwrap());
        seconds.insert(second.trim().parse::<i32>().unwrap());
    }

    (firsts, seconds)
}

#[aoc_runner_derive::aoc(day1, part1)]
pub fn part1((firsts, seconds): &(MultiSet<i32>, MultiSet<i32>)) -> u32 {
    firsts
        .iter()
        .zip(seconds.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum()
}

#[aoc_runner_derive::aoc(day1, part2)]
pub fn part2((firsts, seconds): &(MultiSet<i32>, MultiSet<i32>)) -> usize {
    firsts
        .iter()
        .map(|value| (*value as usize) * seconds.get(value))
        .sum()
}
