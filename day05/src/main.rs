use std::collections::HashSet;

fn parse_input(input: &str) -> (HashSet<(&str, &str)>, Vec<Vec<&str>>) {
    let mut it = input.split("\n\n");

    let rules: Vec<(&str, &str)> = it
        .next()
        .unwrap()
        .split("\n")
        .map(|l| {
            let p = l.split("|").collect::<Vec<&str>>();
            (p[0].trim(), p[1].trim())
        })
        .collect();

    // let mut graph: Graph = HashMap::new();
    let mut orders: HashSet<(&str, &str)> = HashSet::new();
    for (l, r) in rules.iter() {
        orders.insert((l, r));
    }

    let updates: Vec<Vec<&str>> = it
        .next()
        .unwrap()
        .split("\n")
        .map(|l| l.split(",").collect())
        .collect();

    (orders, updates)
}

fn check_update(update: &Vec<&str>, orders: &HashSet<(&str, &str)>) -> bool {
    for i in 0..update.len() - 1 {
        let l = update[i];
        let r = update[i + 1];
        if !orders.contains(&(l, r)) {
            return false;
        }
    }
    true
}

fn fix_order<'a>(update: &'a Vec<&str>, orders: &HashSet<(&str, &str)>) -> Vec<&'a str> {
    let mut update = update.clone();
    update.sort_by(|a, b| {
        if orders.contains(&(a, b)) {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }
    });
    update
}

fn main() {
    let contents = std::fs::read_to_string(std::env::args().nth(1).expect("Missing input file"))
        .expect("Error");

    let (orders, updates) = parse_input(&contents);

    let mut sum = 0;
    let mut sum2 = 0;
    for u in &updates {
        if check_update(u, &orders) {
            let mid = u[u.len() / 2];
            sum += mid.parse::<i32>().unwrap();
        } else {
            let fixed = fix_order(u, &orders);
            let mid = fixed[fixed.len() / 2];
            sum2 += mid.parse::<i32>().unwrap();
        }
    }

    println!("{}", sum);
    println!("{}", sum2);
}
