use std::collections::{HashMap, HashSet, VecDeque};

type Pos = (i32, i32); // y, x
type Grid = HashMap<Pos, char>;

#[derive(Debug)]
struct Entry {
    area: i32,
    peri: i32,
    cells: HashSet<Pos>,
}

fn flood_fill(pos: &Pos, grid: &Grid, visited: &mut HashSet<Pos>) -> Option<Entry> {
    if visited.contains(pos) {
        return None;
    }
    let c = grid.get(&pos).unwrap();
    let mut queue = VecDeque::from([*pos]);
    let mut entry = Entry {
        area: 0,
        peri: 0,
        cells: HashSet::new(),
    };

    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        entry.area += 1;
        entry.cells.insert(pos);
        let (y, x) = pos;
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let np = (y + dy, x + dx);
            if let Some(nc) = grid.get(&np) {
                if nc != c {
                    entry.peri += 1;
                } else if !visited.contains(&np) {
                    queue.push_back(np);
                }
            } else {
                entry.peri += 1;
            }
        }
    }
    Some(entry)
}

fn find_areas(grid: &Grid) -> Vec<Entry> {
    let mut visited = HashSet::new();
    let mut entries = Vec::new();
    for pos in grid.keys() {
        if let Some(entry) = flood_fill(pos, grid, &mut visited) {
            entries.push(entry);
        }
    }
    entries
}

fn compute_fence_cost1(grid: &Grid) -> i32 {
    find_areas(&grid).iter().map(|e| e.area * e.peri).sum()
}

fn compute_fence_cost2(grid: &Grid) -> i32 {
    let areas = find_areas(&grid);
    let mut sum = 0;
    for a in areas.iter() {
        let mut sides = 0;
        for pos in &a.cells {
            let (y, x) = *pos;
            let c = grid.get(&pos).unwrap();
            let default = &'☠';
            let top = grid.get(&(y - 1, x)).unwrap_or(default);
            let right = grid.get(&(y, x + 1)).unwrap_or(default);
            let bottom = grid.get(&(y + 1, x)).unwrap_or(default);
            let left = grid.get(&(y, x - 1)).unwrap_or(default);
            if top != c && right != c {
                sides += 1;
            }
            if top != c && left != c {
                sides += 1;
            }
            if bottom != c && right != c {
                sides += 1;
            }
            if bottom != c && left != c {
                sides += 1;
            }
            // Top right corner.
            if top == c && right == c && grid.get(&(y - 1, x + 1)).unwrap_or(default) != c {
                sides += 1;
            }
            // Bottom right corner.
            if bottom == c && right == c && grid.get(&(y + 1, x + 1)).unwrap_or(default) != c {
                sides += 1;
            }
            // Bottom left corner.
            if bottom == c && left == c && grid.get(&(y + 1, x - 1)).unwrap_or(default) != c {
                sides += 1;
            }
            // Top left corner.
            if top == c && left == c && grid.get(&(y - 1, x - 1)).unwrap_or(default) != c {
                sides += 1;
            }
        }
        // println!(
        //     "{} {:?}, Sides: {} -- cost {}",
        //     grid.get(&a.poss.iter().next().unwrap()).unwrap(),
        //     a.poss,
        //     sides,
        //     a.area * sides
        // );
        sum += a.area * sides;
    }
    sum
}

fn main() {
    let grid: Grid = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((y as i32, x as i32), c))
        })
        .collect();

    println!("{}", compute_fence_cost1(&grid));
    println!("{}", compute_fence_cost2(&grid));
}
