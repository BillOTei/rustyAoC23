advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let seeds_and_maps = input.split("\n\n");
    let binding = seeds_and_maps.collect::<Vec<&str>>();

    let seeds_to_soil_map = get_map(1, "seed-to-soil map:", &binding);
    let soil_to_fertilizer_map = get_map(2, "soil-to-fertilizer map:", &binding);
    let fertilizer_to_water_map = get_map(3, "fertilizer-to-water map:", &binding);
    let water_to_light_map = get_map(4, "water-to-light map:", &binding);
    let light_to_temperature_map = get_map(5, "light-to-temperature map:", &binding);
    let temperature_to_humidity_map = get_map(6, "temperature-to-humidity map:", &binding);
    let humidity_to_location_map = get_map(7, "humidity-to-location map:", &binding);
    let seeds = get_seeds(&binding);

    let mut res = Vec::new();
    for seed in seeds {
        let mut tmp = seed.clone();
        tmp = scan_map(tmp, &seeds_to_soil_map);
        tmp = scan_map(tmp, &soil_to_fertilizer_map);
        tmp = scan_map(tmp, &fertilizer_to_water_map);
        tmp = scan_map(tmp, &water_to_light_map);
        tmp = scan_map(tmp, &light_to_temperature_map);
        tmp = scan_map(tmp, &temperature_to_humidity_map);
        tmp = scan_map(tmp, &humidity_to_location_map);
        res.push(tmp);
    }

    Some(*res.iter().min().unwrap())
}

fn scan_map(v: u64, map: &Vec<Vec<u64>>) -> u64 {
    for m in map {
        match translate(v, m) {
            Some(r) => {
                return r;
            }
            None => {}
        }
    }

    v
}

fn translate(x: u64, m: &Vec<u64>) -> Option<u64> {
    let range = m[2];
    let source_range = m[1];
    let dest_range = m[0];
    if x >= source_range && x < source_range + range {
        return Some(dest_range + (x - source_range));
    }

    None
}

fn get_map(idx: usize, label: &str, input: &Vec<&str>) -> Vec<Vec<u64>> {
    input[idx]
        .split("\n").filter(|v| *v != label)
        .map(|l| l.split(" ").map(|v| v.parse::<u64>().unwrap()).collect::<Vec<u64>>())
        .collect::<Vec<Vec<u64>>>()
}

fn get_seeds(input: &Vec<&str>) -> Vec<u64> {
    let seeds_str = input.first().unwrap().split("seeds: ").filter(|v| *v != "").collect::<Vec<&str>>();
    let binding = seeds_str.first().unwrap().split(" ").collect::<Vec<&str>>();

    binding.iter().map(|s| s.parse::<u64>().unwrap()).collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let seeds_and_maps = input.split("\n\n");
    let binding = seeds_and_maps.collect::<Vec<&str>>();

    let seeds_to_soil_map = get_map(1, "seed-to-soil map:", &binding);
    let soil_to_fertilizer_map = get_map(2, "soil-to-fertilizer map:", &binding);
    let fertilizer_to_water_map = get_map(3, "fertilizer-to-water map:", &binding);
    let water_to_light_map = get_map(4, "water-to-light map:", &binding);
    let light_to_temperature_map = get_map(5, "light-to-temperature map:", &binding);
    let temperature_to_humidity_map = get_map(6, "temperature-to-humidity map:", &binding);
    let humidity_to_location_map = get_map(7, "humidity-to-location map:", &binding);
    let seeds = get_seeds(&binding);

    let mut res = 0;
    for (i, seed_or_range) in seeds.iter().enumerate() {
        if i % 2 == 0 {
            let last = seed_or_range + seeds[i + 1];
            for seed in *seed_or_range as usize..last as usize {
                let mut tmp = seed.clone() as u64;
                tmp = scan_map(tmp, &seeds_to_soil_map);
                tmp = scan_map(tmp, &soil_to_fertilizer_map);
                tmp = scan_map(tmp, &fertilizer_to_water_map);
                tmp = scan_map(tmp, &water_to_light_map);
                tmp = scan_map(tmp, &light_to_temperature_map);
                tmp = scan_map(tmp, &temperature_to_humidity_map);
                tmp = scan_map(tmp, &humidity_to_location_map);
                if res == 0 || tmp < res {
                    res = tmp
                }
            }
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
