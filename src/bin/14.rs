use array2d::Array2D;

use advent_of_code::get_map;

use crate::Direction::{East, South, West};

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
    let row = map.get(x);
    let checked_x = x.checked_sub(1);
    if row.is_none() || checked_x.is_none() {
        return (x, y);
    }
    let c = row.unwrap().get(checked_x.unwrap());
    if c.is_none() {
        return (x, y);
    }
    match c.unwrap() {
        '.' => slide_west(x, checked_x.unwrap(), map),
        _ => (x, y)
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = get_map(input);
    let mut r = 0;
    cycle(&mut map, South);

    for r in map.as_rows() {
        println!("{:?}", r)
    }


    Some(r)
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
        Direction::North => {
            let cols: Vec<Vec<char>> = map.as_columns();
            for (x, col) in cols.iter().enumerate() {
                for (y, c) in col.iter().enumerate() {
                    if c == &'O' {
                        let (slid_x, slid_y) = slide_north(x, y, &cols);
                        update_map(map, x, y, slid_x, slid_y);
                    }
                }
            }
            cycle(map, West)
        }
        West => {
            let rows: Vec<Vec<char>> = map.as_rows();
            for (y, row) in rows.iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    if c == &'O' {
                        let (slid_x, slid_y) = slide_west(x, y, &rows);
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
                while y >= 0 {
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
        East => {}
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
        assert_eq!(result, None);
    }
}
