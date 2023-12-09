use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (map, instructions) = parse_map(input);

    let mut steps = 0;
    let mut node = "AAA";
    while node != "ZZZ" {
        for instruction in &instructions {
            steps += 1;
            let coordinates = map.get(node).unwrap();
            match instruction {
                'L' => node = coordinates.0,
                'R' => node = coordinates.1,
                _ => {}
            }
        }
    }

    Some(steps)
}

fn parse_map(input: &str) -> (HashMap<&str, (&str, &str)>, Vec<char>) {
    let instructions_map = input.split("\n\n").collect::<Vec<&str>>();
    let instructions = instructions_map[0].chars().collect::<Vec<char>>();
    let map_str = instructions_map[1].split("\n").collect::<Vec<&str>>();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for str in map_str {
        let node_coord = str.split(" = ").collect::<Vec<&str>>();
        let coord = node_coord[1].split(", ").collect::<Vec<&str>>();
        let mut left = coord[0].chars();
        left.next();
        let mut right = coord[1].chars();
        right.next_back();

        map.insert(node_coord[0], (left.as_str(), right.as_str()));
    }

    (map, instructions)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
