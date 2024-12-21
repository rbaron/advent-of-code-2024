//  -----> x
// |
// |
// v y

use itertools::Itertools;
use std::{
    char,
    collections::{HashMap, HashSet},
    hash::Hash,
    i32,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

type Moves = Vec<char>;

impl Pos {
    fn dist(&self, other: &Pos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn diff(&self, other: &Pos) -> Pos {
        Pos {
            // x: other.x - self.x,
            // y: other.y - self.y,
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn to_moves(&self, allowed: &HashSet<Pos>) -> Vec<Moves> {
        let mut res: Vec<Vec<char>> = Vec::new();
        let mut moves = Vec::new();
        if self.x > 0 {
            moves.extend(vec!['>'; self.x.abs() as usize]);
        } else if self.x < 0 {
            moves.extend(vec!['<'; self.x.abs() as usize]);
        }
        if self.y > 0 {
            moves.extend(vec!['v'; self.y.abs() as usize]);
        } else if self.y < 0 {
            moves.extend(vec!['^'; self.y.abs() as usize]);
        }
        let perms = moves.iter().permutations(moves.len()).unique();
        for perm in perms {
            // println!("perm {:?} {:?}", perm, moves);
            let mut v: Vec<char> = perm.iter().map(|&c| *c).collect();
            v.push('A');
            res.push(v);
        }

        // if moves.len() == 1 {
        //     moves.push('A');
        //     res.push(moves);
        //     return res;
        // }
        // let mut clone = moves.clone();
        // clone.push('A');
        // res.push(clone);
        // moves.reverse();
        // moves.push('A');
        // res.push(moves);

        res
    }
}

// fn count_steps(
//     digits: Vec<char>,
//     keymap: &HashMap<char, Pos>,
//     sortmap: &HashMap<char, Pos>,
// ) -> Vec<char> {
//     let mut pos = keymap.get(&'A').unwrap();
//     let mut steps = Vec::new();
//     for digit in digits {
//         let new_pos = keymap.get(&digit).unwrap();
//         let diff = pos.diff(new_pos);
//         let moves = diff.to_moves(sortmap);
//         steps.extend(&moves);
//         pos = new_pos;
//     }
//     steps
// }

fn validate_moves(pos: &Pos, moves: &Moves, allowed: &HashSet<Pos>) -> bool {
    let mut pos = pos.clone();
    for m in moves {
        match m {
            '^' => pos.y -= 1,
            'v' => pos.y += 1,
            '<' => pos.x -= 1,
            '>' => pos.x += 1,
            _ => (),
        }
        if !allowed.contains(&pos) {
            return false;
        }
    }
    true
}

fn get_moves(
    digits: &[char],
    from_char: char,
    keymap: &HashMap<char, Pos>,
    allowed: &HashSet<Pos>,
    cache: &mut HashMap<(Vec<char>, char), Vec<Moves>>,
) -> Vec<Moves> {
    if digits.is_empty() {
        return vec![vec![]];
    }
    if cache.contains_key(&(digits.to_vec(), from_char)) {
        return cache.get(&(digits.to_vec(), from_char)).unwrap().clone();
    }
    let from = keymap.get(&from_char).unwrap();
    let to = keymap
        .get(&digits[0])
        .expect(format!("digit not found: {}", digits[0]).as_str());

    let moves = to
        .diff(from)
        .to_moves(&allowed)
        .iter()
        .filter(|m| validate_moves(from, m, allowed))
        .map(|m| m.clone())
        .collect::<Vec<Moves>>();

    // println!("{:?} {:?} -- {:?}", from, to, moves);
    let mut res: Vec<Moves> = Vec::new();
    for sub in get_moves(&digits[1..], digits[0], keymap, allowed, cache)
        .iter()
        .unique()
    {
        for m in &moves {
            let mut full: Moves = m.clone();
            full.extend(sub.clone());
            res.push(full);
        }
    }
    res = res.iter().unique().map(|m| m.clone()).collect();
    cache.insert((digits.to_vec(), from_char), res.clone());
    res
}

fn main() {
    let KEYPAD = HashMap::from([
        ('7', Pos { y: 0, x: 0 }),
        ('8', Pos { y: 0, x: 1 }),
        ('9', Pos { y: 0, x: 2 }),
        ('4', Pos { y: 1, x: 0 }),
        ('5', Pos { y: 1, x: 1 }),
        ('6', Pos { y: 1, x: 2 }),
        ('1', Pos { y: 2, x: 0 }),
        ('2', Pos { y: 2, x: 1 }),
        ('3', Pos { y: 2, x: 2 }),
        ('0', Pos { y: 3, x: 1 }),
        ('A', Pos { y: 3, x: 2 }),
    ]);
    let KEYPAD_ALLOWED: HashSet<Pos> = KEYPAD.values().map(|v| *v).collect();

    let ROBOT = HashMap::from([
        ('^', Pos { y: 0, x: 1 }),
        ('A', Pos { y: 0, x: 2 }),
        ('<', Pos { y: 1, x: 0 }),
        ('v', Pos { y: 1, x: 1 }),
        ('>', Pos { y: 1, x: 2 }),
    ]);
    let ROBOT_ALLOWED: HashSet<Pos> = ROBOT.values().map(|v| *v).collect();

    let filename = std::env::args().nth(1).unwrap();
    let inputs = std::fs::read_to_string(filename).unwrap();

    let mut res: i32 = 0;
    // let input = "456A";
    for input in inputs.split('\n') {
        let chars = input.chars().collect::<Vec<char>>();
        let moves1 = get_moves(&chars, 'A', &KEYPAD, &KEYPAD_ALLOWED, &mut HashMap::new());
        for m in moves1.iter().unique() {
            // println!("{:?}", m.len());
        }
        println!("move1 {:?}", moves1.len());

        let moves2: Vec<Moves> = moves1
            .iter()
            .unique()
            .flat_map(|m| get_moves(m, 'A', &ROBOT, &ROBOT_ALLOWED, &mut HashMap::new()))
            .collect();
        println!("move2 {:?}", moves2.len());
        // for m in moves2.iter().unique() {
        //     println!("{:?} {:?}", m.len(), m);
        // }
        // break;

        // let moves2 = vec![moves2.iter().min_by_key(|m| m.len()).unwrap()];
        let l = moves2.iter().map(|m| m.len()).min().unwrap();

        // let blks: Vec<Vec<Vec<Moves>>> = moves2
        //     .iter()
        //     .map(|m| {
        //         m.iter()
        //             .cloned()
        //             .collect::<Vec<char>>()
        //             .split(|&c| c == 'A')
        //             .map(|s| s.to_vec())
        //             .collect()
        //     })
        //     .collect();
        let blks: Vec<Vec<Moves>> = moves2
            .iter()
            .map(|m| {
                m.iter()
                    .cloned()
                    .collect::<Vec<char>>()
                    .split(|&c| c == 'A')
                    .map(|s| s.to_vec())
                    .collect()
            })
            .collect();

        // for blk in blks.iter() {
        //     for m in blk.iter() {
        //         println!("{:?} {:?}", m.len(), m);
        //     }
        //     println!();
        // }

        let mut cache: HashMap<String, i32> = HashMap::new();
        let mut min = i32::MAX;
        for blk in blks.iter().unique() {
            let mut size = 0;
            for m in blk.iter() {
                let key = m.iter().collect::<String>();
                if cache.contains_key(&key) {
                    size += cache.get(&key).unwrap();
                    // println!("skip {:?}", key);
                    continue;
                }
                let mut c = m.clone();
                c.push('A');
                let moves = get_moves(&c, 'A', &ROBOT, &ROBOT_ALLOWED, &mut HashMap::new());
                let shortest = moves.iter().min_by_key(|m| m.len()).unwrap();
                let val = shortest.len() as i32;
                cache.insert(key, val);
                size += val;
            }
            if size < min {
                min = size;
            }
        }

        println!("MIN {:?}", min - 1);

        // let moves3: Vec<Moves> = moves2
        //     .iter()
        //     .filter(|m| m.len() == l)
        //     .flat_map(|m| get_moves(m, 'A', &ROBOT, &ROBOT_ALLOWED, &mut HashMap::new()))
        //     .collect();
        // println!("{:?}", moves3.len());

        // let shortest = moves3.iter().min_by_key(|m| m.len()).unwrap();
        // println!("{:?} {:?}", input, shortest.len());
        // println!("{:?} {:?}", input, shortest.iter().collect::<String>());
        // break;

        // res += (input[..input.len() - 1].parse::<i32>().unwrap() * shortest.len() as i32);
        res += (input[..input.len() - 1].parse::<i32>().unwrap() * (min - 1) as i32);
    }
    println!("RES {}", res);
}
