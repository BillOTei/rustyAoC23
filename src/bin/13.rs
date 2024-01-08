use array2d::Array2D;
use itertools::Itertools;

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

fn find_smudge(map: &Vec<Vec<char>>) -> Option<usize> {
    let combinations = map.clone().into_iter().enumerate().combinations(2);
    let mut all_mirrored_at = vec![];
    for u in combinations {
        let a = u[0].clone();
        let b = u[1].clone();
        let mut difference: Vec<((usize, usize), char)> = vec![];
        for (x, tile) in a.1.into_iter().enumerate() {
            let c = b.1[x];
            if c != tile {
                difference.push(((x, a.0), tile))
            }
        }
        if difference.len() == 1 {
            let mut tried_map = map.clone();
            let new_char = if difference[0].1 == '#' {
                '.'
            } else {
                '#'
            };
            let ((x, y), _) = difference[0];
            tried_map[y][x] = new_char;
            all_mirrored_at.push(mirrored_at(&tried_map));
        }
    }

    all_mirrored_at
        .iter()
        .find(|&m| m.is_some())
        .and_then(|m| m.clone())
}

fn mirrored_at_2(grid: &Vec<Vec<char>>) -> Option<usize> {
    (1..grid.len()).find(|&offset| {
        let half1 = grid.iter().take(offset).rev();
        let half2 = grid.iter().skip(offset);
        let combined = half1.zip(half2); // the shortest half determines how long this is!
        let differences: usize = combined
            .map(|(row1, row2)| row1.iter().zip(row2.iter()).filter(|(a, b)| a != b).count())
            .sum();

        differences == 1
    })
}

pub fn part_two(input: &str) -> Option<usize> {
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
        if let Some(h_offset) = mirrored_at_2(grid) {
            r += h_offset * 100;
        }
        let cols: Array2D<char> = maps_2d[x].clone();
        if let Some(v_offset) = mirrored_at_2(&cols.as_columns()) {
            r += v_offset
        }
    }

    Some(r)
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
        assert_eq!(result, Some(400));
    }
}
