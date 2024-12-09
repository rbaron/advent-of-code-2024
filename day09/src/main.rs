use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug)]
struct MemBlk {
    pos: usize,
    len: usize,
}

// Sort by start address in descending order.
impl Ord for MemBlk {
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
    // Sort by pos in ascending order.
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

fn part1(contents: &str) -> u64 {
    let (mut files, mut memblks) = parse(&contents);
    let mut final_files = BinaryHeap::new();
    // Get last file.
    while let Some(mut file) = files.pop() {
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
                }
            }
            None => {
                break;
            }
        }
    }
    checksum(final_files.as_slice())
}

fn part2(contents: &str) -> u64 {
    let (mut files, mut memblks) = parse(&contents);
    let mut final_files = BinaryHeap::new();
    // Get last file.
    'outer: while let Some(mut file) = files.pop() {
        let mut new_blks = BinaryHeap::new();
        // Get first free memory block.
        while let Some(memblk) = memblks.pop() {
            if memblk.pos >= file.pos {
                final_files.push(file);
                new_blks.push(memblk);
                for blk in new_blks {
                    memblks.push(blk);
                }
                continue 'outer;
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
                    new_blks.push(new_blk);
                }
                for blk in new_blks {
                    memblks.push(blk);
                }
                continue 'outer;
            } else if file.size > memblk.len {
                new_blks.push(memblk);
            }
        }
        // If we reach here, we have not found a memory block that fits the file.
        final_files.push(file);
        for blk in new_blks {
            memblks.push(blk);
        }
    }

    return checksum(&final_files.as_slice());
}

fn main() {
    let contents = std::fs::read_to_string(std::env::args().nth(1).expect("Missing filename"))
        .expect("Failed to read file");
    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
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
