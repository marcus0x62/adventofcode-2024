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
use std::{env::args, fs::read_to_string, io::Error, process::exit};

#[derive(Clone)]
struct TrailMap {
    dimensions: (isize, isize),
    map: Vec<Vec<u8>>,
    trailheads: Vec<(isize, isize)>,
}

enum WalkResult {
    Intermediate(Vec<(isize, isize)>),
    Final(Vec<Vec<(isize, isize)>>),
}

impl TrailMap {
    fn new(state: String) -> Self {
        let mut map = vec![];
        let mut trailheads = vec![];

        let lines = state.lines().collect::<Vec<&str>>();
        let dimensions = (lines.len() as isize, lines[0].len() as isize);

        for (x, line) in state.lines().enumerate() {
            let mut row = vec![];
            for (y, char) in line.chars().enumerate() {
                let Some(digit) = char.to_digit(10) else {
                    panic!("Invalid input");
                };
                row.push(digit as u8);

                if digit == 0 {
                    trailheads.push((x as isize, y as isize));
                }
            }
            map.push(row);
        }

        Self {
            map,
            dimensions,
            trailheads,
        }
    }

    fn score(&self, head: (isize, isize), rating: bool) -> usize {
        match self.walk(head, 0, vec![head], vec![]) {
            WalkResult::Intermediate(_) => panic!("Unexpected intermediate result for walk"),
            WalkResult::Final(list) => {
                if rating {
                    list.len()
                } else {
                    list.iter()
                        .filter_map(|x| x.last())
                        .fold(vec![], |mut acc, x| {
                            if !acc.contains(&x) {
                                acc.push(x);
                            }
                            acc
                        })
                        .len()
                }
            }
        }
    }

    fn walk(
        &self,
        pos: (isize, isize),
        level: u8,
        mut path: Vec<(isize, isize)>,
        mut found: Vec<Vec<(isize, isize)>>,
    ) -> WalkResult {
        path.push((pos.0, pos.1));

        if self.map[pos.0 as usize][pos.1 as usize] == 9 {
            return WalkResult::Intermediate(path);
        }

        for step in [
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ] {
            if step.0 >= 0
                && step.0 < self.dimensions.0
                && step.1 >= 0
                && step.1 < self.dimensions.1
                && self.map[step.0 as usize][step.1 as usize] == level + 1
                && !path.contains(&step)
            {
                match self.walk(step, level + 1, path.clone(), found.clone()) {
                    WalkResult::Final(list) => {
                        found = list.iter().fold(found, |mut acc, elem| {
                            if !acc.contains(&elem) {
                                acc.push(elem.clone());
                            }
                            acc
                        });
                    }
                    WalkResult::Intermediate(loc) => found.push(loc),
                }
            }
        }

        WalkResult::Final(found)
    }
}

fn main() -> Result<(), Error> {
    let Some(file) = args().nth(1) else {
        eprintln!("Usage: day10 inputfile");
        exit(1);
    };

    let contents = read_to_string(file)?;

    let map = TrailMap::new(contents);

    let mut score = 0;
    for head in &map.trailheads {
        score += map.score(*head, false);
    }

    println!("Part 1 score: {score}");

    score = 0;
    for head in &map.trailheads {
        score += map.score(*head, true);
    }

    println!("Part 2 score: {score}");

    Ok(())
}
