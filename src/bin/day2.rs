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
        eprintln!("Usage: day2 inputfile");
        exit(1);
    };

    let contents = read_to_string(file)?;

    let mut safe_levels = 0;
    let mut dampener_levels = 0;

    for line in contents.lines() {
        let numbers = line
            .split_whitespace()
            .filter_map(|candidate| match candidate.parse() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect::<Vec<i32>>();

        if is_safe(&numbers) {
            safe_levels += 1;
        } else {
            for i in 0..numbers.len() {
                let mut tmp = numbers.clone();
                tmp.remove(i);

                if is_safe(&tmp) {
                    dampener_levels += 1;
                    break;
                }
            }
        }
    }

    println!("Total safe levels: {safe_levels}");
    println!("Total dampened levels: {}", safe_levels + dampener_levels);

    Ok(())
}

fn is_safe(numbers: &[i32]) -> bool {
    let (mut inc, mut dec, mut same, mut toomuch) = (false, false, false, false);

    for i in 1..numbers.len() {
        match (numbers[i], numbers[i - 1]) {
            _ if numbers[i] > numbers[i - 1] => inc = true,
            _ if numbers[i] < numbers[i - 1] => dec = true,
            _ => same = true,
        }

        if (numbers[i] - numbers[i - 1]).abs() > 3 {
            toomuch = true;
        }
    }

    matches!(
        (inc, dec, same, toomuch),
        (true, false, false, false) | (false, true, false, false)
    )
}
