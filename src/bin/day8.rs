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
use std::{collections::HashMap, env::args, fmt, fs::read_to_string, io::Error, process::exit};

#[derive(Clone)]
struct Antennas {
    dimensions: (i32, i32),
    antipodes: HashMap<(i32, i32), bool>,
    points: HashMap<(i32, i32), u8>,
}

impl Antennas {
    fn new(state: String) -> Self {
        let mut points = HashMap::new();

        let lines = state.lines().collect::<Vec<&str>>();
        let rows = lines.len();
        let cols = lines[0].len();

        for (x, row) in lines.iter().enumerate() {
            for (y, col) in row.chars().enumerate() {
                if col == '.' {
                    continue;
                } else {
                    points.insert((x as i32, y as i32), col as u8);
                }
            }
        }

        Antennas {
            dimensions: (rows as i32, cols as i32),
            points,
            antipodes: HashMap::new(),
        }
    }

    fn distances(&mut self, part2: bool) {
        let points = self.points.keys();
        for ants in points.clone() {
            let Some(ant_type) = self.points.get(ants) else {
                panic!("Unable to get antenna type for {ants:?}");
            };

            for others in points.clone() {
                if others == ants {
                    continue;
                }
                let Some(other_type) = self.points.get(others) else {
                    panic!("Unable to get other antenna type for {others:?}");
                };

                if other_type == ant_type {
                    let distance = (others.0 - ants.0, others.1 - ants.1);
                    let mut sub = (ants.0 - distance.0, ants.1 - distance.1);

                    loop {
                        if sub.0 >= 0
                            && sub.1 >= 0
                            && sub.0 < self.dimensions.0
                            && sub.1 < self.dimensions.1
                        {
                            self.antipodes.insert(sub, true);
                            sub = (sub.0 - distance.0, sub.1 - distance.1);
                        } else {
                            break;
                        }

                        if !part2 {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn n_antipodes(&self, part2: bool) -> usize {
        if !part2 {
            self.antipodes.keys().len()
        } else {
            let mut antipodes = self.antipodes.keys().collect::<Vec<&(i32, i32)>>();
            antipodes.extend(self.points.keys());
            antipodes.sort();
            antipodes.dedup();
            antipodes.len()
        }
    }
}

impl fmt::Display for Antennas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.dimensions.0 {
            for j in 0..self.dimensions.1 {
                if let Some(point) = self.points.get(&(i, j)) {
                    let _ = write!(f, "{}", *point as char);
                } else if self.antipodes.contains_key(&(i, j)) {
                    let _ = write!(f, "#");
                } else {
                    let _ = write!(f, ".");
                }
            }
            let _ = writeln!(f);
        }

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let Some(file) = args().nth(1) else {
        eprintln!("Usage: day8 inputfile");
        exit(1);
    };

    let contents = read_to_string(file)?;

    let mut part1 = Antennas::new(contents);
    let mut part2 = part1.clone();

    part1.distances(false);
    println!("Part 1: {} antipodes", part1.n_antipodes(false));
    part2.distances(true);
    println!("Part 2: {} antipodes", part2.n_antipodes(true));

    Ok(())
}
