#[derive(Debug)]
struct Test {
    test_val: i64,
    // Operands stored in reverse order.
    operands: Vec<i64>,
}

fn eval_operators(operands: &[i64]) -> Vec<i64> {
    match operands.split_first() {
        Some((head, tail)) => match tail.len() {
            0 => vec![*head],
            _ => {
                let mut results = Vec::new();
                for r in eval_operators(tail) {
                    results.push(*head + r);
                    results.push(*head * r);

                    let concat: i64 = (r.to_string() + &head.to_string())
                        .to_owned()
                        .parse()
                        .unwrap();

                    // let digits = (r as f64).log10().floor() as u32 + 1;
                    // println!("digits: {}", digits);
                    // let concat = r * 10_i64.pow(digits) + head;
                    results.push(concat);
                }

                results
            }
        },
        None => panic!("No operands"),
    }
}

fn main() {
    let contents = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut tests = Vec::new();
    for line in contents.lines() {
        match line.split(":").collect::<Vec<&str>>().as_slice() {
            [test_val, operands] => {
                let mut t = Test {
                    test_val: test_val.parse().unwrap(),
                    operands: operands
                        .split(" ")
                        .filter(|x| !x.is_empty())
                        .map(|x| x.parse().unwrap())
                        .collect(),
                };
                t.operands.reverse();
                tests.push(t);
            }
            _ => {
                panic!("Invalid input");
            }
        }
    }

    let sum1 = tests
        .iter()
        .map(|t| {
            if eval_operators(&t.operands).iter().any(|x| *x == t.test_val) {
                t.test_val
            } else {
                0
            }
        })
        .sum::<i64>();

    println!("{}", sum1);
}
