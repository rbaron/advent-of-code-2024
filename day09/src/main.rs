use std::{collections::BinaryHeap, fmt::Binary, mem};

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct File {
    pos: usize,
    id: u32,
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

fn main() {
    let contents = std::fs::read_to_string(std::env::args().nth(1).expect("Missing filename"))
        .expect("Failed to read file");
    let (mut files, mut memblks) = parse(&contents);

    let mut final_files = BinaryHeap::new();

    // Get last file.
    while let Some(mut file) = files.pop() {
        // println!("File {:?}", file);
        // println!("MemBlks {:?}", memblks);

        // Get first mem block.
        match memblks.pop() {
            Some(memblk) => {
                if memblk.pos >= file.pos {
                    final_files.push(file);
                    memblks.push(memblk);
                    continue;
                }
                // If file fits in memory block.
                if file.size <= memblk.len {
                    let new_blk = MemBlk {
                        pos: memblk.pos + file.size,
                        len: memblk.len - file.size,
                    };
                    file.pos = memblk.pos;
                    final_files.push(file);
                    if new_blk.len > 0 {
                        memblks.push(new_blk);
                    }
                } else if file.size > memblk.len {
                    // If file does not fit in memory block.
                    let remaining_file = File {
                        id: file.id,
                        // pos: file.pos + memblk.len,
                        // Trick: we shift the file to the right by the size of the moved chunk.
                        pos: file.pos,
                        size: file.size - memblk.len,
                    };
                    file.size = memblk.len;
                    file.pos = memblk.pos;
                    final_files.push(file);
                    files.push(remaining_file);
                    // final_files.push(file);
                    // memblks.push(memblk);
                }
            }
            None => {
                // panic!("OOM");
                // return final_files;
                break;
            }
        }

        // break;
        // let mut memblk = memblks.pop().expect("Not enough memory");
        // if file.size > memblk.len {
        //     println!("File {} does not fit in memory", file.id);
        //     continue;
        // }
        // println!(
        //     "File {} fits in memory block starting at {}",
        //     file.id, memblk.start
        // );
        // if file.size < memblk.len {
        //     memblk.start += file.size;
        //     memblk.len -= file.size;
        //     memblks.push(memblk);
        // }
    }

    // println!("Remaining files: {:?}", files);
    // for file in files {
    //     final_files.push(file);
    //     // println!("File {:?}", file);
    // }

    let cs = checksum(&final_files.into_sorted_vec());
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
}
