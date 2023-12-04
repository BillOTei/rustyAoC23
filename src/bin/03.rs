use std::collections::HashMap;

use array2d::Array2D;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let array_2d = get_map(input);
    let mut r = 0;
    for (y, row_iter) in array_2d.rows_iter().enumerate() {
        let mut checked = false;
        for (x, element) in row_iter.enumerate() {
            let d: char = element.clone();
            if d.is_digit(10) && !checked {
                if !get_neighbors(x, y, &array_2d).is_empty() {
                    checked = true;

                    r += get_number(x, y, &array_2d)
                }
            } else if !d.is_digit(10) {
                checked = false;
            }
        }
    }

    Some(r)
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

fn get_number(x: usize, y: usize, array2d: &Array2D<char>) -> u32 {
    // We're dealing with only 3 digits numbers
    let mut i = x.clone();
    let mut j = x.clone() - 1;
    let mut next_neighbors = Vec::new();
    let mut prev_neighbors = Vec::new();
    while array2d.get(y, i).is_some() && array2d.get(y, i).unwrap().is_digit(10) {
        next_neighbors.push(array2d.get(y, i).unwrap());
        i += 1
    }
    while array2d.get(y, j).is_some() && array2d.get(y, j).unwrap().is_digit(10) {
        prev_neighbors.push(array2d.get(y, j).unwrap());
        if j > 0 {
            j -= 1
        } else {
            break;
        }
    }

    let mut prev: Vec<&char> = prev_neighbors.clone().into_iter().rev().collect();
    prev.append(&mut next_neighbors);

    prev
        .into_iter()
        .fold(String::new(), |a, b| a + &b.to_string())
        .parse()
        .unwrap()
}

fn get_neighbors(x: usize, y: usize, array2d: &Array2D<char>) -> Vec<&char> {
    let mut neighbors = Vec::new();
    neighbors.push(array2d.get(y, x.checked_add(1).unwrap()));
    if x > 0 {
        neighbors.push(array2d.get(y, x.checked_sub(1).unwrap()));
        neighbors.push(array2d.get(y.checked_add(1).unwrap(), x.checked_sub(1).unwrap()));
    }
    if y > 0 {
        neighbors.push(array2d.get(y.checked_sub(1).unwrap(), x));
        neighbors.push(array2d.get(y.checked_sub(1).unwrap(), x.checked_add(1).unwrap()));
    }
    if x > 0 && y > 0 {
        neighbors.push(array2d.get(y.checked_sub(1).unwrap(), x.checked_sub(1).unwrap()));
    }
    neighbors.push(array2d.get(y.checked_add(1).unwrap(), x));
    neighbors.push(array2d.get(y.checked_add(1).unwrap(), x.checked_add(1).unwrap()));

    neighbors
        .into_iter()
        .flatten()
        .filter(|c| !c.is_digit(10) && **c != '.')
        .collect()
}

fn get_neighboring_star(x: usize, y: usize, array2d: &Array2D<char>) -> Vec<(usize, usize, Option<&char>)> {
    let mut neighbors: Vec<(usize, usize, Option<&char>)> = Vec::new();
    neighbors.push((x.checked_add(1).unwrap(), y, array2d.get(y, x.checked_add(1).unwrap())));
    if x > 0 {
        neighbors.push((x.checked_sub(1).unwrap(), y, array2d.get(y, x.checked_sub(1).unwrap())));
        neighbors.push((x.checked_sub(1).unwrap(), y.checked_add(1).unwrap(), array2d.get(y.checked_add(1).unwrap(), x.checked_sub(1).unwrap())));
    }
    if y > 0 {
        neighbors.push((x, y.checked_sub(1).unwrap(), array2d.get(y.checked_sub(1).unwrap(), x)));
        neighbors.push((x.checked_add(1).unwrap(), y.checked_sub(1).unwrap(), array2d.get(y.checked_sub(1).unwrap(), x.checked_add(1).unwrap())));
    }
    if x > 0 && y > 0 {
        neighbors.push((x.checked_sub(1).unwrap(), y.checked_sub(1).unwrap(), array2d.get(y.checked_sub(1).unwrap(), x.checked_sub(1).unwrap())));
    }
    neighbors.push((x, y.checked_add(1).unwrap(), array2d.get(y.checked_add(1).unwrap(), x)));
    neighbors.push((x.checked_add(1).unwrap(), y.checked_add(1).unwrap(), array2d.get(y.checked_add(1).unwrap(), x.checked_add(1).unwrap())));

    neighbors
        .into_iter()
        .filter(|(_, _, c)| c.is_some() && c.unwrap() == &'*')
        .collect::<Vec<(usize, usize, Option<&char>)>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    let array_2d = get_map(input);
    // let mut r = Vec::new();
    let mut stars_count: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (y, row_iter) in array_2d.rows_iter().enumerate() {
        let mut checked = false;
        for (x, element) in row_iter.enumerate() {
            let d: char = element.clone();
            if d.is_digit(10) && !checked {
                let star_vec = get_neighboring_star(x, y, &array_2d);
                if !star_vec.is_empty() {
                    let (x_star, y_star, _) = star_vec.first().unwrap().clone();
                    let key = (x_star, y_star);
                    let gear = get_number(x, y, &array_2d);
                    if stars_count.contains_key(&key) {
                        let mut v: Vec<u32> = stars_count.get(&(x_star, y_star)).unwrap().clone();
                        v.push(gear);
                        stars_count.insert(key, v);
                    } else {
                        stars_count.insert(key, vec![gear]);
                    }
                    checked = true
                }
            } else if !d.is_digit(10) {
                checked = false;
            }
        }
    }
    let r = stars_count.iter().fold(0u32, |acc, (_, gears)| {
        if gears.len() == 2 {
            acc + gears.first().unwrap() * gears.last().unwrap()
        } else {
            acc
        }
    });

    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
