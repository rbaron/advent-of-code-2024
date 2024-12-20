use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn add(&self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn dist(&self, other: &Pos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn neighbors(&self) -> Vec<Pos> {
        vec![
            Pos { x: 0, y: 1 },
            Pos { x: 0, y: -1 },
            Pos { x: 1, y: 0 },
            Pos { x: -1, y: 0 },
        ]
        .into_iter()
        .map(|p| self.add(p))
        .collect()
    }
}

type Grid = HashMap<Pos, char>;

fn find_path(
    pos: &Pos,
    end: &Pos,
    path: Vec<Pos>,
    grid: &Grid,
    visited: &mut HashSet<Pos>,
) -> Vec<Pos> {
    if visited.contains(&(*pos)) {
        return vec![];
    }

    visited.insert(*pos);

    if pos == end {
        let mut new_path = path.clone();
        new_path.push(*pos);
        return new_path;
    }

    let mut res = vec![];
    for neighbor in pos.neighbors() {
        if !grid.contains_key(&neighbor) {
            continue;
        }
        let mut new_visited = visited.clone();
        let mut new_path = path.clone();
        new_path.push(*pos);
        match *grid.get(&neighbor).unwrap() {
            '#' | 'S' => (),
            '.' | 'E' => {
                res.extend(find_path(
                    &neighbor,
                    &end,
                    new_path,
                    &grid,
                    &mut new_visited,
                ));
            }
            c => panic!("unexpected char '{}'", c),
        }
    }
    res
}

fn main() {
    let filename = std::env::args().nth(1).expect("no filename given");
    let grid: HashMap<Pos, char> = std::fs::read_to_string(&filename)
        .expect("failed to read file")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Pos {
                        x: x as i32,
                        y: y as i32,
                    },
                    c,
                )
            })
        })
        .collect();

    let get_char_pos = |c| grid.iter().find(|&(_, &c2)| c == c2).map(|(pos, _)| *pos);
    let start = get_char_pos('S').unwrap();
    let end = get_char_pos('E').unwrap();

    let path = find_path(&start, &end, vec![], &grid, &mut HashSet::new());

    let dist_by_node: HashMap<Pos, i32> = path
        .iter()
        .enumerate()
        .map(|(i, &pos)| (pos, i as i32))
        .collect();

    let count_gains = |max_dist| {
        let mut gains = 0;
        for v in path.iter().combinations(2) {
            let (a, b) = (v[0], v[1]);
            if a.dist(&b) <= max_dist {
                let gain = (dist_by_node[&a] - dist_by_node[&b]).abs() - a.dist(b);
                if gain >= 100 {
                    gains += 1;
                }
            }
        }
        gains
    };

    let gains1 = count_gains(2);
    println!("{}", gains1);

    let gains2 = count_gains(20);
    println!("{}", gains2);
}
