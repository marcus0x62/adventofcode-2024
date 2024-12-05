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
use {std::env::args, std::fs::read_to_string, std::io::Error, std::process::exit};

fn main() -> Result<(), Error> {
    let Some(file) = args().nth(1) else {
        eprintln!("Usage: day5 inputfile");
        exit(1);
    };

    let contents = read_to_string(file)?;
    let contents = contents.split("\n\n").collect::<Vec<&str>>();

    let (part1, part2) = (contents[0], contents[1]);

    let mut rules = vec![];
    for line in part1.lines() {
        let fields = line.split('|').collect::<Vec<&str>>();

        rules.push((fields[0], fields[1]));
    }

    let mut mid_total = 0;

    let mut incorrect_pages = vec![];

    for update in part2.lines() {
        let pages = update.split(',').collect::<Vec<&str>>();

        let mut correct = true;
        for rule in rules.iter() {
            let (before, after) = rule;

            if let Some(index) = find(&pages, after) {
                if find(&pages, before).is_some() && !pages[0..=index].contains(before) {
                    correct = false;
                }
            }
        }

        let mid_idx = pages.len() / 2;
        if correct {
            mid_total += pages[mid_idx].parse::<u32>().unwrap();
        } else {
            incorrect_pages.push(update);
        }
    }

    println!("Part 1 Total: {mid_total}");

    let mut corrected_total = 0;
    for update in incorrect_pages {
        let mut pages = update.split(',').collect::<Vec<&str>>();

        let mut i = 0;
        loop {
            for (before, after) in rules.iter() {
                let Some(after_index) = find(&pages, after) else {
                    continue;
                };

                let Some(before_index) = find(&pages, before) else {
                    continue;
                };

                if !pages[0..=after_index].contains(before) {
                    (pages[before_index], pages[after_index]) =
                        (pages[after_index], pages[before_index]);
                    i = 0;
                }
            }

            if i == pages.len() {
                break;
            }

            i += 1;
        }

        let mid_idx = pages.len() / 2;
        corrected_total += pages[mid_idx].parse::<u32>().unwrap();
    }

    println!("Corrected total: {corrected_total}");
    Ok(())
}

fn find(haystack: &Vec<&str>, needle: &str) -> Option<usize> {
    haystack.iter().position(|x| *x == needle)
}
