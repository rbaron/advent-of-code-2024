use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    y: i32,
    x: i32,
}

type Cost = usize;

impl Pos {
    fn add(&self, other: Pos) -> Pos {
        Pos {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }

    fn neighbors(&self) -> [Pos; 4] {
        [
            self.add(Pos { y: -1, x: 0 }),
            self.add(Pos { y: 0, x: 1 }),
            self.add(Pos { y: 1, x: 0 }),
            self.add(Pos { y: 0, x: -1 }),
        ]
    }
}

type Grid = HashMap<Pos, char>;

fn parse_input(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|l| l.split(",").map(|s| s.parse::<i32>().unwrap()))
        .map(|mut s| Pos {
            x: s.next().unwrap(),
            y: s.next().unwrap(),
        })
        .collect()
}

// Manhattan distance.
fn heuristic_cost_estimate(start: &Pos, end: &Pos) -> Cost {
    (end.y - start.y).abs() as usize + (end.x - start.x).abs() as usize
}

fn a_star(start: &Pos, end: &Pos, grid: &Grid, h: i32, w: i32) -> Option<Cost> {
    let mut open_set = BinaryHeap::from([Reverse((heuristic_cost_estimate(start, end), *start))]);

    let mut g_score = HashMap::new();
    g_score.insert(*start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(*start, heuristic_cost_estimate(start, end));

    while !open_set.is_empty() {
        let (_, current) = open_set.pop().unwrap().0;

        if current == *end {
            return Some(g_score[&current]);
        }

        for &npos in current.neighbors().iter() {
            if npos.y < 0 || npos.y >= h || npos.x < 0 || npos.x >= w {
                continue;
            }
            if grid.get(&npos).unwrap_or(&'.') == &'#' {
                continue;
            }
            let tentative_g_score = g_score[&current] + 1;
            if tentative_g_score < *g_score.get(&npos).unwrap_or(&usize::MAX) {
                g_score.insert(npos, tentative_g_score);
                f_score.insert(
                    npos,
                    tentative_g_score + heuristic_cost_estimate(&npos, end),
                );
                open_set.push(Reverse((f_score[&npos], npos)));
            }
        }
    }
    None
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(&filename).unwrap();
    let bytes = parse_input(&input);

    let (exit, n_bytes) = match filename.as_str() {
        s if s.contains("test") => (Pos { y: 6, x: 6 }, 12),
        _ => (Pos { y: 70, x: 70 }, 1024),
    };

    let grid = bytes
        .iter()
        .take(n_bytes)
        .fold(HashMap::new(), |mut grid, pos| {
            grid.insert(*pos, '#');
            grid
        });

    let min_cost = a_star(&Pos { y: 0, x: 0 }, &exit, &grid, exit.y + 1, exit.x + 1);
    println!("{}", min_cost.unwrap());

    let mut grid = grid.clone();
    for byte in bytes.iter().skip(n_bytes) {
        grid.insert(*byte, '#');
        let cost = a_star(&Pos { y: 0, x: 0 }, &exit, &grid, exit.y + 1, exit.x + 1);
        if let None = cost {
            println!("{},{}", byte.x, byte.y);
            break;
        }
    }
}
