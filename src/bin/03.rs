use array2d::Array2D;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut array = Vec::new();
    for row in rows {
        let row_vec: Vec<char> = row.chars().collect();
        array.push(row_vec);
    }
    let array_2d = Array2D::from_rows(&array).unwrap();
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
            } else {
                checked = false;
            }
        }
    }

    Some(r)
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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
