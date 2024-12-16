use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    y: i32,
    x: i32,
}

type Dir = usize;
const DIRS: [Pos; 4] = [
    Pos { y: 0, x: 1 },  // East.
    Pos { y: 1, x: 0 },  // South.
    Pos { y: 0, x: -1 }, // West.
    Pos { y: -1, x: 0 }, // North.
];

type Cost = usize;

impl Pos {
    fn add(&self, other: Pos) -> Pos {
        Pos {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }

    fn neighbors(&self, dir: usize) -> [(Pos, Dir, Cost); 4] {
        [
            (self.add(DIRS[dir]), dir, 1),         // Go straight.
            (*self, (dir + 1) % DIRS.len(), 1000), // Turn right.
            (*self, (dir + 3) % DIRS.len(), 1000), // Turn left.
            (*self, (dir + 2) % DIRS.len(), 2000), // Turn around (probably not needed?).
        ]
    }
}

type Grid = HashMap<Pos, char>;

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Pos {
                        y: y as i32,
                        x: x as i32,
                    },
                    c,
                )
            })
        })
        .collect()
}

#[allow(dead_code)]
fn draw_grid(grid: &Grid, pos: HashSet<Pos>) {
    let min_x = grid.keys().map(|p| p.x).min().unwrap();
    let max_x = grid.keys().map(|p| p.x).max().unwrap();
    let min_y = grid.keys().map(|p| p.y).min().unwrap();
    let max_y = grid.keys().map(|p| p.y).max().unwrap();

    let mut grid = grid.clone();
    for p in pos {
        grid.insert(p, 'O');
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = grid.get(&Pos { y, x }).unwrap_or(&'#');
            print!("{}", c);
        }
        println!();
    }
}

fn shortest_path(start: &Pos, dir: Dir, end: &Pos, grid: &Grid) -> (Cost, HashSet<Pos>) {
    let mut dist_by_node = HashMap::new();
    let mut visited: HashSet<(Pos, Dir)> = HashSet::new();
    let mut from = HashMap::new();
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, (*start, dir), vec![*start])));
    dist_by_node.insert((*start, dir), 0);

    let mut min_dist = usize::MAX;
    let mut paths = Vec::new();
    while !q.is_empty() {
        let u = q.pop().unwrap();

        let Reverse((cost, (pos, dir), path)) = u;
        if cost > *dist_by_node.get(&(pos, dir)).unwrap_or(&usize::MAX) {
            continue;
        }

        if cost > min_dist {
            continue;
        }

        let k = (pos, dir);
        if pos == *end {
            let dist = dist_by_node[&k];
            if dist <= min_dist {
                min_dist = dist;
                paths.push(path.clone());
            }
        }

        visited.insert(k);

        for &(v, dir, cost) in pos.neighbors(dir).iter() {
            if grid.get(&v).unwrap_or(&'#') == &'#' {
                continue;
            }
            let k2 = (v, dir);
            if visited.contains(&k2) {
                continue;
            }

            let alt = dist_by_node.get(&k).unwrap_or(&(usize::MAX / 2)) + cost;
            if alt <= *dist_by_node.get(&k2).unwrap_or(&usize::MAX) {
                dist_by_node.insert(k2, alt);
                let mut new_path = path.clone();
                new_path.push(v);
                q.push(Reverse((alt, k2, new_path)));
                from.insert(k2, k);
            }
        }
    }
    let mut all_pos = HashSet::new();
    for p in &paths {
        all_pos.extend(p.iter().cloned());
    }
    (min_dist, all_pos)
}

fn main() {
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .expect("Failed to read input file");
    let grid = parse_input(&input);

    let start = grid.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let end = grid.iter().find(|(_, &c)| c == 'E').unwrap().0;

    let (dist, pos) = shortest_path(start, 0, end, &grid);

    println!("{}", dist);
    println!("{}", pos.len());
}
