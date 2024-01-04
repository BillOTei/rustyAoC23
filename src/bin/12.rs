use std::collections::HashMap;

use crate::Spring::{Damaged, Operational, Unknown};

advent_of_code::solution!(12);

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Record {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

impl Record {
    fn new(springs: Vec<Spring>, counts: Vec<usize>) -> Self {
        Self { springs, counts }
    }

    fn valid_arrangements(&self, cache: &mut HashMap<Record, usize>) -> usize {
        if let Some(&solutions) = cache.get(self) {
            return solutions;
        }

        if self.counts.is_empty() {
            let v = match self.springs.iter().any(|c| *c == Damaged) {
                true => 0,
                false => 1
            };

            cache.insert(self.clone(), v);

            return v;
        }

        let needed_space = self.counts.iter().sum::<usize>() + self.counts.len() - 1;
        if self.springs.len() < needed_space {
            cache.insert(self.clone(), 0);

            return 0;
        }

        let first = self.springs[0];
        if first == Operational {
            let result = Self::new(self.springs[1..].to_vec(), self.counts.clone()).valid_arrangements(cache);
            cache.insert(self.clone(), result);

            return result;
        }

        let group = self.counts[0];
        let are_all_non_operational = self.springs[..group].iter().all(|c| *c != Operational);
        let end = (group + 1).min(self.springs.len());

        let mut solutions: usize = 0;

        if are_all_non_operational
            && ((self.springs.len() > group && self.springs[group] != Damaged) || self.springs.len() <= group) {
            solutions += Self::new(self.springs[end..].to_vec(), self.counts[1..].to_vec()).valid_arrangements(cache);
        }

        if first == Unknown {
            solutions += Self::new(self.springs[1..].to_vec(), self.counts.clone()).valid_arrangements(cache);
        }

        cache.insert(self.clone(), solutions);

        solutions
    }
}

fn parse(input: &str, extend: bool) -> Vec<Record> {
    input.lines().map(|line| {
        let (springs, counts) = line.split_once(' ').unwrap();
        let springs: Vec<Spring> = springs
            .chars()
            .map(|c| match c {
                '.' => Operational,
                '#' => Damaged,
                '?' => Unknown,
                _ => panic!("at the disco (nice one)"),
            })
            .collect();
        let mut final_springs = springs.clone();
        let counts: Vec<usize> = counts.split(',').map(|s| s.parse().unwrap()).collect();
        let mut final_counts = counts.clone();
        if extend {
            for _ in 0..4 {
                final_springs.push(Unknown);
                final_springs.append(&mut springs.clone());
                final_counts.append(&mut counts.clone());
            }
        }
        Record { springs: final_springs, counts: final_counts }
    })
        .collect::<Vec<Record>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let records = parse(input, false);
    let mut r = 0usize;
    let mut cache = HashMap::<Record, usize>::new();
    for record in records {
        r += record.valid_arrangements(&mut cache);
    }

    Some(r)
}

pub fn part_two(input: &str) -> Option<usize> {
    let records = parse(input, true);
    let mut r = 0usize;
    let mut cache: HashMap<Record, usize> = HashMap::new();
    for record in records {
        r += record.valid_arrangements(&mut cache)
    }

    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
