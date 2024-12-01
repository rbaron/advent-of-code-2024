use std::collections::{BinaryHeap, HashMap};

fn main() {
    let contents =
        std::fs::read_to_string(std::env::args().nth(1).expect("Missing input filename")).unwrap();

    let mut col1 = BinaryHeap::<i32>::new();
    let mut col2 = BinaryHeap::<i32>::new();

    contents.lines().for_each(|line| {
        let mut iter = line.split("   ");
        col1.push(iter.next().unwrap().parse::<i32>().unwrap());
        col2.push(iter.next().unwrap().parse::<i32>().unwrap());
    });

    let col1 = col1.into_sorted_vec();
    let col2 = col2.into_sorted_vec();

    let sum_dists = std::iter::zip(col1.iter(), col2.iter())
        .map(|(i, j)| (i - j).abs())
        .sum::<i32>();

    println!("{}", sum_dists);

    let counts_by_digit = col2.iter().fold(HashMap::new(), |mut counts, i| {
        *counts.entry(i).or_insert(0) += 1;
        counts
    });

    let sim_score = col1
        .iter()
        .map(|&i| match counts_by_digit.get(&i) {
            Some(v) => i * *v,
            None => 0,
        })
        .sum::<i32>();

    println!("{}", sim_score);
}
