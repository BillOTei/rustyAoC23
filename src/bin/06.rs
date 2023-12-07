advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let times = parse_data(&lines, "Time: ", 0);
    let distances = parse_data(&lines, "Distance: ", 1);
    let mut r = 1;
    for (id, time) in times.iter().enumerate() {
        let mut ways = 0;
        let record_distance = distances[id];
        for duration in 1u32..*time {
            if duration * (time - duration) > record_distance {
                ways += 1;
            }
        }
        r *= ways
    }

    Some(r)
}

fn parse_data(lines: &Vec<&str>, data: &str, i: usize) -> Vec<u32> {
    lines[i].split(data).filter(|v| *v != "")
        .map(|v| v.split(" ").filter(|v| *v != "").map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()).flatten().collect::<Vec<u32>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let time = parse_data(&lines, "Time: ", 0).iter().map(|t| t.to_string())
        .collect::<Vec<String>>().join("").parse::<u64>().unwrap();
    let distance = parse_data(&lines, "Distance: ", 1).iter().map(|d| d.to_string())
        .collect::<Vec<String>>().join("").parse::<u64>().unwrap();
    let mut ways = 0;
    for duration in 1u64..time {
        if duration * (time - duration) > distance {
            ways += 1;
        }
    }

    Some(ways)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
