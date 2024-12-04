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
        eprintln!("Usage: day4 inputfile");
        exit(1);
    };

    let contents = read_to_string(file)?;

    let mut table = vec![];
    for line in contents.lines() {
        table.push(line.chars().collect::<Vec<char>>());
    }

    let rows = table.len();
    let columns = table[0].len();

    let mut total = 0;
    let mut p2_total = 0;

    let check =
        |a: &char, b: &char, c: &char, d: &char| *a == 'X' && *b == 'M' && *c == 'A' && *d == 'S';
    let p2_check = |first: &[char; 3], second: &[char; 3], block: &[&[char]; 3]| {
        block[0][0] == first[0]
            && block[1][1] == first[1]
            && block[2][2] == first[2]
            && block[0][2] == second[0]
            && block[1][1] == second[1]
            && block[2][0] == second[2]
    };

    for i in 0..rows {
        for j in 0..columns {
            // across, forwards
            if j < columns - 3
                && check(
                    &table[i][j],
                    &table[i][j + 1],
                    &table[i][j + 2],
                    &table[i][j + 3],
                )
            {
                total += 1;
            }

            // across, backwards
            if j < columns - 3
                && check(
                    &table[i][j + 3],
                    &table[i][j + 2],
                    &table[i][j + 1],
                    &table[i][j],
                )
            {
                total += 1;
            }

            // lateral, forwards
            if i < rows - 3
                && check(
                    &table[i][j],
                    &table[i + 1][j],
                    &table[i + 2][j],
                    &table[i + 3][j],
                )
            {
                total += 1;
            }

            // lateral, backwards
            if i < rows - 3
                && check(
                    &table[i + 3][j],
                    &table[i + 2][j],
                    &table[i + 1][j],
                    &table[i][j],
                )
            {
                total += 1;
            }

            if i < rows - 3 && j < columns - 3 {
                // diagonal, l-r
                if check(
                    // top-bottom
                    &table[i][j],
                    &table[i + 1][j + 1],
                    &table[i + 2][j + 2],
                    &table[i + 3][j + 3],
                ) || check(
                    // bottom-top
                    &table[i + 3][j + 3],
                    &table[i + 2][j + 2],
                    &table[i + 1][j + 1],
                    &table[i][j],
                ) {
                    total += 1;
                }

                // diagonal, r-l
                if check(
                    // top-bottom
                    &table[i][j + 3],
                    &table[i + 1][j + 2],
                    &table[i + 2][j + 1],
                    &table[i + 3][j],
                ) || check(
                    // bottom-top
                    &table[i + 3][j],
                    &table[i + 2][j + 1],
                    &table[i + 1][j + 2],
                    &table[i][j + 3],
                ) {
                    total += 1;
                }
            }

            if i < rows - 2 && j < columns - 2 {
                let matrix = &[
                    &table[i][j..j + 3],
                    &table[i + 1][j..j + 3],
                    &table[i + 2][j..j + 3],
                ];

                if p2_check(&['M', 'A', 'S'], &['M', 'A', 'S'], matrix)
                    || p2_check(&['S', 'A', 'M'], &['S', 'A', 'M'], matrix)
                    || p2_check(&['M', 'A', 'S'], &['S', 'A', 'M'], matrix)
                    || p2_check(&['S', 'A', 'M'], &['M', 'A', 'S'], matrix)
                {
                    p2_total += 1;
                }
            }
        }
    }

    println!("Total: {total}");
    println!("Part two total: {p2_total}");

    Ok(())
}
