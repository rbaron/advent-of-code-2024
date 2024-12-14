use std::collections::{HashSet, VecDeque};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn add(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Blk {
    da: Pos,
    db: Pos,
    prize: Pos,
}

fn parse_block(block: &str) -> Blk {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X\=(\d+), Y=(\d+)",
    )
    .unwrap();
    let cap: Vec<i64> = re
        .captures(block)
        .unwrap()
        .iter()
        .skip(1)
        .map(|v| v.unwrap().as_str().parse::<i64>().unwrap())
        .collect();
    match cap.as_slice() {
        [a, b, c, d, e, f] => Blk {
            da: Pos { x: *a, y: *b },
            db: Pos { x: *c, y: *d },
            prize: Pos { x: *e, y: *f },
        },
        _ => panic!("Invalid capture: {:?}", cap),
    }
}

fn min_cost(blk: &Blk) -> Option<i32> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((Pos { x: 0, y: 0 }, 0));
    let mut min = Option::None;
    while let Some((pos, cost)) = queue.pop_front() {
        if seen.contains(&(pos, cost)) || (min.is_some() && cost >= min.unwrap()) {
            continue;
        }
        seen.insert((pos, cost));
        if pos.y > blk.prize.y || pos.x > blk.prize.x {
            continue;
        }
        if pos == blk.prize {
            if min.is_none() || cost < min.unwrap() {
                min = Some(cost);
            }
        }
        queue.push_back((pos.add(&blk.da), cost + 3));
        queue.push_back((pos.add(&blk.db), cost + 1));
    }
    min
}

const ADD: i64 = 10000000000000;

fn solve(blk: &Blk) -> Option<i64> {
    let (ax, ay) = (blk.da.x, blk.da.y);
    let (bx, by) = (blk.db.x, blk.db.y);
    let (px, py) = (ADD + blk.prize.x, ADD + blk.prize.y);

    let a_nom = bx * py - (by * px);
    let a_den = bx * ay - (ax * by);
    if a_nom % a_den != 0 {
        return None;
    }
    let a = a_nom / a_den;

    let b_den = bx;
    let b_nom = px - ax * a;
    if b_nom % b_den != 0 {
        return None;
    }
    let b = b_nom / b_den;

    Some(a * 3 + b)
}

fn main() {
    let blocks = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .split("\n\n")
        .map(parse_block)
        .collect::<Vec<_>>();

    let cost1: i32 = blocks.iter().map(min_cost).map(|v| v.unwrap_or(0)).sum();
    println!("{}", cost1);

    let cost2: i64 = blocks.iter().map(solve).map(|v| v.unwrap_or(0)).sum();
    println!("{}", cost2);
}
