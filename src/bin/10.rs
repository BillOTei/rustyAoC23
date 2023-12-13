use array2d::Array2D;

advent_of_code::solution!(10);

fn handle_pipe(c: char, coord: (usize, usize), prev: (usize, usize)) -> (usize, usize) {
    match c {
        '|' => {
            if coord.0 != prev.0 || coord.1 == prev.1 {
                panic!("wrong coordinates {:?} {:?} {:?}", coord, prev, c)
            } else if coord.1 > prev.1 {
                (coord.0, coord.1 + 1)
            } else if coord.1 > 0 {
                (coord.0, coord.1 - 1)
            } else {
                panic!("wrong coordinates {:?} {:?} {:?}", coord, prev, c)
            }
        }
        '-' => {
            if coord.1 != prev.1 || coord.0 == prev.0 {
                panic!("wrong coordinates {:?} {:?} {:?}", coord, prev, c)
            } else if coord.0 > prev.0 {
                (coord.0 + 1, coord.1)
            } else if coord.0 > 0 {
                (coord.0 - 1, coord.1)
            } else {
                panic!("wrong coordinates {:?} {:?} {:?}", coord, prev, c)
            }
        }
        'L' => {
            if coord.0 == prev.0 && coord.1 > prev.1 {
                (coord.0 + 1, coord.1)
            } else if prev.0 > 0 && coord.1 > 0 && coord.0 == prev.0 - 1 && coord.1 == prev.1 {
                (coord.0, coord.1 - 1)
            } else {
                panic!("wrong coordinates {:?} {:?} {:?}", coord, prev, c)
            }
        }
        'J' => {
            if coord.0 > 0 && coord.0 == prev.0 && coord.1 > prev.1 {
                (coord.0 - 1, coord.1)
            } else if coord.1 == prev.1 && coord.1 > 0 && coord.0 > prev.0 {
                (coord.0, coord.1 - 1)
            } else {
                panic!("wrong coordinates {:?} {:?} {:?}", coord, prev, c)
            }
        }
        '7' => {
            if coord.0 > 0 && coord.0 == prev.0 && coord.1 < prev.1 {
                (coord.0 - 1, coord.1)
            } else if coord.1 == prev.1 && coord.0 > prev.0 {
                (coord.0, coord.1 + 1)
            } else {
                panic!("wrong coordinates {:?} {:?} {:?}", coord, prev, c)
            }
        }
        'F' => {
            if coord.0 == prev.0 && coord.1 < prev.1 {
                (coord.0 + 1, coord.1)
            } else if coord.0 < prev.0 && coord.1 == prev.1 {
                (coord.0, coord.1 + 1)
            } else {
                panic!("wrong coordinates {:?} {:?} {:?}", coord, prev, c)
            }
        }
        _ => panic!("wrong coordinates {:?} {:?} {:?}", coord, prev, c)
        ,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = get_map(input);
    let mut visited = vec![];
    let mut prev = (101usize, 96usize); // Change that according to your input
    let starting_pipes = get_surrounding_pipes_start(prev.0, prev.1, &map); // it's a loop ! len == 2
    let mut curr_x= starting_pipes[0].0;
    let mut curr_y= starting_pipes[0].1;;
    let mut curr_char= starting_pipes[0].2.clone();
    visited.push((curr_x, curr_y, curr_char));

    while curr_char != 'S' {
        let next = handle_pipe(curr_char.clone(), (curr_x, curr_y), prev);
        match map.get(next.1, next.0).unwrap() {
            &'.' => panic!("wrong stuff happening"),
            &c => {
                curr_char = c;
                visited.push((next.0, next.1, c));
                prev = (curr_x, curr_y);
                curr_x = next.0;
                curr_y = next.1;
            }
        }
    }

    Some(visited.len() / 2)
}

fn get_surrounding_pipes_start(x: usize, y: usize, map: &Array2D<char>) -> Vec<(usize, usize, &char)> {
    let mut r = vec![];
    let up = y.checked_sub(1).map(|c| map.get(c, x)).flatten();
    let can_connect_up = vec!['S', '|', '7', 'F'];

    let down = map.get(y + 1, x);
    let can_connect_down = vec!['S', '|', 'J', 'L'];

    let left = x.checked_sub(1).map(|c| map.get(y, c)).flatten();
    let can_connect_left = vec!['S', '-', 'F', 'L'];

    let right = map.get(y, x + 1);
    let can_connect_right = vec!['S', '-', 'J', '7'];

    if up.is_some_and(|c| can_connect_up.contains(c)) {
        r.push((x, y - 1, up.unwrap()))
    }
    if down.is_some_and(|c| can_connect_down.contains(c)) {
        r.push((x, y + 1, down.unwrap()))
    }
    if left.is_some_and(|c| can_connect_left.contains(c)) {
        r.push((x - 1, y, left.unwrap()))
    }
    if right.is_some_and(|c| can_connect_right.contains(c)) {
        r.push((x + 1, y, right.unwrap()))
    }

    r
}

fn get_surrounding_pipes_horiz(x: usize, y: usize, map: &Array2D<char>) -> Vec<(usize, usize, &char)> {
    let mut r = vec![];

    let left = x.checked_sub(1).map(|c| map.get(y, c)).flatten();
    let can_connect_left = vec!['S', '-', 'F', 'L'];

    let right = map.get(y, x + 1);
    let can_connect_right = vec!['S', '-', 'J', '7'];

    if left.is_some_and(|c| can_connect_left.contains(c)) {
        r.push((x - 1, y, left.unwrap()))
    }
    if right.is_some_and(|c| can_connect_right.contains(c)) {
        r.push((x + 1, y, right.unwrap()))
    }

    r
}

fn get_surrounding_pipes_vert(x: usize, y: usize, map: &Array2D<char>) -> Vec<(usize, usize, &char)> {
    let mut r = vec![];
    let up = y.checked_sub(1).map(|c| map.get(c, x)).flatten();
    let can_connect_up = vec!['S', '|', '7', 'F'];

    let down = map.get(y + 1, x);
    let can_connect_down = vec!['S', '|', 'J', 'L'];

    if up.is_some_and(|c| can_connect_up.contains(c)) {
        r.push((x, y - 1, up.unwrap()))
    }
    if down.is_some_and(|c| can_connect_down.contains(c)) {
        r.push((x, y + 1, down.unwrap()))
    }

    r
}

fn get_surrounding_pipes_7(x: usize, y: usize, map: &Array2D<char>) -> Vec<(usize, usize, &char)> {
    let mut r = vec![];

    let down = map.get(y + 1, x);
    let can_connect_down = vec!['S', '|', 'J', 'L'];

    let left = x.checked_sub(1).map(|c| map.get(y, c)).flatten();
    let can_connect_left = vec!['S', '-', 'F', 'L'];

    if down.is_some_and(|c| can_connect_down.contains(c)) {
        r.push((x, y + 1, down.unwrap()))
    }
    if left.is_some_and(|c| can_connect_left.contains(c)) {
        r.push((x - 1, y, left.unwrap()))
    }

    r
}

fn get_surrounding_pipes_l(x: usize, y: usize, map: &Array2D<char>) -> Vec<(usize, usize, &char)> {
    let mut r = vec![];
    let up = y.checked_sub(1).map(|c| map.get(c, x)).flatten();
    let can_connect_up = vec!['S', '|', '7', 'F'];

    let right = map.get(y, x + 1);
    let can_connect_right = vec!['S', '-', 'J', '7'];

    if up.is_some_and(|c| can_connect_up.contains(c)) {
        r.push((x, y - 1, up.unwrap()))
    }
    if right.is_some_and(|c| can_connect_right.contains(c)) {
        r.push((x + 1, y, right.unwrap()))
    }

    r
}

fn get_surrounding_pipes_j(x: usize, y: usize, map: &Array2D<char>) -> Vec<(usize, usize, &char)> {
    let mut r = vec![];
    let up = y.checked_sub(1).map(|c| map.get(c, x)).flatten();
    let can_connect_up = vec!['S', '|', '7', 'F'];

    let left = x.checked_sub(1).map(|c| map.get(y, c)).flatten();
    let can_connect_left = vec!['S', '-', 'F', 'L'];
    if up.is_some_and(|c| can_connect_up.contains(c)) {
        r.push((x, y - 1, up.unwrap()))
    }
    if left.is_some_and(|c| can_connect_left.contains(c)) {
        r.push((x - 1, y, left.unwrap()))
    }

    r
}

fn get_surrounding_pipes_f(x: usize, y: usize, map: &Array2D<char>) -> Vec<(usize, usize, &char)> {
    let mut r = vec![];

    let down = map.get(y + 1, x);
    let can_connect_down = vec!['S', '|', 'J', 'L'];

    let right = map.get(y, x + 1);
    let can_connect_right = vec!['S', '-', 'J', '7'];

    if down.is_some_and(|c| can_connect_down.contains(c)) {
        r.push((x, y + 1, down.unwrap()))
    }
    if right.is_some_and(|c| can_connect_right.contains(c)) {
        r.push((x + 1, y, right.unwrap()))
    }

    r
}

fn get_map(input: &str) -> Array2D<char> {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut array = Vec::new();
    for row in rows {
        let row_vec: Vec<char> = row.chars().collect();
        array.push(row_vec);
    }

    Array2D::from_rows(&array).unwrap()
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
