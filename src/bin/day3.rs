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

use regex::Regex;

fn main() -> Result<(), Error> {
    let Some(file) = args().nth(1) else {
        eprintln!("Usage: day3 inputfile");
        exit(1);
    };

    let contents = read_to_string(file)?;

    let re = Regex::new(r"do\(\)|don\'t\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total = 0;
    let mut cond_total = 0;
    let mut multiplying = true;
    for (whole, []) in re.captures_iter(&contents).map(|c| c.extract()) {
        match whole {
            "do()" => multiplying = true,
            "don't()" => multiplying = false,
            _ => {
                let Some((_, [one, two])) = mul_re.captures(whole).map(|c| c.extract()) else {
                    println!("Invalid mul!");
                    exit(1);
                };
                let Ok(int_one) = one.parse::<u32>() else {
                    println!("Invalid 1st int");
                    exit(1);
                };
                let Ok(int_two) = two.parse::<u32>() else {
                    println!("Invalid 2nd int");
                    exit(1);
                };

                total += int_one * int_two;

                if multiplying {
                    cond_total += int_one * int_two;
                }
            }
        }
    }

    println!("Total: {total}");
    println!("Conditional total: {cond_total}");
    Ok(())
}
