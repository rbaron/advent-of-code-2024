use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Binary,
    mem,
};

#[derive(PartialEq, Eq, Debug)]
struct MemBlk {
    pos: usize,
    len: usize,
}

impl Ord for MemBlk {
    // Sort by start address in descending order.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pos.cmp(&other.pos).reverse()
    }
}

impl PartialOrd for MemBlk {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct File {
    id: u32,
    pos: usize,
    size: usize,
}

fn parse(input: &str) -> (BinaryHeap<File>, BinaryHeap<MemBlk>) {
    let mut files = BinaryHeap::new();
    let mut memblks = BinaryHeap::new();
    let mut pos: usize = 0;
    let mut file_id = 0;
    let mut iter = input.chars();
    while let Some(file_size) = iter.next() {
        let file = File {
            id: file_id,
            pos: pos,
            size: file_size.to_digit(10).unwrap() as usize,
        };
        file_id += 1;
        pos += file.size;
        files.push(file);
        match iter.next() {
            Some(freesize) => {
                if freesize != '0' {
                    let blk = MemBlk {
                        pos,
                        len: freesize.to_digit(10).unwrap() as usize,
                    };
                    pos += blk.len;
                    memblks.push(blk);
                }
            }
            None => break,
        }
    }
    (files, memblks)
}

fn checksum(files: &[File]) -> u64 {
    files.iter().fold(0, |acc, file| {
        acc + (file.pos..(file.pos + file.size))
            .map(|pos| pos * file.id as usize)
            .sum::<usize>() as u64
    })
}

fn defrag(blks: &mut BinaryHeap<MemBlk>) {
    if blks.is_empty() {
        return;
    }
    let mut new_blks = BinaryHeap::new();

    let mut blk = blks.pop().unwrap();

    while let Some(mut next_blk) = blks.pop() {
        if blk.pos + blk.len == next_blk.pos {
            blk.len += next_blk.len;
        } else {
            new_blks.push(blk);
            blk = next_blk;
        }

        if blks.is_empty() {
            new_blks.push(blk);
            break;
        }
    }

    for blk in new_blks {
        blks.push(blk);
    }
    // blks = new_blks;
    // mem::swap(&mut &blks, &mut new_blks);
    // mem::replace(&mut blks, &mut new_blks);

    // blks = new_blks;

    // move new_blks into blks;
    // mem::swap(&mut blks, &mut new_blks);
    // *&mut blks = &mut *new_blks;
}

fn main() {
    let contents = std::fs::read_to_string(std::env::args().nth(1).expect("Missing filename"))
        .expect("Failed to read file");
    let (mut files, mut memblks) = parse(&contents);

    let mut final_files = BinaryHeap::new();

    // Get last file.
    'outer: while let Some(mut file) = files.pop() {
        // println!("File {} {:?}", file.id, file);
        // println!("MemBlks {:?}", memblks);

        // Get first mem block.
        // match memblks.pop() {
        let mut new_blks = BinaryHeap::new();
        while let Some(memblk) = memblks.pop() {
            // Some(memblk) => {
            if memblk.pos >= file.pos {
                final_files.push(file);
                new_blks.push(memblk);
                for blk in new_blks {
                    memblks.push(blk);
                }
                defrag(&mut memblks);
                continue 'outer;
            }

            // If file fits in memory block.
            if file.size <= memblk.len {
                // println!(
                //     "File {} fits in memory block starting at {}",
                //     file.id, memblk.pos
                // );
                let new_blk = MemBlk {
                    pos: memblk.pos + file.size,
                    len: memblk.len - file.size,
                };
                file.pos = memblk.pos;
                final_files.push(file);
                if new_blk.len > 0 {
                    new_blks.push(new_blk);
                }
                for blk in new_blks {
                    memblks.push(blk);
                }
                continue 'outer;
            } else if file.size > memblk.len {
                // println!(
                //     "File {} does not fit in memory block starting at {}",
                //     file.id, memblk.pos
                // );
                new_blks.push(memblk);
            }
        }
        // If we reach here, we have not found a memory block that fits the file.
        final_files.push(file.clone());
        for blk in new_blks {
            memblks.push(blk);
        }
    }

    // println!("Remaining files: {:?}", files);
    // for file in files {
    //     final_files.push(file);
    //     // println!("File {:?}", file);
    // }
    let mut v = final_files.into_sorted_vec();
    // v.sort_by(|a, b| a.pos.cmp(&b.pos));
    // let mut pos = 0;
    // for file in &v {
    //     for i in pos..file.pos {
    //         print!(".");
    //     }
    //     pos = file.pos + file.size;
    //     for i in file.pos..(file.pos + file.size) {
    //         print!("{}", file.id);
    //     }
    // }

    let cs = checksum(&v);
    println!("Checksum: {}", cs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memblk_ordering() {
        let mut heap = BinaryHeap::from([
            MemBlk { pos: 20, len: 10 },
            MemBlk { pos: 0, len: 0 },
            MemBlk { pos: 10, len: 10 },
        ]);

        assert_eq!(heap.pop(), Some(MemBlk { pos: 0, len: 0 }));
        assert_eq!(heap.pop(), Some(MemBlk { pos: 10, len: 10 }));
        assert_eq!(heap.pop(), Some(MemBlk { pos: 20, len: 10 }));
    }

    #[test]
    fn file_ordering() {
        let mut heap = BinaryHeap::from([
            File {
                id: 1,
                pos: 0,
                size: 20,
            },
            File {
                id: 2,
                pos: 20,
                size: 0,
            },
            File {
                id: 3,
                pos: 20,
                size: 10,
            },
        ]);

        assert_eq!(
            heap.pop(),
            Some(File {
                id: 3,
                pos: 20,
                size: 10
            })
        );
        assert_eq!(
            heap.pop(),
            Some(File {
                id: 2,
                pos: 20,
                size: 0
            })
        );
        assert_eq!(
            heap.pop(),
            Some(File {
                id: 1,
                pos: 0,
                size: 20
            })
        );
    }

    #[test]
    fn parse_input() {
        let input = "123450";
        let (files, memblks) = parse(input);
        assert_eq!(
            files.into_sorted_vec(),
            vec![
                File {
                    id: 0,
                    pos: 0,
                    size: 1
                },
                File {
                    id: 1,
                    pos: 3,
                    size: 3
                },
                File {
                    id: 2,
                    pos: 10,
                    size: 5
                },
            ]
        );
        assert_eq!(
            memblks.into_sorted_vec(),
            vec![MemBlk { pos: 6, len: 4 }, MemBlk { pos: 1, len: 2 },]
        );
    }

    #[test]
    fn test_defrag() {
        let mut blks = BinaryHeap::from([
            MemBlk { pos: 10, len: 5 },
            MemBlk { pos: 15, len: 2 },
            MemBlk { pos: 19, len: 4 },
        ]);
        defrag(&mut blks);
        assert_eq!(
            blks.into_sorted_vec(),
            vec![MemBlk { pos: 19, len: 4 }, MemBlk { pos: 10, len: 7 }]
        );
    }
}
