use array2d::Array2D;

use advent_of_code::get_map;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let maps_list: Vec<Vec<Vec<char>>> = input.split("\n\n").map(|m| m
        .lines()
        .map(|r| r.chars()
            .collect())
        .collect()
    )
        .collect();
    let maps_2d: Vec<Array2D<char>> = input.split("\n\n").clone().map(|m| get_map(m)).collect();
    let mut r = 0;
    for (x, grid) in maps_list.iter().enumerate() {
        let mut h = 0;
        let mut v = 0;
        if let Some(h_offset) = mirrored_at(grid) {
            h = h_offset * 100;
        }
        let cols: Array2D<char> = maps_2d[x].clone();
        if let Some(v_offset) = mirrored_at(&cols.as_columns()) {
            v = v_offset
        }

        if h > v {
            r += h
        } else {
            r += v
        }
    }

    Some(r)
}

fn mirrored_at(map: &Vec<Vec<char>>) -> Option<usize> {
    for offset in 1..map.len() {
        let (left, right) = map.split_at(offset);
        let rev_left = left.iter().rev();
        let mut combined = rev_left.zip(right); // the shortest half determines how long this is!
        if combined.all(|(row1, row2)| row1 == row2) {
            return Some(offset);
        }
    }

    None
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
