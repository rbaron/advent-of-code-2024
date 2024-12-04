use std::{collections::HashMap, env::args, fs::File, io::BufRead};

fn count_occurences(
    row: i32,
    col: i32,
    word: &Vec<char>,
    charmap: &HashMap<(i32, i32), char>,
) -> i32 {
    let mut count = 0;
    for dy in vec![-1, 0, 1] {
        for dx in vec![-1, 0, 1] {
            let mut list: Vec<char> = Vec::new();
            for n in 0..word.len() as i32 {
                if let Some(&c) = charmap.get(&(row + n * dy, col + n * dx)) {
                    list.push(c);
                }
            }
            if list == *word {
                count += 1;
            }
        }
    }
    count
}

fn has_x(row: i32, col: i32, charmap: &HashMap<(i32, i32), char>) -> bool {
    let d1 = vec![
        charmap.get(&(row - 1, col - 1)).unwrap_or(&' '),
        charmap.get(&(row + 1, col + 1)).unwrap_or(&' '),
    ];
    let d2 = vec![
        charmap.get(&(row + 1, col - 1)).unwrap_or(&' '),
        charmap.get(&(row - 1, col + 1)).unwrap_or(&' '),
    ];
    charmap.get(&(row, col)) == Some(&'A')
        && vec![d1, d2]
            .iter()
            .all(|d| *d == vec![&'M', &'S'] || *d == vec![&'S', &'M'])
}

fn main() {
    let reader =
        std::io::BufReader::new(File::open(args().nth(1).expect("Missing filename")).unwrap());

    let lines = reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut charmap: HashMap<(i32, i32), char> = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, &c) in line.iter().enumerate() {
            let k = (row as i32, col as i32);
            charmap.insert(k, c);
        }
    }

    let mut xmas_count = 0;
    for row in 0..lines.len() {
        for col in 0..lines[0].len() {
            xmas_count += count_occurences(
                row.try_into().unwrap(),
                col.try_into().unwrap(),
                &vec!['X', 'M', 'A', 'S'],
                &charmap,
            );
        }
    }
    println!("{}", xmas_count);

    let mut x_count = 0;
    for row in 0..lines.len() - 1 {
        for col in 0..lines[0].len() - 1 {
            if has_x(row as i32, col as i32, &charmap) {
                x_count += 1;
            }
        }
    }
    println!("{}", x_count);
}
