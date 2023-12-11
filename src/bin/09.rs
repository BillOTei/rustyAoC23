advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let histories = input
        .split("\n")
        .map(|h| h.split(" ").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let mut r = 0i32;
    for history in histories {
        let mut lasts: Vec<i32> = vec![];
        lasts.push(history.last().unwrap().clone());

        r += reduce(history, &mut lasts).iter().sum::<i32>()
    }

    Some(r)
}

fn vec_diff(input: Vec<i32>) -> Vec<i32> {
    let vals = input.iter();
    let next_vals = input.iter().skip(1);

    vals.zip(next_vals).map(|(cur, next)| next - cur).collect()
}

fn reduce(l: Vec<i32>, prev: &mut Vec<i32>) -> &mut Vec<i32> {
    if l.last().unwrap() == &0i32 && l.iter().sum::<i32>() == 0i32 {
        return prev;
    }
    let sub = vec_diff(l);
    let last = sub.last().unwrap().clone();
    prev.push(last);

    reduce(sub, prev)
}

fn reduce_left(l: Vec<i32>, prev: &mut Vec<i32>) -> &mut Vec<i32> {
    if l.last().unwrap() == &0i32 && l.iter().sum::<i32>() == 0i32 {
        return prev;
    }
    let sub = vec_diff(l);
    let first = sub.first().unwrap().clone();
    prev.push(first);

    reduce_left(sub, prev)
}

pub fn part_two(input: &str) -> Option<i32> {
    let histories = input
        .split("\n")
        .map(|h| h.split(" ").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let mut result = 0i32;
    for history in histories {
        let mut firsts: Vec<i32> = vec![];
        firsts.push(history.first().unwrap().clone());
        reduce_left(history, &mut firsts);
        let len = firsts.len();
        let step = firsts[len - 2];
        let mut r = step;
        firsts.reverse();
        for (i, v) in firsts.iter().enumerate() {
            if i >= 2 {
                r = v - r
            }
        }
        result += r;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
