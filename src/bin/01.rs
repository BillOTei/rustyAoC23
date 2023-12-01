advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut r: u32 = 0;
    for l in lines {
        r += read_str(l)
    }

    Some(r)
}

fn read_str(str: &str) -> u32 {
    let t: Vec<char> = str
        .chars()
        .filter(|c| c.is_digit(10))
        .collect();
    let mut binding = t.first().copied();
    let first = binding.get_or_insert('0');
    let mut binding_last = t.last().copied();
    let last = binding_last.get_or_insert('0');
    let mut r_str = String::new();
    r_str.push(*first);
    r_str.push(*last);
    let r: u32 = r_str.parse().unwrap();

    r
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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
