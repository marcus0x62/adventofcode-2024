// MIT License
//
// Copyright (c) 2024 Marcus Butler
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use std::{env::args, fmt, fs::read_to_string, io::Error, process::exit};

#[derive(Clone)]
struct Disk(Vec<Blocks>);

#[derive(Clone, Debug)]
enum Blocks {
    Free,
    Used(u32),
}

impl Disk {
    fn new(state: String) -> Self {
        let mut space = vec![];

        let mut digits = state.trim().chars().map(|x| x.to_digit(10).unwrap());

        let mut idx = 0..;
        loop {
            let Some(id) = idx.next() else {
                panic!("can't generate id");
            };

            let Some(len) = digits.next() else {
                break;
            };

            let free = digits.next().unwrap_or(0);

            for _ in 0..len {
                space.push(Blocks::Used(id));
            }

            for _ in 0..free {
                space.push(Blocks::Free);
            }
        }

        Self(space)
    }

    fn defrag(&mut self) {
        loop {
            if !self.is_fragmented() {
                break;
            }

            let mut next_id = 0;

            for (x, block) in self.0.iter().enumerate().rev() {
                match block {
                    Blocks::Free => continue,
                    Blocks::Used(id) => {
                        next_id = *id;
                        self.0[x] = Blocks::Free;
                    }
                }

                break;
            }

            for (x, block) in self.0.iter().enumerate() {
                if let Blocks::Used(_) = block {
                    continue;
                }

                self.0[x] = Blocks::Used(next_id);
                break;
            }
        }
    }

    fn defrag_pt2(&mut self) {
        let mut ids = vec![];
        for block in self.0.iter().rev() {
            if let Blocks::Used(n) = block {
                if !ids.contains(n) {
                    ids.push(*n);
                }
            }
        }

        for id in ids {
            let len = self.block_len(id);
            match (self.block_range(id), self.first_free_range(len)) {
                (Some(block_range), Some(free_range)) if block_range.0 > free_range.0 => {
                    for i in block_range.0..=block_range.1 {
                        self.0[i] = Blocks::Free;
                    }
                    for i in free_range.0..=free_range.1 {
                        self.0[i] = Blocks::Used(id);
                    }
                }
                _ => {}
            }
        }
    }

    fn block_len(&self, id: u32) -> usize {
        let mut len = 0;
        for block in &self.0 {
            if let Blocks::Used(n) = block {
                if *n == id {
                    len += 1;
                }
            }
        }
        len
    }

    fn block_range(&self, id: u32) -> Option<(usize, usize)> {
        let mut start = false;
        let mut range = (0, 0);
        for (x, block) in self.0.iter().enumerate() {
            if let Blocks::Used(n) = block {
                if !start && *n == id {
                    start = true;
                    range.0 = x;
                    range.1 = x;
                } else if *n == id {
                    range.1 = x;
                }
            }
        }

        if start {
            Some(range)
        } else {
            None
        }
    }

    fn first_free_range(&self, len: usize) -> Option<(usize, usize)> {
        let mut found_free = false;
        let mut found = 0;
        let mut range = (0, 0);

        for (x, block) in self.0.iter().enumerate() {
            if let Blocks::Free = block {
                if !found_free {
                    found_free = true;
                    range.0 = x;
                }
                found += 1;

                if found == len {
                    range.1 = x;
                    return Some(range);
                }
            }

            if let Blocks::Used(_) = block {
                if found_free {
                    found_free = false;
                    found = 0;
                }
            }
        }

        None
    }

    fn is_fragmented(&self) -> bool {
        let mut saw_free = false;
        for block in &self.0 {
            if let Blocks::Free = block {
                saw_free = true;
                continue;
            }

            if let Blocks::Used(_) = block {
                if saw_free {
                    return true;
                }
            }
        }
        false
    }

    fn checksum(&self) -> u64 {
        let mut sum = 0;

        for (i, block) in self.0.iter().enumerate() {
            let Blocks::Used(id) = block else {
                continue;
            };

            sum += *id as u64 * i as u64
        }

        sum
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in &self.0 {
            let _ = match block {
                Blocks::Free => write!(f, "."),
                Blocks::Used(id) => write!(f, "[{id}]"),
            };
        }

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let Some(file) = args().nth(1) else {
        eprintln!("Usage: day9 inputfile");
        exit(1);
    };

    let contents = read_to_string(file)?;

    let mut part1 = Disk::new(contents);
    let mut part2 = part1.clone();

    part1.defrag();
    println!("Part 1 checksum: {}", part1.checksum());

    part2.defrag_pt2();
    println!("Part 2 checksum: {}", part2.checksum());

    Ok(())
}
