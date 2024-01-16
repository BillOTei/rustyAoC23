use indexmap::IndexMap;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let list = input.split(',');
    let mut r = 0usize;

    for instruction in list {
        r += hash(instruction)
    }

    Some(r)
}

fn hash(word: &str) -> usize {
    let mut r = 0u32;
    for c in word.chars() {
        let v = c as u32;
        r += v;
        r *= 17;
        r = r % 256;
    }

    r as usize
}

pub fn part_two(input: &str) -> Option<usize> {
    let list = input.split(',');
    let mut boxes = vec![];
    for _ in 0..256 {
        let map = IndexMap::<&str, u8>::new();
        boxes.push(map);
    }

    for instruction in list {
        if let Some((label, focal_str)) = instruction.split_once('=') {
            let box_idx = hash(label);
            let focal_length = focal_str.parse::<u8>().unwrap();
            let mut b: IndexMap<&str, u8> = boxes[box_idx].clone();
            b.insert(label, focal_length);
            boxes[box_idx] = b;
        } else {
            let (label, _) = instruction.split_once('-').unwrap();
            let box_index = hash(label);
            let mut b: IndexMap<&str, u8> = boxes[box_index].clone();
            b.shift_remove(label);
            boxes[box_index] = b;
        }
    }

    Some(boxes.into_iter().enumerate()
        .map(|(i, b)| {
            let mut focusing_power = 0usize;
            for (p, (_, v)) in b.into_iter().enumerate() {
                focusing_power += (i + 1) * (p + 1) * (v as usize)
            }

            focusing_power
        })
        .sum())
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
        assert_eq!(result, Some(145));
    }
}
