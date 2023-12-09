use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (map, instructions) = parse_map_1(input);

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

fn parse_map_1(input: &str) -> (HashMap<&str, (&str, &str)>, Vec<char>) {
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

fn parse_map_2(input: &str) -> (HashMap<&str, (&str, &str)>, Vec<char>, Vec<&str>) {
    let instructions_map = input.split("\n\n").collect::<Vec<&str>>();
    let instructions = instructions_map[0].chars().collect::<Vec<char>>();
    let map_str = instructions_map[1].split("\n").collect::<Vec<&str>>();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut start_nodes = vec![];
    for str in map_str {
        let node_coord = str.split(" = ").collect::<Vec<&str>>();
        let coord = node_coord[1].split(", ").collect::<Vec<&str>>();
        let mut left = coord[0].chars();
        left.next();
        let mut right = coord[1].chars();
        right.next_back();
        if node_coord[0].chars().last().unwrap() == 'A' {
            start_nodes.push(node_coord[0]);
        }

        map.insert(node_coord[0], (left.as_str(), right.as_str()));
    }

    (map, instructions, start_nodes)
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, instructions, start_nodes) = parse_map_2(input);

    let mut paths = HashMap::new();
    for node in start_nodes {
        let mut current_node = node;
        let mut path: Vec<&str> = vec![];
        while current_node.chars().last().unwrap() != 'Z' {
            for instruction in &instructions {
                let coordinates = map.get(current_node).unwrap();
                match instruction {
                    'L' => current_node = coordinates.0,
                    'R' => current_node = coordinates.1,
                    _ => {}
                }
            }
            path.push(current_node);
        }
        paths.insert(node, path);
    }

    Some((paths
        .values()
        .into_iter()
        .map(|v| v.len())
        .reduce(|a, b| lcm(a, b))
        .unwrap()
        * instructions.len()) as u32)
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
        assert_eq!(result, Some(6));
    }
}
