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
use std::{
    collections::{HashMap, VecDeque},
    env::args,
    fs::read_to_string,
    io::Error,
    process::exit,
};

#[derive(Clone)]
struct Equations {
    answer: u64,
    numbers: VecDeque<u64>,
}

#[derive(Clone, Debug, PartialEq)]
enum Operators {
    Plus,
    Multiply,
    Concat,
    Stub,
}

impl Equations {
    fn new(state: String) -> Vec<Self> {
        let mut list = vec![];

        for line in state.lines() {
            let fields = line.split(':').collect::<Vec<&str>>();

            if fields.len() != 2 {
                panic!("Malformed input line");
            }

            let answer = fields[0].parse().unwrap();
            let numbers = fields[1]
                .split(' ')
                .filter_map(|x| {
                    let Ok(x) = x.parse() else {
                        return None;
                    };
                    Some(x)
                })
                .collect::<Vec<u64>>();

            list.push(Equations {
                answer,
                numbers: numbers.into(),
            });
        }

        list
    }
}

fn valid(equations: Vec<Equations>, operators: &[Operators]) -> Vec<u64> {
    let mut valid = vec![];

    let mut cached_tables = HashMap::<usize, Vec<Vec<Operators>>>::new();

    for equation in equations {
        if equation.numbers.is_empty() {
            continue;
        }

        if equation.numbers.len() == 1 {
            if equation.numbers[0] == equation.answer {
                valid.push(equation.answer);
            } else {
                continue;
            }
        }

        let table_size = equation.numbers.len() - 1;
        cached_tables
            .entry(table_size)
            .or_insert_with(|| oper_table(table_size, operators));

        let Some(table) = cached_tables.get(&table_size) else {
            panic!("couldn't get cached table...");
        };

        for row in table {
            let mut numbers = equation.numbers.clone();

            let Some(mut total) = numbers.pop_front() else {
                panic!("can't get first number");
            };

            for oper in row {
                let lhs = total;

                let Some(rhs) = numbers.pop_front() else {
                    panic!("can't get rhs");
                };

                total = match oper {
                    Operators::Concat => {
                        let Ok(num) = format!("{lhs}{rhs}").parse() else {
                            break;
                        };

                        num
                    }
                    Operators::Multiply => {
                        let Some(value) = lhs.checked_mul(rhs) else {
                            break;
                        };

                        value
                    }
                    Operators::Plus => {
                        let Some(value) = lhs.checked_add(rhs) else {
                            break;
                        };

                        value
                    }
                    Operators::Stub => total,
                };
            }

            if total == equation.answer {
                valid.push(equation.answer);
                break;
            }
        }
    }

    valid
}

fn oper_table(n: usize, operators: &[Operators]) -> Vec<Vec<Operators>> {
    let mut table = vec![];

    for i in 0..4usize.pow(n as u32) {
        let mut row = vec![];
        for j in 0..n {
            let val = (i >> (j * 2)) & 3;
            match val {
                0 => row.push(Operators::Plus),
                1 => row.push(Operators::Multiply),
                2 => row.push(Operators::Concat),
                3 => row.push(Operators::Stub),
                _ => {}
            }
        }

        let mut valid = true;
        for elem in &row {
            if !operators.contains(elem) {
                valid = false;
            }
        }

        if valid {
            table.push(row);
        }
    }

    table
}

fn main() -> Result<(), Error> {
    let Some(file) = args().nth(1) else {
        eprintln!("Usage: day7 inputfile");
        exit(1);
    };

    let contents = read_to_string(file)?;

    let board = Equations::new(contents);

    let total = valid(board.clone(), &[Operators::Plus, Operators::Multiply])
        .iter()
        .sum::<u64>();
    println!("Part 1 Total: {total}");

    let total = valid(
        board,
        &[Operators::Plus, Operators::Multiply, Operators::Concat],
    )
    .iter()
    .sum::<u64>();
    println!("Part 2 Total: {total}");

    Ok(())
}
