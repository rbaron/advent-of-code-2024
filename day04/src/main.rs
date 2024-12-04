use std::{collections::HashSet, env::args, fs::File, io::BufRead};

fn transpose(lines: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed = vec![vec![' '; lines.len()]; lines[0].len()];
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            transposed[j][i] = lines[i][j];
        }
    }
    transposed
}

fn count_word(line: impl Iterator<Item = char>, word: &str) -> usize {
    let line = line.collect::<String>();
    let word_rev = word.chars().rev().collect::<String>();
    line.match_indices(word).count() + line.match_indices(&word_rev).count()
}

struct WordMatrix {
    lines: Vec<Vec<char>>,
}

struct DiagIter<'a> {
    lines: &'a Vec<Vec<char>>,
    row: usize,
    col: usize,
    dx: i32,
    dy: i32,
}

// Iterates over chars in the diagonal direction of a matrix.
impl Iterator for DiagIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.lines.len().try_into().unwrap()
            || self.col >= self.lines[0].len().try_into().unwrap()
        {
            return None;
        }
        let c = self.lines[self.row][self.col];
        self.row = (self.row as i32 + self.dy) as usize;
        self.col = (self.col as i32 + self.dx) as usize;
        Some(c)
    }
}

impl WordMatrix {
    // Returns iterators over the diagonals of the matrix, starting at the first element of each row and column.
    fn diag_iters<'a>(&'a self) -> impl Iterator<Item = DiagIter> + 'a {
        let h = self.lines.len() as i32;
        let w = self.lines[0].len() as i32;
        let row_iters = (0..h as i32)
            .map(move |i| DiagIter {
                lines: &self.lines,
                row: i as usize,
                col: 0,
                dx: 1,
                dy: 1,
            })
            .into_iter()
            .chain((1..w as i32).map(move |i| DiagIter {
                lines: &self.lines,
                row: 0,
                col: i as usize,
                dx: 1,
                dy: 1,
            }));
        let col_iters = (0..h as i32)
            .map(move |i| DiagIter {
                lines: &self.lines,
                row: i as usize,
                col: 0,
                dx: 1,
                dy: -1,
            })
            .into_iter()
            .chain((1..w as i32).map(move |i| DiagIter {
                lines: &self.lines,
                row: h as usize - 1,
                col: i as usize,
                dx: 1,
                dy: -1,
            }));
        row_iters.chain(col_iters)
    }
}

fn has_x(row: usize, col: usize, lines: &Vec<Vec<char>>) -> bool {
    let w = lines[0].len();
    let h = lines.len();

    if (row < 1 || row >= h - 1 || col < 1 || col >= w - 1 || lines[row][col] != 'A') {
        return false;
    }

    let diag1 = HashSet::from([lines[row - 1][col - 1], lines[row + 1][col + 1]]);
    let diag2 = HashSet::from([lines[row - 1][col + 1], lines[row + 1][col - 1]]);
    let expected = HashSet::from(['M', 'S']);

    diag1 == expected && diag2 == expected
}

fn main() {
    let reader =
        std::io::BufReader::new(File::open(args().nth(1).expect("Missing filename")).unwrap());

    let lines = reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let matrix = WordMatrix { lines };

    let word = "XMAS";
    let diag_count = matrix
        .diag_iters()
        .map(|d| count_word(d, word))
        .sum::<usize>();

    let line_count = matrix
        .lines
        .iter()
        .map(|line| count_word(line.iter().copied(), word))
        .sum::<usize>();

    let col_count = transpose(&matrix.lines)
        .iter()
        .map(|line| count_word(line.iter().copied(), word))
        .sum::<usize>();

    println!("{}", diag_count + line_count + col_count);

    let lines = &matrix.lines;

    let mut x_count = 0;
    for row in 1..lines.len() - 1 {
        for col in 1..lines[0].len() - 1 {
            if has_x(row, col, lines) {
                x_count += 1;
            }
        }
    }
    println!("{}", x_count);
}
