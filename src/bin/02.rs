use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let game_rules: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
    let games = input.split("\n");
    let mut r = 0;
    for g in games {
        let game_by_id: Vec<&str> = g.split(": ").collect();
        let game_id = get_game_id(game_by_id.clone());
        let game_by_set: Vec<&str> = get_game_set(game_by_id);
        let mut check = true;
        for set in game_by_set {
            let cubes = get_game_cubes(set);
            for cube in cubes {
                let count: u32 = cube.first().unwrap().parse().unwrap();
                let color = cube.last().unwrap();
                if &count > game_rules.get(color).unwrap() {
                    check = false
                }
            }
        }
        if check {
            r += game_id
        }
    }

    Some(r)
}

fn get_game_id(g: Vec<&str>) -> u32 {
    let game_name_vec: Vec<&str> = g.first().unwrap().split(" ").collect();
    let game_id_vec: Vec<u32> = game_name_vec.last().iter().flat_map(|i| i.parse()).collect();

    game_id_vec.first().unwrap().clone()
}

fn get_game_set(g: Vec<&str>) -> Vec<&str> {
    g[1..]
        .to_vec()
        .first()
        .unwrap()
        .split("; ")
        .collect()
}

fn get_game_cubes(g: &str) -> Vec<Vec<&str>> {
    let cubes_str: Vec<&str> = g.split(", ").collect();

    cubes_str
        .into_iter()
        .map(|c| c.split(" ").collect::<Vec<&str>>())
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.split("\n");
    let mut r = 0;
    for g in games {
        let mut game_counts: HashMap<&str, u32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);
        let game_by_id: Vec<&str> = g.split(": ").collect();
        let game_by_set: Vec<&str> = get_game_set(game_by_id);
        for set in game_by_set {
            let cubes = get_game_cubes(set);
            for cube in cubes {
                let count: u32 = cube.first().unwrap().parse().unwrap();
                let color = cube.last().unwrap();
                if game_counts.get(color).unwrap() < &count {
                    game_counts.insert(color, count);
                }
            }
        }
        let power_list = game_counts
            .values()
            .cloned()
            .collect::<Vec<u32>>();
        let power = power_list.iter().fold(1u32, |mut acc, v| {
            acc *= v;

            acc
        });
        r += power;
    }

    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
