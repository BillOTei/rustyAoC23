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

fn read_str_2(str: &str) -> u32 {
    let words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut words_matches = Vec::new();
    for w in words {
        let v: Vec<(usize, &str)> = str.match_indices(w).collect();
        words_matches.push(v);
    }
    words_matches.retain(|v| !v.is_empty());
    let flattened_words = words_matches.into_iter().flatten().collect::<Vec<(usize, &str)>>();
    let mut values: Vec<_> = flattened_words.iter().map(|(i, v)| match v {
        &"one" => (i.clone(), String::from("1")),
        &"two" => (i.clone(), String::from("2")),
        &"three" => (i.clone(), String::from("3")),
        &"four" => (i.clone(), String::from("4")),
        &"five" => (i.clone(), String::from("5")),
        &"six" => (i.clone(), String::from("6")),
        &"seven" => (i.clone(), String::from("7")),
        &"eight" => (i.clone(), String::from("8")),
        &"nine" => (i.clone(), String::from("9")),
        &_ => (i.clone(), String::from("0"))
    }).collect();
    for (x, c) in str.chars().enumerate() {
        if c.is_digit(10) {
            values.push((x, c.to_string()))
        }
    }
    values.sort_by(|(i_a, _v_a), (i_b, _v_b)| i_a.cmp(i_b));
    let binding = values.first();
    let first = binding.map(|(_i, b)| b).unwrap();
    let binding_last = values.last();
    let last = binding_last.map(|(_i, b)| b).unwrap();
    let mut r_str = String::new();
    r_str.push_str(first);
    r_str.push_str(last);

    r_str.parse::<u32>().unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut r: u32 = 0;
    for l in lines {
        r += read_str_2(l)
    }

    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
