use std::collections::HashMap;

use queues::{IsQueue, Queue, queue};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut r = 0u32;
    for board in rows {
        let parsed_id = board.split(": ");
        let parsed_lists: Vec<&str> = parsed_id.last().unwrap().split(" | ").collect();
        let winning_list_str = parsed_lists.first().unwrap();
        let winning_list: Vec<&str> = winning_list_str.split(" ").filter(|v| *v != "").collect();
        let played_list: Vec<&str> = parsed_lists.last().unwrap().split(" ").filter(|v| *v != "").collect();
        let winning_count = get_winning_count(played_list, winning_list);
        if winning_count > 0 {
            r += 2_u32.pow(winning_count - 1)
        }
    }

    Some(r)
}

fn get_winning_count(played: Vec<&str>, winning: Vec<&str>) -> u32 {
    let mut winning_count = 0;
    for played_nb in played {
        if winning.contains(&played_nb) {
            winning_count += 1;
        }
    }

    winning_count
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut cards_count: HashMap<usize, u32> = HashMap::new();
    let mut q: Queue<usize> = queue![];
    for n in 1..=rows.len() {
        let _ = q.add(n);
        cards_count.insert(n, 1);
    }
    while q.size() > 0 {
        let id = q.peek().unwrap();
        let board = rows[id - 1];
        let parsed_id = board.split(": ");
        let parsed_lists: Vec<&str> = parsed_id.last().unwrap().split(" | ").collect();
        let winning_list_str = parsed_lists.first().unwrap();
        let winning_list: Vec<&str> = winning_list_str.split(" ").filter(|v| *v != "").collect();
        let played_list: Vec<&str> = parsed_lists.last().unwrap().split(" ").filter(|v| *v != "").collect();
        let winning_count = get_winning_count(played_list, winning_list);
        if winning_count > 0 {
            let start = id + 1;
            let end = id + usize::try_from(winning_count).unwrap();
            for i in start..=end {
                let maybe_current_count = cards_count.get(&i);
                if maybe_current_count.is_some() {
                    cards_count.insert(i, maybe_current_count.unwrap() + cards_count.get(&id).unwrap());
                }
            }
        }
        q.remove().unwrap();
    }

    Some(cards_count.values().into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
