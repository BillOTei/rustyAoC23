use std::collections::HashSet;
use std::collections::VecDeque;

use array2d::Array2D;
use itertools::Itertools;

use advent_of_code::get_map;

use crate::Direction::{Down, Left, Right, Up};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    let map = get_map(input);
    let mut visited = HashSet::new();
    let mut starts: VecDeque<Point> = VecDeque::new();
    let mut seen = HashSet::new();
    let current_point = Point { x: 0, y: 0, c: map.get(0, 0).unwrap().to_ascii_lowercase(), direction: Right };
    starts.push_back(current_point.clone());
    seen.insert(current_point);

    while let Some(start) = starts.pop_front() {
        let next_starts = do_move(start, &mut visited, &map);
        if next_starts.len() == 2 {
            for p in next_starts {
                if !seen.contains(&p) {
                    seen.insert(p.clone());
                    starts.push_back(p.clone());
                }
            }
        }
    }

    Some(visited.iter().sorted().len())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
    c: char,
    direction: Direction,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
#[derive(Ord, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
}

fn do_move(start: Point, visited: &mut HashSet<Pos>, grid: &Array2D<char>) -> Vec<Point> {
    let mut current_point = Point { x: start.x, y: start.y, c: start.c, direction: start.direction };
    let mut seen = HashSet::new();
    let mut prev_len = seen.len();
    loop {
        visited.insert(Pos { x: current_point.x, y: current_point.y });
        let (next_points, split) = process_move(current_point.clone(), &grid);
        if next_points.len() == 2 || next_points.is_empty() {
            return next_points;
        }
        for p in next_points.clone() {
            visited.insert(Pos { x: p.x, y: p.y });
            seen.insert(Pos { x: p.x, y: p.y });
        }
        if split && seen.len() == prev_len {
            return next_points;
        }
        prev_len = seen.len();
        current_point = next_points[0].clone();
    }
}

fn process_move(point: Point, grid: &Array2D<char>) -> (Vec<Point>, bool) {
    let Point { x, y, c, direction } = point;
    let checked_x_sub = x.checked_sub(1);
    let checked_y_sub = y.checked_sub(1);
    let right = grid.get(y, x + 1).map(|c| Point { x: x + 1, y, c: *c, direction: Right });
    let left = checked_x_sub.and_then(|x| grid.get(y, x)).map(|c| Point { x: x - 1, y, c: *c, direction: Left });
    let up = checked_y_sub.and_then(|y| grid.get(y, x)).map(|c| Point { x, y: y - 1, c: *c, direction: Up });
    let down = grid.get(y + 1, x).map(|c| Point { x, y: y + 1, c: *c, direction: Down });
    let pass_through_right = right.iter().map(|v| v.clone()).collect::<Vec<_>>();
    let pass_through_left = left.iter().map(|v| v.clone()).collect::<Vec<_>>();
    let pass_through_up = up.iter().map(|v| v.clone()).collect::<Vec<_>>();
    let pass_through_down = down.iter().map(|v| v.clone()).collect::<Vec<_>>();
    let mut split_vertical = pass_through_up.clone();
    split_vertical.append(&mut pass_through_down.clone());
    let mut split_horizontal = pass_through_right.clone();
    split_horizontal.append(&mut pass_through_left.clone());

    match c {
        '.' => match direction {
            Right => (pass_through_right, false),
            Left => (pass_through_left, false),
            Up => (pass_through_up, false),
            Down => (pass_through_down, false)
        },
        '/' => match direction {
            Right => (pass_through_up, false),
            Left => (pass_through_down, false),
            Up => (pass_through_right, false),
            Down => (pass_through_left, false)
        },
        '\\' => match direction {
            Right => (pass_through_down, false),
            Left => (pass_through_up, false),
            Up => (pass_through_left, false),
            Down => (pass_through_right, false)
        },
        '|' => match direction {
            Right => (split_vertical, true),
            Left => (split_vertical, true),
            Up => (pass_through_up, false),
            Down => (pass_through_down, false)
        },
        '-' => match direction {
            Right => (pass_through_right, false),
            Left => (pass_through_left, false),
            Up => (split_horizontal, true),
            Down => (split_horizontal, true),
        },
        _ => (vec![], false)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = get_map(input);
    let last_y = map.row_len() - 1;
    let last_x = map.column_len() - 1;
    let mut all_starts = vec![];
    let mut r = 0;
    for (y, _) in map.as_rows().iter().enumerate() {
        let current_point = Point { x: 0, y, c: map.get(y, 0).unwrap().to_ascii_lowercase(), direction: Right };
        all_starts.push(current_point.clone());
        let last_current = Point { x: last_x, y, c: map.get(y, last_x).unwrap().to_ascii_lowercase(), direction: Left };
        all_starts.push(last_current.clone());
    }
    for (x, _) in map.as_columns().iter().enumerate() {
        let current_point = Point { x, y: 0, c: map.get(0, x).unwrap().to_ascii_lowercase(), direction: Down };
        all_starts.push(current_point.clone());
        let last_current = Point { x, y: last_y, c: map.get(last_y, x).unwrap().to_ascii_lowercase(), direction: Up };
        all_starts.push(last_current.clone());
    }
    for start_point in all_starts {
        let mut visited = HashSet::new();
        let mut starts: VecDeque<Point> = VecDeque::new();
        let mut seen = HashSet::new();
        starts.push_back(start_point.clone());
        seen.insert(start_point);

        while let Some(start) = starts.pop_front() {
            let next_starts = do_move(start, &mut visited, &map);
            if next_starts.len() == 2 {
                for p in next_starts {
                    if !seen.contains(&p) {
                        seen.insert(p.clone());
                        starts.push_back(p.clone());
                    }
                }
            }
        }

        if visited.len() > r {
            r = visited.len();
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
