use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    y: i32,
    x: i32,
}

#[derive(Debug)]
struct Scan {
    h: i32,
    w: i32,
    pos_by_antenna: HashMap<char, Vec<Pos>>,
}

fn parse(filename: &str) -> Scan {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut scan = Scan {
        h: 0,
        w: 0,
        pos_by_antenna: HashMap::new(),
    };
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let pos = scan.pos_by_antenna.entry(c).or_insert(Vec::new());
                pos.push(Pos {
                    y: y as i32,
                    x: x as i32,
                });
            }
        }
    }
    scan.h = contents.lines().count() as i32;
    scan.w = contents.lines().next().unwrap().chars().count() as i32;
    scan
}

fn get_antinode(scan: &Scan, p1: &Pos, p2: &Pos) -> Option<Pos> {
    let v = Pos {
        y: p2.y - p1.y,
        x: p2.x - p1.x,
    };
    let p = Pos {
        y: p2.y + v.y,
        x: p2.x + v.x,
    };
    if p.y >= 0 && p.y < scan.h && p.x >= 0 && p.x < scan.w {
        Some(p)
    } else {
        None
    }
}

fn get_antinode2(scan: &Scan, p1: &Pos, p2: &Pos) -> Vec<Pos> {
    let v = Pos {
        y: p2.y - p1.y,
        x: p2.x - p1.x,
    };
    let mut res = Vec::new();
    let mut p = *p1;
    loop {
        p.y += v.y;
        p.x += v.x;
        if p.y < 0 || p.y >= scan.h || p.x < 0 || p.x >= scan.w {
            break;
        }
        res.push(p);
    }
    res
}

fn main() {
    let scan = parse(&std::env::args().nth(1).unwrap());
    let mut unique_pos = HashSet::new();
    for (_, pos) in &scan.pos_by_antenna {
        for pts in pos.iter().permutations(2) {
            match pts.as_slice() {
                [p1, p2] => match get_antinode(&scan, p1, p2) {
                    Some(p) => {
                        unique_pos.insert(p);
                    }
                    None => {}
                },
                _ => panic!("unexpected permutation"),
            }
        }
    }
    println!("{}", unique_pos.len());

    let mut unique_pos = HashSet::new();
    for (_, pos) in &scan.pos_by_antenna {
        for pts in pos.iter().permutations(2) {
            match pts.as_slice() {
                [p1, p2] => {
                    for p in get_antinode2(&scan, p1, p2) {
                        unique_pos.insert(p);
                    }
                }
                _ => panic!("unexpected permutation"),
            }
        }
    }
    println!("{}", unique_pos.len());
}
