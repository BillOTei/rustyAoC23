use advent_of_code::get_map;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut raw_rows = input.split("\n").collect::<Vec<&str>>();
    let map = get_map(input);

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
    for y in rows_to_expand {
        let mut prev = expanded_raw_rows[0..y + offset].to_vec();
        let next = expanded_raw_rows[y + offset..expanded_raw_rows.len()].to_vec();
        prev.push(new_row.as_str());
        expanded_raw_rows = vec![prev, next].concat();
        offset += 1;
    }
    // Columns expansion
    let mut expanded_raw_columns = expanded_raw_rows.clone().iter().map(|&c| c.to_string()).collect::<Vec<String>>();
    for mut row in &mut expanded_raw_columns {
        for x in cols_to_expand.clone() {
            row.insert(x, '.');
        }
    }


    println!("{:?}", expanded_raw_columns);

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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
