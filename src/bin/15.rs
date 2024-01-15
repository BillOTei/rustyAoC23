advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let list = input.split(',');
    let mut r = 0u64;

    for instruction in list {
        r += hash(instruction)
    }

    Some(r)
}

fn hash(word: &str) -> u64 {
    let mut r = 0u64;
    for c in word.chars() {
        let v = c as u64;
        r += v;
        r *= 17;
        r = r % 256;
    }

    r
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
