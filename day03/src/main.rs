use regex::Regex;

#[derive(Debug)]
enum Instr {
    Do,
    Dont,
    Mul { a: i32, b: i32 },
}

fn eval(instrs: &[Instr], filter: fn(&&Instr) -> bool) -> i32 {
    instrs
        .iter()
        .filter(filter)
        .fold((0, true), |(acc, running), i| match (running, i) {
            (true, Instr::Dont) => (acc, false),
            (false, Instr::Do) => (acc, true),
            (true, Instr::Mul { a, b }) => (acc + a * b, true),
            _ => (acc, running),
        })
        .0
}

fn main() {
    let contents =
        std::fs::read_to_string(std::env::args().nth(1).expect("Missing filename")).unwrap();

    let patt = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don\'t\(\)").unwrap();

    let instrs: Vec<Instr> = patt
        .find_iter(&contents)
        .map(|c| match c.as_str() {
            "do()" => Instr::Do,
            "don't()" => Instr::Dont,
            _ => {
                let mut nums = c.as_str()[4..c.as_str().len() - 1]
                    .split(",")
                    .map(|s| s.trim().parse::<i32>().unwrap());
                let a = nums.next().unwrap();
                let b = nums.next().unwrap();
                Instr::Mul { a, b }
            }
        })
        .collect();

    println!(
        "Sum1: {}",
        eval(&instrs, |i| match i {
            Instr::Mul { .. } => true,
            _ => false,
        })
    );

    println!("Sum2: {}", eval(&instrs, |_| true));
}
