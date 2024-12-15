use core::panic;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    y: i32,
    x: i32,
}

impl Pos {
    fn add(&self, other: &Pos) -> Pos {
        Pos {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn from_char(c: char) -> Option<Move> {
        match c {
            '^' => Some(Move::Up),
            'v' => Some(Move::Down),
            '<' => Some(Move::Left),
            '>' => Some(Move::Right),
            _ => panic!("Invalid move: {}", c),
        }
    }

    fn to_pos(&self) -> Pos {
        match self {
            Move::Up => Pos { y: -1, x: 0 },
            Move::Right => Pos { y: 0, x: 1 },
            Move::Down => Pos { y: 1, x: 0 },
            Move::Left => Pos { y: 0, x: -1 },
        }
    }
}

type Grid = HashMap<Pos, char>;

fn parse_input(input: &str) -> (Grid, Vec<Move>) {
    let blks: Vec<&str> = input.split("\n\n").collect();

    let grid: Grid = blks[0]
        .split("\n")
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
        .collect();

    (
        grid,
        blks[1]
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| Move::from_char(c).unwrap())
            .collect(),
    )
}

fn try_move(pos: &Pos, _move: &Move, grid: &mut Grid) -> Pos {
    let c = match grid.get(pos) {
        Some('#') => return *pos,
        Some('.') => return *pos,
        Some(c) => *c,
        _ => panic!("Invalid position: {:?}", pos),
    };

    let next_pos = pos.add(&_move.to_pos());
    let next_c = grid.get(&next_pos).unwrap();
    let mut test_grid: Grid = grid.clone();

    if *_move == Move::Up {
        if *next_c == '[' {
            let dx = next_pos.add(&Move::Right.to_pos());
            try_move(&next_pos, _move, &mut test_grid);
            try_move(&dx, _move, &mut test_grid);
            if test_grid.get(&next_pos) == Some(&'.') && test_grid.get(&dx) == Some(&'.') {
                test_grid.insert(next_pos, c);
                test_grid.insert(dx, '.');
                test_grid.insert(*pos, '.');
                *grid = test_grid;
            }
            return next_pos;
        } else if *next_c == ']' {
            let dx = next_pos.add(&Move::Left.to_pos());
            try_move(&next_pos, _move, &mut test_grid);
            try_move(&dx, _move, &mut test_grid);
            if test_grid.get(&next_pos) == Some(&'.') && test_grid.get(&dx) == Some(&'.') {
                test_grid.insert(next_pos, c);
                test_grid.insert(dx, '.');
                test_grid.insert(*pos, '.');
                *grid = test_grid;
            }
            return next_pos;
        }
    } else if *_move == Move::Down {
        if *next_c == '[' {
            let dx = next_pos.add(&Move::Right.to_pos());
            try_move(&next_pos, _move, &mut test_grid);
            try_move(&dx, _move, &mut test_grid);
            if test_grid.get(&next_pos) == Some(&'.') && test_grid.get(&dx) == Some(&'.') {
                test_grid.insert(next_pos, c);
                test_grid.insert(dx, '.');
                test_grid.insert(*pos, '.');
                *grid = test_grid;
            }
            return next_pos;
        } else if *next_c == ']' {
            let dx = next_pos.add(&Move::Left.to_pos());
            try_move(&next_pos, _move, &mut test_grid);
            try_move(&dx, _move, &mut test_grid);
            if test_grid.get(&next_pos) == Some(&'.') && test_grid.get(&dx) == Some(&'.') {
                test_grid.insert(next_pos, c);
                test_grid.insert(dx, '.');
                test_grid.insert(*pos, '.');
                *grid = test_grid;
            }
            return next_pos;
        }
    }

    // Otherwise keep doing the move.
    try_move(&next_pos, _move, grid);
    // If next move was successfully executed, move current position.
    if let Some(&'.') = grid.get(&next_pos) {
        grid.insert(*pos, '.');
        grid.insert(pos.add(&_move.to_pos()), c);
        return next_pos;
    }
    *pos
}

fn expand_grid(grid: &Grid) -> Grid {
    grid.iter()
        .flat_map(|(k, &v)| {
            let new_pos = vec![
                Pos { y: k.y, x: 2 * k.x },
                Pos {
                    y: k.y,
                    x: 2 * k.x + 1,
                },
            ];
            match v {
                '.' => vec![(new_pos[0], '.'), (new_pos[1], '.')],
                '#' => vec![(new_pos[0], '#'), (new_pos[1], '#')],
                'O' => vec![(new_pos[0], '['), (new_pos[1], ']')],
                '@' => vec![(new_pos[0], '@'), (new_pos[1], '.')],
                _ => panic!("Invalid char: {}", v),
            }
        })
        .collect()
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    let min_x = grid.keys().map(|p| p.x).min().unwrap();
    let max_x = grid.keys().map(|p| p.x).max().unwrap();
    let min_y = grid.keys().map(|p| p.y).min().unwrap();
    let max_y = grid.keys().map(|p| p.y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", grid.get(&Pos { y, x }).unwrap());
        }
        println!();
    }
}

fn main() {
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let (grid, dirs) = parse_input(&input);

    let mut grid1 = grid.clone();
    let mut robot_pos = *grid.iter().find(|(_, &c)| c == '@').unwrap().0;
    for dir in &dirs {
        robot_pos = try_move(&robot_pos, &dir, &mut grid1);
    }
    let sum: i32 = grid1
        .iter()
        .filter(|(_, &c)| c == 'O')
        .map(|(k, _)| k.y * 100 + k.x)
        .sum();
    println!("{:?}", sum);

    let mut grid2 = expand_grid(&grid);
    let mut robot_pos = *grid2.iter().find(|(_, &c)| c == '@').unwrap().0;
    for dir in &dirs {
        try_move(&robot_pos, &dir, &mut grid2);
        // print_grid(&grid2);
        // println!();
        robot_pos = *grid2.iter().find(|(_, &c)| c == '@').unwrap().0;
    }
    let sum: i32 = grid2
        .iter()
        .filter(|(_, &c)| c == '[')
        .map(|(k, _)| k.y * 100 + k.x)
        .sum();
    println!("{:?}", sum);
}
