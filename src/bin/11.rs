use array2d::Array2D;
use itertools::Itertools;
use pathfinding::prelude::astar;

use advent_of_code::get_map;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let (expanded_map, _, _) = expand_map(input);
    let galaxies = get_galaxies(&expanded_map);

    // Get all combination paths
    let combinations = galaxies.iter().tuple_combinations().collect::<Vec<(&(usize, usize), &(usize, usize))>>();

    let mut r = 0;
    for pair in combinations {
        let (a, b) = pair;
        let shortest_path = astar(
            a,
            |p| get_successors(&expanded_map, p),
            |p| {
                let x: i32 = p.0 as i32;
                let y: i32 = p.1 as i32;
                ((x - b.0 as i32).abs() + (y - b.1 as i32).abs()) as u32
            },
            |p| *p == *b);

        r += shortest_path.unwrap().0.len() - 1
    }

    Some(r)
}

fn get_successors(array2d: &Array2D<char>, p: &(usize, usize)) -> Vec<((usize, usize), u32)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let (x, y) = *p;
    if array2d.get(y, x.checked_add(1).unwrap()).is_some() {
        neighbors.push((x.checked_add(1).unwrap(), y));
    }
    if x > 0 {
        if array2d.get(y, x.checked_sub(1).unwrap()).is_some() {
            neighbors.push((x.checked_sub(1).unwrap(), y));
        }
        // No diagonals
        // if array2d.get(y.checked_add(1).unwrap(), x.checked_sub(1).unwrap()).is_some() {
        //     neighbors.push((x.checked_sub(1).unwrap(), y.checked_add(1).unwrap()));
        // }
    }
    if y > 0 {
        if array2d.get(y.checked_sub(1).unwrap(), x).is_some() {
            neighbors.push((x, y.checked_sub(1).unwrap()));
        }
        // if array2d.get(y.checked_sub(1).unwrap(), x.checked_add(1).unwrap()).is_some() {
        //     neighbors.push((x.checked_add(1).unwrap(), y.checked_sub(1).unwrap()));
        // }
    }
    // if x > 0 && y > 0 {
    //     if array2d.get(y.checked_sub(1).unwrap(), x.checked_sub(1).unwrap()).is_some() {
    //         neighbors.push((x.checked_sub(1).unwrap(), y.checked_sub(1).unwrap()));
    //     }
    // }
    if array2d.get(y.checked_add(1).unwrap(), x).is_some() {
        neighbors.push((x, y.checked_add(1).unwrap()));
    }
    // if array2d.get(y.checked_add(1).unwrap(), x.checked_add(1).unwrap()).is_some() {
    //     neighbors.push((x.checked_add(1).unwrap(), y.checked_add(1).unwrap()));
    // }

    neighbors.iter().map(|&(x, y)| ((x, y), 1)).collect::<Vec<((usize, usize), u32)>>()
}

fn expand_map(input: &str) -> (Array2D<char>, Vec<usize>, Vec<usize>) {
    let raw_rows = input.split("\n").collect::<Vec<&str>>();
    let map = get_map(input);

    let expansion_factor = 50;

    let mut rows_to_expand = vec![];
    let mut cols_to_expand = vec![];
    for (y, row) in map.as_rows().iter().enumerate() {
        let r_iter = row.iter();
        if r_iter.filter(|&c| c == &'.').count() == row.len() {
            rows_to_expand.push(y);
        }
    }
    for (x, col) in map.as_columns().iter().enumerate() {
        let c_iter = col.iter();
        if c_iter.filter(|&c| c == &'.').count() == col.len() {
            cols_to_expand.push(x);
        }
    }

    // Rows expansion
    let x_len = map.row_len();
    let mut expanded_raw_rows = raw_rows.clone();
    let new_row = (0..x_len).map(|_| '.').collect::<String>();
    let mut offset = 0;
    for y in rows_to_expand.clone() {
        let mut prev = expanded_raw_rows[0..y + offset].to_vec();
        let next = expanded_raw_rows[y + offset..expanded_raw_rows.len()].to_vec();
        prev.push(new_row.as_str());
        expanded_raw_rows = vec![prev, next].concat();
        offset += 1;
    }
    // Columns expansion
    let mut expanded_raw_columns = expanded_raw_rows.clone().iter().map(|&c| c.to_string()).collect::<Vec<String>>();
    for row in &mut expanded_raw_columns {
        let mut offset = 0;
        for x in cols_to_expand.clone() {
            row.insert(x + offset, '.');
            offset += 1;
        }
    }

    let expanded_raw_map = expanded_raw_columns.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    (Array2D::from_rows(&expanded_raw_map).unwrap(), rows_to_expand, cols_to_expand)
}

fn get_galaxies(map: &Array2D<char>) -> Vec<(usize, usize)> {
    // Find galaxies
    let mut galaxies = vec![];
    for (y, row) in map.as_rows().iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if p == &'#' {
                galaxies.push((x, y))
            }
        }
    }

    galaxies
}

pub fn part_two(input: &str) -> Option<u64> {
    let (expanded_map, expanded_rows, expanded_cols) = expand_map(input);
    let mut galaxies = get_galaxies(&expanded_map);
    let expansion_factor = 5u64;

    // Get all combination paths
    let combinations = galaxies.iter().tuple_combinations().collect::<Vec<(&(usize, usize), &(usize, usize))>>();

    let mut r = 0u64;
    for pair in combinations {
        let (a, b) = pair;
        // Expansion factor calculation
        let mut include_expansion = 0u64;
        for y_expanded in expanded_rows.clone() {
            let y_to_compare = y_expanded + 1;
            if y_to_compare > a.1 && y_to_compare < b.1 || y_to_compare > b.1 && y_to_compare < a.1 {
                include_expansion += 1;
            }
        }
        for x_expanded in expanded_cols.clone() {
            let x_to_compare = x_expanded + 1;
            if x_to_compare > a.0 && x_to_compare < b.0 || x_to_compare > b.0 && x_to_compare < a.0 {
                include_expansion += 1;
            }
        }
        let final_expansion = if include_expansion == 0 {
            1
        } else {
            include_expansion * expansion_factor
        };

        println!("{:?} {}", pair, include_expansion);

        let shortest_path = astar(
            a,
            |p| get_successors(&expanded_map, p),
            |p| {
                let x: i32 = p.0 as i32;
                let y: i32 = p.1 as i32;
                ((x - b.0 as i32).abs() + (y - b.1 as i32).abs()) as u32
            },
            |p| *p == *b);

        r += (shortest_path.unwrap().0.len() - 1) as u64 * final_expansion
    }

    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
