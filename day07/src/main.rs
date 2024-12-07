#[derive(Debug)]
struct Test {
    test_val: u64,
    // Operands stored in reverse order so I can evaluate the tail first and
    // apply the original left-to-right eval order easily.
    operands: Vec<u64>,
}

fn eval_operators(operands: &[u64], use_concat: bool) -> Vec<u64> {
    match operands.split_first() {
        Some((head, tail)) => match tail.len() {
            0 => vec![*head],
            _ => {
                let mut results = Vec::new();
                for r in eval_operators(tail, use_concat) {
                    results.push(*head + r);
                    results.push(*head * r);

                    if !use_concat {
                        continue;
                    }
                    // Concat by multiplying the head by 10^(# head digits) and adding the head.
                    let digits = (*head as f64).log10().floor() as u32 + 1;
                    let concat = r * 10_u64.pow(digits) + *head;
                    results.push(concat as u64);
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
            if eval_operators(&t.operands, /*use_concat=*/ false)
                .iter()
                .any(|x| *x == t.test_val)
            {
                t.test_val
            } else {
                0
            }
        })
        .sum::<u64>();
    println!("{}", sum1);

    let sum2 = tests
        .iter()
        .map(|t| {
            if eval_operators(&t.operands, /*use_concat=*/ true)
                .iter()
                .any(|x| *x == t.test_val)
            {
                t.test_val
            } else {
                0
            }
        })
        .sum::<u64>();
    println!("{}", sum2);
}
