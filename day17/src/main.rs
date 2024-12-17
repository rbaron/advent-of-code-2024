use core::panic;

#[derive(Debug)]
struct State {
    ip: usize,
    a: i64,
    b: i64,
    c: i64,
}

impl State {
    fn new() -> State {
        State {
            ip: 0,
            a: 0,
            b: 0,
            c: 0,
        }
    }
}

type Program = Vec<u8>;

fn parse(input: &str) -> (State, Program) {
    let blks = input.split("\n\n").collect::<Vec<_>>();
    let state = blks[0]
        .split("\n")
        .enumerate()
        .fold(State::new(), |acc, (i, l)| {
            let val = l.split(": ").nth(1).unwrap().parse().unwrap();
            match i {
                0 => State { a: val, ..acc },
                1 => State { b: val, ..acc },
                2 => State { c: val, ..acc },
                _ => panic!("too many lines"),
            }
        });
    let program = blks[1]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    (state, program)
}

fn combo(op: u8, state: &State) -> i64 {
    match op {
        0..=3 => op as i64,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => panic!("unknown op"),
    }
}

fn run(program: &Program, state: &mut State) -> Vec<u8> {
    let mut stdout = Vec::new();
    loop {
        // println!(
        //     "ip: {} op: {} | {}, a: {}, b: {}, c: {}",
        //     state.ip,
        //     program[state.ip],
        //     program[state.ip + 1],
        //     state.a,
        //     state.b,
        //     state.c
        // );
        match program.get(state.ip as usize) {
            Some(0) => state.a = state.a / 2i64.pow(combo(program[state.ip + 1], state) as u32),
            Some(1) => state.b = state.b ^ program[state.ip + 1] as i64,
            Some(2) => state.b = combo(program[state.ip + 1], state) % 8,
            Some(3) => {
                if state.a != 0 {
                    state.ip = program[state.ip + 1] as usize;
                    continue;
                }
            }
            Some(4) => state.b = state.b ^ state.c,
            Some(5) => stdout.push(combo(program[state.ip + 1], state) as u8 % 8),
            Some(6) => state.b = state.a / 2i64.pow(combo(program[state.ip + 1], state) as u32),
            Some(7) => state.c = state.a / 2i64.pow(combo(program[state.ip + 1], state) as u32),
            Some(k) => panic!("unknown op: {}", k),
            None => return stdout,
        }
        state.ip += 2;
    }
}

/*
| pos | op  | operand | update                         |
| --- | --- | ------- | ------------------------------ |
| 0   | 2   | 4       | b = a % 8                      |
| 2   | 1   | 3       | b = b ^ 3 = b ^ 0b11           |
| 4   | 7   | 5       | c = a / 2**b                   |
| 6   | 1   | 5       | b = b ^ 5                      |
| 8   | 0   | 3       | a = a / 8                      |
| 10  | 4   | 3       | b = b ^ c                      |
| 12  | 5   | 5       | print(b % 8)                   |
| 14  | 3   | 0       | if a => goto 0; otherwise HALT |
*/
fn run_one(a: i64) -> u8 {
    let b = a % 8;
    let b = b ^ 3;
    let c = a >> b;
    let b = b ^ 5 ^ c;
    return (b % 8) as u8;
}

fn find(a: i64, idx: isize, program: &Program) -> Option<i64> {
    if idx < 0 {
        // We overshot.
        return Some(a >> 3);
    }

    let digit = program[idx as usize];
    if run_one(a) != digit {
        return None;
    }

    let min = (0..8)
        .map(|a2| find((a << 3) | a2, idx - 1, program))
        .filter(|x| x.is_some())
        .min()?;
    min
}

fn main() {
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let (state, program) = parse(&input);

    let stdout = run(&program, &mut State { ..state });

    let stdout = stdout
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", stdout);

    let a = (0..8)
        .map(|i| find(i, program.len() as isize - 1, &program))
        .filter(|x| x.is_some())
        .min()
        .unwrap();
    println!("{:?}", a.unwrap());
}
