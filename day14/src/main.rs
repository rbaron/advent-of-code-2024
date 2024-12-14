use regex::Regex;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    y: i32,
    x: i32,
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }

    fn mul(&self, scalar: i32) -> Point {
        Point {
            y: self.y * scalar,
            x: self.x * scalar,
        }
    }

    fn div(&self, scalar: i32) -> Point {
        Point {
            y: self.y / scalar,
            x: self.x / scalar,
        }
    }

    fn mod_point(&self, other: &Point) -> Point {
        Point {
            y: ((self.y % other.y) + other.y) % other.y,
            x: ((self.x % other.x) + other.x) % other.x,
        }
    }
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Point,
    vel: Point,
}

fn parse_robots(robots: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(\-?\d+),(\-?\d+) v=(\-?\d+),(\-?\d+)").unwrap();
    robots
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Robot {
                pos: Point {
                    x: caps[1].parse().unwrap(),
                    y: caps[2].parse().unwrap(),
                },
                vel: Point {
                    x: caps[3].parse().unwrap(),
                    y: caps[4].parse().unwrap(),
                },
            }
        })
        .collect()
}

fn get_quadrant(p: &Point, size: &Point) -> Option<i32> {
    let zero = size.div(2);
    // Shift to new coord system.
    let pos = p.add(&zero.mul(-1));
    if pos.y == 0 || pos.x == 0 {
        return None;
    }
    match (pos.y > 0, pos.x > 0) {
        (true, true) => Some(1),   // Bottom right.
        (true, false) => Some(2),  // Bottom left.
        (false, false) => Some(3), // Top left.
        (false, true) => Some(4),  // Top right.
    }
}

fn draw_robots(robots: &Vec<Robot>, size: &Point) {
    let mut grid = vec![vec![' '; size.x as usize]; size.y as usize];
    for r in robots {
        grid[r.pos.y as usize][r.pos.x as usize] = '#';
    }
    println!("Grid size: {:?} x {:?}", grid.len(), grid[0].len());
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let robots = parse_robots(&std::fs::read_to_string(&filename).unwrap());

    let size = match filename.contains("test") {
        true => Point { y: 7, x: 11 },
        false => Point { y: 103, x: 101 },
    };

    let steps = 100;
    let mut counts = vec![0; 4];
    for r in &robots {
        let new_pos = r.pos.add(&r.vel.mul(steps)).mod_point(&size);
        if let Some(quad) = get_quadrant(&new_pos, &size) {
            counts[(quad - 1) as usize] += 1;
        }
    }
    let res1 = counts.iter().fold(1, |acc, x| acc * x);
    println!("{}", res1);

    for step in 0.. {
        let mut new_robots = robots.clone();
        for r in new_robots.iter_mut() {
            r.pos = r.pos.add(&r.vel.mul(step)).mod_point(&size);
        }

        // I realize two interesting patterns repeat:
        // - A vertical one every 101 steps, beginning at 13
        // - A horizontal one every 103 steps, beginning at 89
        // I guessed something even more interesting would happen whenever both meet,
        // and it turns out that's true.
        if (step + 1 - 89) % 103 == 0 && (step + 1 - 13) % 101 == 0 {
            println!("{}", step);
            draw_robots(&new_robots, &size);
            break;
        }
    }
}
