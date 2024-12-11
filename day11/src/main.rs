use std::collections::HashMap;

fn count_len(stone: i64, times: i64, cache: &mut HashMap<(i64, i64), usize>) -> usize {
    if times == 0 {
        return 1;
    }

    if let Some(&cached) = cache.get(&(stone, times)) {
        return cached;
    }

    let res = match (stone, stone.to_string().len() % 2) {
        (0, _) => count_len(1, times - 1, cache),
        (_, 0) => {
            let s = stone.to_string();
            count_len(s[0..s.len() / 2].parse::<i64>().unwrap(), times - 1, cache)
                + count_len(s[s.len() / 2..].parse::<i64>().unwrap(), times - 1, cache)
        }
        _ => count_len(stone * 2024, times - 1, cache),
    };
    cache.insert((stone, times), res);
    res
}

fn run(stones: &Vec<i64>, times: i64) -> usize {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|stone| count_len(*stone, times, &mut cache))
        .sum()
}

fn main() {
    let stones = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("{}", run(&stones, 25));
    println!("{}", run(&stones, 75));
}
