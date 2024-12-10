use std::collections::{HashMap, HashSet, VecDeque};

type Pos = (i32, i32);
type Path = Vec<Pos>;
type Map = HashMap<Pos, i32>;

fn neighbors(pos: &Pos, map: &Map) -> Vec<Pos> {
    let curr_height = map.get(pos).unwrap();
    let mut result = Vec::new();
    let (y, x) = pos;
    for (dy, dx) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let new_pos = (y + dy, x + dx);
        if let Some(height) = map.get(&new_pos) {
            if (height - curr_height) == 1 {
                result.push(new_pos);
            }
        }
    }
    result
}

fn count_paths(from: &Pos, map: &Map) -> i32 {
    let mut queue: VecDeque<Pos> = VecDeque::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    queue.push_back(*from);
    let mut sum = 0;
    while let Some(from) = queue.pop_front() {
        if visited.contains(&from) {
            continue;
        }
        visited.insert(from.clone());
        if let Some(height) = map.get(&from) {
            if *height == 9 {
                sum += 1
            }
        }
        neighbors(&from, &map).iter().for_each(|n| {
            queue.push_back(*n);
        });
    }
    sum
}

fn count_paths2(from: &Pos, map: &Map) -> i32 {
    let mut queue: VecDeque<(Path, Pos)> = VecDeque::new();
    let mut visited: HashSet<(Path, Pos)> = HashSet::new();
    queue.push_back((vec![(*from)], *from));
    let mut sum = 0;
    while let Some(entry) = queue.pop_front() {
        if visited.contains(&entry) {
            continue;
        }
        visited.insert(entry.clone());
        let (path, from) = entry;
        if let Some(height) = map.get(&from) {
            if *height == 9 {
                sum += 1
            }
        }
        neighbors(&from, &map).iter().for_each(|n| {
            let mut new_path = path.clone();
            new_path.push(from);
            queue.push_back((new_path, *n));
        });
    }
    sum
}

fn main() {
    let map: Map = std::fs::read_to_string(std::env::args().nth(1).expect("Missing filename"))
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((y as i32, x as i32), c.to_string().parse().unwrap_or(1000)))
        })
        .collect();

    let zeros: Vec<&Pos> = map
        .iter()
        .filter(|(_k, v)| **v == 0)
        .map(|(k, _v)| k)
        .collect();

    let sum: i32 = zeros.iter().map(|p| count_paths(*p, &map)).sum();
    println!("{}", sum);

    let sum: i32 = zeros.iter().map(|p| count_paths2(*p, &map)).sum();
    println!("{}", sum);
}
