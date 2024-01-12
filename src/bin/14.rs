use array2d::Array2D;

use advent_of_code::get_map;

use crate::Direction::{East, North, South, West};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let map = get_map(input);
    let mut cols: Vec<Vec<char>> = map.as_columns().clone();
    let mut r = 0;
    let rows_count = map.row_len();
    for (x, col) in cols.clone().iter().enumerate() {
        for (y, c) in col.iter().enumerate() {
            if c == &'O' {
                let (slid_x, slid_y) = slide_north(x, y, &cols);
                cols[x][y] = '.';
                cols[slid_x][slid_y] = 'O';
                r += rows_count - slid_y
            }
        }
    }

    Some(r)
}

fn slide_north(x: usize, y: usize, map: &Vec<Vec<char>>) -> (usize, usize) {
    let col = map.get(x);
    let checked_y = y.checked_sub(1);
    if col.is_none() || checked_y.is_none() {
        return (x, y);
    }
    let c = col.unwrap().get(checked_y.unwrap());
    if c.is_none() {
        return (x, y);
    }
    match c.unwrap() {
        '.' => slide_north(x, checked_y.unwrap(), map),
        _ => (x, y)
    }
}

fn slide_south(x: usize, y: usize, map: &Vec<Vec<char>>) -> (usize, usize) {
    let col = map.get(x);
    let checked_y = y.checked_add(1);
    if col.is_none() || checked_y.is_none() {
        return (x, y);
    }
    let c = col.unwrap().get(checked_y.unwrap());
    if c.is_none() {
        return (x, y);
    }
    match c.unwrap() {
        '.' => slide_south(x, checked_y.unwrap(), map),
        _ => (x, y)
    }
}

fn slide_west(x: usize, y: usize, map: &Vec<Vec<char>>) -> (usize, usize) {
    let row = map.get(y);
    let checked_x = x.checked_sub(1);
    if row.is_none() || checked_x.is_none() {
        return (x, y);
    }
    let c = row.unwrap().get(checked_x.unwrap());
    if c.is_none() {
        return (x, y);
    }
    match c.unwrap() {
        '.' => slide_west(checked_x.unwrap(), y, map),
        _ => (x, y)
    }
}

fn slide_east(x: usize, y: usize, map: &Vec<Vec<char>>) -> (usize, usize) {
    let row = map.get(y);
    let checked_x = x.checked_add(1);
    if row.is_none() || checked_x.is_none() {
        return (x, y);
    }
    let c = row.unwrap().get(checked_x.unwrap());
    if c.is_none() {
        return (x, y);
    }
    match c.unwrap() {
        '.' => slide_east(checked_x.unwrap(), y, map),
        _ => (x, y)
    }
}

fn map_weight(map: &Array2D<char>) -> usize {
    map.as_rows()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, row)| {
            let round_rocks = row.iter().filter(|tile| **tile == 'O').count();
            round_rocks * (i + 1)
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = get_map(input);
    let mut buffer = vec![map.clone()];
    for _ in 0..1_000_000_000 {
        cycle(&mut map, North);
        if let Some(idx) = buffer.iter().position(|x| x == &map) {
            let cycle_len = buffer.len() - idx;
            let final_idx = idx + (1_000_000_000 - idx) % cycle_len;

            return Some(map_weight(&buffer[final_idx]));
        } else {
            buffer.push(map.clone());
        }
    }

    None
}

enum Direction {
    North,
    West,
    South,
    East,
}

fn update_map(map: &mut Array2D<char>, x: usize, y: usize, slid_x: usize, slid_y: usize) {
    map.set(y, x, '.').expect("panic at the disco map");
    map.set(slid_y, slid_x, 'O').expect("panic at the disco map");
}

fn cycle(map: &mut Array2D<char>, direction: Direction) {
    match direction {
        North => {
            let mut cols: Vec<Vec<char>> = map.as_columns();
            for (x, col) in cols.clone().iter().enumerate() {
                for (y, c) in col.iter().enumerate() {
                    if c == &'O' {
                        let (slid_x, slid_y) = slide_north(x, y, &cols);
                        cols[x][y] = '.';
                        cols[slid_x][slid_y] = 'O';
                        update_map(map, x, y, slid_x, slid_y);
                    }
                }
            }
            cycle(map, West)
        }
        West => {
            let mut rows: Vec<Vec<char>> = map.as_rows();
            for (y, row) in rows.clone().iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    if c == &'O' {
                        let (slid_x, slid_y) = slide_west(x, y, &rows);
                        rows[y][x] = '.';
                        rows[slid_y][slid_x] = 'O';
                        update_map(map, x, y, slid_x, slid_y);
                    }
                }
            }
            cycle(map, South)
        }
        South => {
            let mut cols: Vec<Vec<char>> = map.as_columns();
            for (x, col) in cols.clone().iter().enumerate() {
                let mut y = col.len() - 1;
                loop {
                    let c = col[y];
                    if c == 'O' {
                        let (slid_x, slid_y) = slide_south(x, y, &cols);
                        cols[x][y] = '.';
                        cols[slid_x][slid_y] = 'O';
                        update_map(map, x, y, slid_x, slid_y);
                    }
                    match y.checked_sub(1) {
                        Some(_) => y -= 1,
                        _ => break
                    }
                }
            }
            cycle(map, East)
        }
        East => {
            let mut rows: Vec<Vec<char>> = map.as_rows();
            for (y, row) in rows.clone().iter().enumerate() {
                let mut x = row.len() - 1;
                loop {
                    let c = row[x];
                    if c == 'O' {
                        let (slid_x, slid_y) = slide_east(x, y, &rows);
                        rows[y][x] = '.';
                        rows[slid_y][slid_x] = 'O';
                        update_map(map, x, y, slid_x, slid_y);
                    }
                    match x.checked_sub(1) {
                        Some(_) => x -= 1,
                        _ => break
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
