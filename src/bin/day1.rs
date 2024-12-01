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
        eprintln!("Usage: day1 inputfile");
        exit(1);
    };

    let contents = read_to_string(file)?;

    let mut list_one = vec![];
    let mut list_two = vec![];

    for line in contents.lines() {
        let numbers = line
            .split_whitespace()
            .filter_map(|candidate| match candidate.parse() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect::<Vec<usize>>();

        if numbers.len() != 2 {
            eprintln!("Malformed input: {numbers:?}");
            exit(1);
        }

        list_one.push(numbers[0]);
        list_two.push(numbers[1]);
    }

    list_one.sort();
    list_two.sort();

    let mut total_distance = 0;

    for (x, y) in list_one.iter().zip(list_two.iter()) {
        if x > y {
            total_distance += x - y;
        } else {
            total_distance += y - x;
        }
    }

    println!("Total distance: {total_distance}");

    let mut similarity = 0;
    for i in list_one.iter() {
        let mut k = 0;
        for j in list_two.iter() {
            if j == i {
                k += 1;
            }
        }
        similarity += i * k;
    }

    println!("Similarity score: {similarity}");

    Ok(())
}
