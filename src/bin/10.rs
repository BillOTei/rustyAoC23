use std::collections::HashSet;
use array2d::Array2D;
use itertools::Itertools;
use queues::*;

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
    let mut curr_x = starting_pipes[0].0;
    let mut curr_y = starting_pipes[0].1;
    let mut curr_char = starting_pipes[0].2.clone();
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

fn get_map(input: &str) -> Array2D<char> {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut array = Vec::new();
    for row in rows {
        let row_vec: Vec<char> = row.chars().collect();
        array.push(row_vec);
    }

    Array2D::from_rows(&array).unwrap()
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = get_map(input);
    let mut visited: HashSet<(usize, usize, char)> = HashSet::new();
    let mut prev = (101usize, 96usize); // Change that according to your input
    let starting_pipes = get_surrounding_pipes_start(prev.0, prev.1, &map); // it's a loop ! len == 2
    let mut curr_x = starting_pipes[0].0;
    let mut curr_y = starting_pipes[0].1;
    let mut curr_char = starting_pipes[0].2.clone();
    visited.insert((prev.0, prev.1, 'S'));
    visited.insert((curr_x, curr_y, curr_char));
    while curr_char != 'S' {
        let next = handle_pipe(curr_char.clone(), (curr_x, curr_y), prev);
        match map.get(next.1, next.0).unwrap() {
            &'.' => panic!("wrong stuff happening"),
            &c => {
                curr_char = c;
                if c != 'S' {
                    visited.insert((next.0, next.1, c));
                }
                prev = (curr_x, curr_y);
                curr_x = next.0;
                curr_y = next.1;
            }
        }
    }
    let max_x = map.num_columns() - 1;
    let mut r = 0;
    for (y, l) in map.as_rows().iter().enumerate() {
        for (x, p) in l.iter().enumerate() {
            let mut vert_pipe_count = 0;
            if !visited.contains(&(x, y, *p)) {
                let mut last_pipe = ' ';
                let slice = &l[x + 1..=max_x];
                for (x_right, right_p) in slice.iter().enumerate() {
                    if visited.contains(&(x_right, y, *right_p)) {
                        match right_p {
                            '|' => vert_pipe_count += 1,
                            'F' => last_pipe = 'F',
                            '7' => {
                                if last_pipe == 'F' {
                                    vert_pipe_count += 2;
                                } else if last_pipe == 'L' {
                                    vert_pipe_count += 1;
                                }
                                last_pipe = ' ';
                            },
                            'L' => last_pipe = 'L',
                            'J' => {
                                if last_pipe == 'F' {
                                    vert_pipe_count += 1;
                                } else if last_pipe == 'L' {
                                    vert_pipe_count += 2;
                                }
                                last_pipe = ' ';
                            },
                            _ => ()
                        }
                    }
                }
                if vert_pipe_count > 0 && vert_pipe_count % 2 == 0 {
                    r += 1;
                }
                println!("count {:?} x {:?} y {:?} p {:?}", vert_pipe_count, x, y, p)
            }
        }
    }

    Some(part_2(input))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    // | is a vertical pipe connecting north and south.
    NorthSouth,
    // - is a horizontal pipe connecting east and west.
    EastWest,
    // L is a 90-degree bend connecting north and east.
    NorthEast,
    // J is a 90-degree bend connecting north and west.
    NorthWest,
    // 7 is a 90-degree bend connecting south and west.
    SouthWest,
    // F is a 90-degree bend connecting south and east.
    SouthEast,
    // . is ground; there is no pipe in this tile.
    Ground,
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    Start,
}
use Tile::*;
impl Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => NorthSouth,
            '-' => EastWest,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            'S' => Start,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    row_idx: usize,
    col_idx: usize,
}

impl Coord {
    fn new(row_idx: usize, col_idx: usize) -> Self {
        Self { row_idx, col_idx }
    }

    fn valid_neighbours(&self, map: &[Vec<Tile>]) -> Vec<Coord> {
        let mut neighbours = vec![];
        let max_height = map.len() - 1;
        let max_width = map[0].len() - 1;

        match map[self.row_idx][self.col_idx] {
            Ground => (),
            Start => {
                // north
                if self.row_idx > 0 {
                    let tile = map[self.row_idx - 1][self.col_idx];
                    if matches!(tile, NorthSouth | SouthWest | SouthEast) {
                        neighbours.push(Coord::new(self.row_idx - 1, self.col_idx));
                    }
                }
                // south
                if self.row_idx < max_height {
                    let tile = map[self.row_idx + 1][self.col_idx];
                    if matches!(tile, NorthSouth | NorthWest | NorthEast) {
                        neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                    }
                }
                // west
                if self.col_idx > 0 {
                    let tile = map[self.row_idx][self.col_idx - 1];
                    if matches!(tile, EastWest | SouthEast | NorthEast) {
                        neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                    }
                }
                // east
                if self.col_idx < max_width {
                    let tile = map[self.row_idx][self.col_idx + 1];
                    if matches!(tile, EastWest | NorthWest | SouthWest) {
                        neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                    }
                }
            }
            NorthSouth => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // south
                if self.row_idx < max_height && map[self.row_idx + 1][self.col_idx] != Ground {
                    match map[self.row_idx + 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
            }
            EastWest => {
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
            NorthEast => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
            NorthWest => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
            }
            SouthWest => {
                // south
                if self.row_idx < max_height {
                    match map[self.row_idx + 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
            }
            SouthEast => {
                // south
                if self.row_idx < max_height {
                    match map[self.row_idx + 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
        }

        neighbours
    }
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Coord) {
    let mut start = Coord::new(0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    let tile = Tile::from(c);
                    if tile == Start {
                        start = Coord::new(row_idx, col_idx)
                    }
                    tile
                })
                .collect()
        })
        .collect();
    (map, start)
}

fn build_loop(start: Coord, map: &[Vec<Tile>]) -> HashSet<Coord> {
    let mut loop_coords = HashSet::new();
    loop_coords.insert(start);
    let mut to_visit = start.valid_neighbours(map);

    while let Some(curr_pos) = to_visit.pop() {
        for neighbour in curr_pos.valid_neighbours(map) {
            if !loop_coords.contains(&neighbour) {
                to_visit.push(neighbour);
                loop_coords.insert(neighbour);
            }
        }
    }

    loop_coords
}

pub fn part_1(input: &str) -> usize {
    let (map, start) = parse(input);
    let loop_coords = build_loop(start, &map);
    loop_coords.len() / 2
}

fn get_start_pipe(map: &Vec<Vec<Tile>>, start: Coord) -> Tile {
    let neighbours = start.valid_neighbours(map);
    let north = neighbours
        .iter()
        .find(|coord| coord.row_idx < start.row_idx)
        .is_some();
    let south = neighbours
        .iter()
        .find(|coord| coord.row_idx > start.row_idx)
        .is_some();
    let west = neighbours
        .iter()
        .find(|coord| coord.col_idx < start.col_idx)
        .is_some();
    let east = neighbours
        .iter()
        .find(|coord| coord.col_idx > start.col_idx)
        .is_some();

    match (north, west, south, east) {
        (true, true, _, _) => NorthWest,
        (true, _, true, _) => NorthSouth,
        (true, _, _, true) => NorthEast,
        (_, true, true, _) => SouthWest,
        (_, _, true, true) => SouthEast,
        (_, true, _, true) => EastWest,
        _ => panic!("No valid tile to replace Start with was found"),
    }
}

/// replace start with a valid pipe segment, and only keep pipe segments that are part of the loop
fn clean_map(start: Coord, loop_coords: &HashSet<Coord>, map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let start_pipe = get_start_pipe(&map, start);

    map.into_iter()
        .enumerate()
        .map(|(row_idx, line)| {
            line.into_iter()
                .enumerate()
                .map(|(col_idx, tile)| match tile {
                    Start => start_pipe,
                    pipe if loop_coords.contains(&Coord::new(row_idx, col_idx)) => pipe,
                    _ => Ground,
                })
                .collect()
        })
        .collect()
}

pub fn part_2(input: &str) -> usize {
    let (map, start) = parse(input);
    let loop_coords = build_loop(start, &map);
    let map = clean_map(start, &loop_coords, map);
    // scan from top to bottom and left to right, counting how many tiles are inside the loop.
    // keep track of a boolean that tells me if I'm inside the loop
    // every time I cross a vertical pipe that does not horizontally block the top (the place where I am in the loop), flip that state
    let mut inside = false;
    map.into_iter()
        .flatten()
        .filter(|tile| match tile {
            Ground => inside,
            NorthSouth | NorthWest | NorthEast => {
                inside = !inside;
                false
            }
            _ => false,
        })
        .count()
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
