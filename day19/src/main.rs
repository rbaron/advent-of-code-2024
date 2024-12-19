use std::collections::HashMap;

fn count_combinations<'a>(
    pattern: &'a str,
    prefix: &'a str,
    towels: &'a [&'a str],
    cache: &mut HashMap<(&'a str, &'a str), usize>,
) -> usize {
    let key = (prefix, pattern);
    if let Some(&v) = cache.get(&key) {
        return v;
    }

    if !pattern.starts_with(prefix) {
        return 0;
    }
    let remainder = &pattern[prefix.len()..];
    if remainder.is_empty() {
        return 1;
    }
    let res = towels
        .iter()
        .map(|towel| count_combinations(remainder, towel, towels, cache))
        .sum();
    cache.insert(key, res);
    res
}

fn main() {
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let (towels, patterns) = match input.split("\n\n").collect::<Vec<&str>>()[..] {
        [towels, patterns] => (
            towels.split(", ").collect::<Vec<&str>>(),
            patterns.split('\n').collect::<Vec<&str>>(),
        ),
        _ => panic!("Invalid input"),
    };

    let mut cache: HashMap<(&str, &str), usize> = HashMap::new();
    let counts = patterns
        .iter()
        .filter(|pattern| count_combinations(pattern, "", &towels, &mut cache) > 0)
        .count();
    println!("{}", counts);

    let counts: usize = patterns
        .iter()
        .map(|pattern| count_combinations(pattern, "", &towels, &mut cache))
        .sum();
    println!("{}", counts);
}
