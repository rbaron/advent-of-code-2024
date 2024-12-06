use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn step(&self, pos: &Pos) -> Pos {
        let (y, x) = pos;
        match self {
            Dir::Up => (y - 1, *x),
            Dir::Down => (y + 1, *x),
            Dir::Left => (*y, x - 1),
            Dir::Right => (*y, x + 1),
        }
    }

    fn turn_right(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

fn find_exit(pos: &Pos, dir: Dir, grid: &HashMap<Pos, char>) -> Result<HashSet<Pos>, ()> {
    let mut pos = *pos;
    let mut dir = dir;
    let mut visited: HashSet<(Pos, Dir)> = HashSet::new();
    let mut positions: HashSet<Pos> = HashSet::new();
    let obs = (6, 3);
    loop {
        // Error if there's a loop.
        if visited.contains(&(pos, dir)) {
            return Err(());
        }
        visited.insert((pos, dir));
        positions.insert(pos);
        let next_pos = dir.step(&pos);
        match grid.get(&next_pos) {
            Some('#') => {
                dir = dir.turn_right();
            }
            None => {
                return Ok(positions);
            }
            Some(_) => {
                pos = next_pos;
            }
        }
    }
}

fn main() {
    let mut grid: HashMap<Pos, char> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .expect("Failed to read input")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .fold(HashMap::new(), move |mut acc, (x, c)| {
                    acc.insert((y as i32, x as i32), c);
                    acc
                })
        })
        .collect();

    let initial_pos = *grid.iter().find(|(_, &c)| c == '^').unwrap().0;

    grid.insert(initial_pos, '.');
    let visited = find_exit(&initial_pos, Dir::Up, &grid).unwrap();
    println!("{}", visited.len());

    let mut n_loops = 0;
    for maybe_obst in visited {
        grid.insert(maybe_obst, '#');
        match find_exit(&initial_pos, Dir::Up, &grid) {
            Err(_) => {
                n_loops += 1;
            }
            Ok(_) => {}
        }
        grid.insert(maybe_obst, '.');
    }
    println!("{}", n_loops);
}
