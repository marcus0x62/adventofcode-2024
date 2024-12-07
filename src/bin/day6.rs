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

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    Obstacle,
    Visited(VisitDirection),
}

#[derive(Clone, PartialEq)]
enum Guard {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
struct VisitDirection(u8);

#[derive(Clone)]
struct Board {
    board: Vec<Vec<Cell>>,
    dimensions: (usize, usize),
    guard: (usize, usize),
    guard_direction: Guard,
}

enum BoardStatus {
    Unresolved,
    Exit(usize),
    Loop,
}

impl Board {
    fn new(state: String) -> Self {
        let mut board = vec![];
        let mut guard = (0, 0);
        let mut guard_direction = Guard::Up;

        for (x, line) in state.lines().enumerate() {
            let mut y = 0;

            board.push(
                line.chars()
                    .map(|c| {
                        let cell = match c {
                            '.' => Cell::Empty,
                            '#' => Cell::Obstacle,
                            '^' => {
                                guard = (x, y);
                                guard_direction = Guard::Up;
                                Cell::Empty
                            }
                            'v' => {
                                guard = (x, y);
                                guard_direction = Guard::Down;
                                Cell::Empty
                            }
                            '>' => {
                                guard = (x, y);
                                guard_direction = Guard::Right;
                                Cell::Empty
                            }
                            '<' => {
                                guard = (x, y);
                                guard_direction = Guard::Left;
                                Cell::Empty
                            }
                            _ => panic!("unknown cell value: {x}"),
                        };

                        y += 1;

                        cell
                    })
                    .collect::<Vec<Cell>>(),
            );
        }

        let dimensions = (board.len(), board[0].len());

        Board {
            board,
            guard,
            guard_direction,
            dimensions,
        }
    }

    fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    fn round(&mut self) -> BoardStatus {
        let (guard_x, guard_y) = self.guard;

        match self.guard_direction {
            Guard::Up => {
                if guard_x == 0 {
                    return BoardStatus::Exit(self.visited());
                }

                if let Cell::Visited(mask) = &self.board[guard_x][guard_y] {
                    self.board[guard_x][guard_y] =
                        Cell::Visited(VisitDirection(mask.0 | VISIT_UP.0));
                } else {
                    self.board[guard_x][guard_y] = Cell::Visited(VISIT_UP);
                }

                match &self.board[guard_x - 1][guard_y] {
                    Cell::Visited(mask) if mask.0 & VISIT_UP.0 > 0 => return BoardStatus::Loop,
                    Cell::Visited(_) | Cell::Empty => self.guard.0 = guard_x - 1,
                    Cell::Obstacle => self.guard_direction = Guard::Right,
                }
            }
            Guard::Down => {
                if guard_x == self.dimensions.0 - 1 {
                    return BoardStatus::Exit(self.visited());
                }

                if let Cell::Visited(mask) = &self.board[guard_x][guard_y] {
                    self.board[guard_x][guard_y] =
                        Cell::Visited(VisitDirection(mask.0 | VISIT_DOWN.0));
                } else {
                    self.board[guard_x][guard_y] = Cell::Visited(VISIT_DOWN);
                }

                match &self.board[guard_x + 1][guard_y] {
                    Cell::Visited(mask) if mask.0 & VISIT_DOWN.0 > 0 => return BoardStatus::Loop,
                    Cell::Visited(_) | Cell::Empty => self.guard.0 = guard_x + 1,
                    Cell::Obstacle => self.guard_direction = Guard::Left,
                }
            }
            Guard::Left => {
                if guard_y == 0 {
                    return BoardStatus::Exit(self.visited());
                }

                if let Cell::Visited(mask) = &self.board[guard_x][guard_y] {
                    self.board[guard_x][guard_y] =
                        Cell::Visited(VisitDirection(mask.0 | VISIT_LEFT.0));
                } else {
                    self.board[guard_x][guard_y] = Cell::Visited(VISIT_LEFT);
                }

                match &self.board[guard_x][guard_y - 1] {
                    Cell::Visited(mask) if mask.0 & VISIT_LEFT.0 > 0 => return BoardStatus::Loop,
                    Cell::Visited(_) | Cell::Empty => self.guard.1 = guard_y - 1,
                    Cell::Obstacle => self.guard_direction = Guard::Up,
                }
            }
            Guard::Right => {
                if guard_y == self.dimensions.1 - 1 {
                    return BoardStatus::Exit(self.visited());
                }

                if let Cell::Visited(mask) = &self.board[guard_x][guard_y] {
                    self.board[guard_x][guard_y] =
                        Cell::Visited(VisitDirection(mask.0 | VISIT_RIGHT.0));
                } else {
                    self.board[guard_x][guard_y] = Cell::Visited(VISIT_RIGHT);
                }

                match &self.board[guard_x][guard_y + 1] {
                    Cell::Visited(mask) if mask.0 & VISIT_RIGHT.0 > 0 => return BoardStatus::Loop,
                    Cell::Visited(_) | Cell::Empty => self.guard.1 = guard_y + 1,
                    Cell::Obstacle => self.guard_direction = Guard::Down,
                }
            }
        }

        BoardStatus::Unresolved
    }

    fn is_loop(&mut self, x: usize, y: usize) -> bool {
        if !self.is_empty(x, y) {
            return false;
        }

        self.board[x][y] = Cell::Obstacle;

        loop {
            match self.round() {
                BoardStatus::Loop => return true,
                BoardStatus::Exit(_) => return false,
                _ => {}
            }
        }
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        matches!(self.board[x][y], Cell::Empty | Cell::Visited(_))
    }

    fn visited(&self) -> usize {
        let mut count = 0;

        for row in &self.board {
            for cell in row {
                if let Cell::Visited(_) = cell {
                    count += 1;
                }
            }
        }

        count + 1
    }
}

fn main() -> Result<(), Error> {
    let Some(file) = args().nth(1) else {
        eprintln!("Usage: day6 inputfile");
        exit(1);
    };

    let contents = read_to_string(file)?;

    let mut board = Board::new(contents);
    let loop_base = board.clone();

    loop {
        match board.round() {
            BoardStatus::Unresolved => {}
            BoardStatus::Exit(x) => {
                println!("Visited: {x}");
                break;
            }
            BoardStatus::Loop => {}
        }
    }

    let mut loops = 0;
    let (rows, cols) = loop_base.dimensions();
    for i in 0..rows {
        for j in 0..cols {
            if loop_base.clone().is_loop(i, j) {
                loops += 1;
            }
        }
    }

    println!("Possible loops: {loops}");
    Ok(())
}

const VISIT_UP: VisitDirection = VisitDirection(0x1);
const VISIT_DOWN: VisitDirection = VisitDirection(0x2);
const VISIT_LEFT: VisitDirection = VisitDirection(0x4);
const VISIT_RIGHT: VisitDirection = VisitDirection(0x8);
