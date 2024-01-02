use crate::Spring::{Damaged, Operational};

advent_of_code::solution!(12);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Record {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

impl Record {
    fn is_valid(&self) -> bool {
        let mut counter = 0;
        let mut res = vec![];
        let mut i = 0usize;
        let len = self.springs.len();
        for spring in &self.springs {
            match spring {
                Spring::Unknown => {
                    return false;
                }
                Damaged => {
                    counter += 1;
                }
                Operational => {
                    if counter > 0 {
                        res.push(counter.clone());
                    }
                    counter = 0
                }
            }
            if counter > 0 && i == len - 1 {
                res.push(counter);
            }
            i += 1;
        }

        res == self.counts
    }

    fn valid_arrangements(&self) -> u32 {
        if let Some(index) = self
            .springs
            .iter()
            .position(|spring| *spring == Spring::Unknown)
        {
            let mut springs_operational = self.springs.clone();
            springs_operational[index] = Operational;
            let op_record = Record {
                springs: springs_operational,
                counts: self.counts.to_vec(),
            };
            let mut springs_broken = self.springs.clone();
            springs_broken[index] = Damaged;
            let broken_record = Record {
                springs: springs_broken,
                counts: self.counts.to_vec(),
            };

            op_record.valid_arrangements() + broken_record.valid_arrangements()
        } else {
            if self.is_valid() {
                1
            } else {
                0
            }
        }
    }
}

fn parse(input: &str) -> Vec<Record> {
    input.lines().map(|line| {
        let (springs, counts) = line.split_once(' ').unwrap();
        let springs = springs
            .chars()
            .map(|c| match c {
                '.' => Operational,
                '#' => Damaged,
                '?' => Spring::Unknown,
                _ => panic!("at the disco (nice one)"),
            })
            .collect();
        let counts = counts.split(',').map(|s| s.parse().unwrap()).collect();

        Record { springs, counts }
    })
        .collect::<Vec<Record>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let records = parse(input);
    let mut r = 0u32;
    for record in records {
        r += record.valid_arrangements();
    }

    Some(r)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
