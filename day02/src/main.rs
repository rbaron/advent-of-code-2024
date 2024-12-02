use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_reports(filename: &str) -> Vec<Vec<i32>> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let diffs: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();
    diffs.iter().all(|&d| d >= 1 && d <= 3) || diffs.iter().all(|&d| d >= -3 && d <= -1)
}

fn is_safe2_rec(
    prev: Option<i32>,
    head: i32,
    tail: &[i32],
    edit_budget: i32,
    cmp: fn(i32, i32) -> bool,
) -> bool {
    // No more edits possible.
    if edit_budget < 0 {
        return false;
    }
    // If we've arrived at a bad state, return false.
    if let Some(prev) = prev {
        if !cmp(prev, head) {
            return false;
        }
    }
    // We've made it to the end.
    if tail.is_empty() {
        return true;
    }
    if cmp(head, tail[0]) {
        // All good so far. Move on to next element.
        is_safe2_rec(Some(head), tail[0], &tail[1..], edit_budget, cmp)
    } else {
        // Try two other options: erase either head or tail[0].
        is_safe2_rec(prev, tail[0], &tail[1..], edit_budget - 1, cmp)
            || is_safe2_rec(prev, head, &tail[1..], edit_budget - 1, cmp)
    }
}

fn is_safe2(report: &[i32], budget: i32) -> bool {
    match report.split_first() {
        Some((head, tail)) => {
            is_safe2_rec(None, *head, tail, budget, |a, b| b - a >= 1 && b - a <= 3)
                || is_safe2_rec(None, *head, tail, budget, |a, b| b - a >= -3 && b - a <= -1)
        }
        None => true,
    }
}

fn main() {
    let reports = parse_reports(&args().nth(1).unwrap());
    println!("{}", reports.iter().filter(|&r| is_safe(r)).count());
    println!("{}", reports.iter().filter(|&r| is_safe2(r, 1)).count());
}

#[cfg(test)]
mod test {
    use super::*;

    fn is_safe2_brute_force(reports: &[i32]) -> bool {
        if is_safe(reports) {
            println!("{:?} safe as is", reports);
            return true;
        }
        for i in 0..reports.len() {
            // Remove element i.
            let mut reports2 = reports.to_vec();
            reports2.remove(i);
            if is_safe(&reports2) {
                println!("{:?} safe after removing {}", reports, reports[i]);
                return true;
            }
        }
        false
    }

    #[test]
    fn test_is_safe2() {
        for report in parse_reports("input-test.txt") {
            assert_eq!(is_safe2(&report, 1), is_safe2_brute_force(&report));
        }
    }

    #[test]
    fn test_is_safe2_equals_is_safe_for_budget_0() {
        for report in parse_reports("input-test.txt") {
            assert_eq!(is_safe2(&report, 0), is_safe(&report));
        }
    }
}
