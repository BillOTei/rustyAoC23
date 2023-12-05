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
        let mut winning_count = 0;
        for played_nb in played_list {
                if winning_list.contains(&played_nb) {
                    winning_count += 1;
                }
        }
        if winning_count > 0 {
            r += 2_u32.pow(winning_count - 1)
        }
    }

    println!("{}", r);

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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
