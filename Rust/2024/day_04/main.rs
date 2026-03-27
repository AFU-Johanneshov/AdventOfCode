use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod operations;
use operations::Operation;
use operations::OperationResult;

const GRIDSIZE: usize = 140;

struct XmasGrid {
    grid: [[char; GRIDSIZE]; GRIDSIZE],
}

impl XmasGrid {
    fn new() -> Self {
        let mut grid: [[char; GRIDSIZE]; GRIDSIZE] = [[' '; GRIDSIZE]; GRIDSIZE];

        let Ok(reader) = get_reader("./data.txt") else {
            panic!(); // Panic if file not found. Not viable for production code but for prototying it
                      // is fine.
        };

        for (y, line) in reader.lines().flatten().enumerate() {
            for (x, char) in line.chars().enumerate() {
                grid[x][y] = char;
            }
        }

        XmasGrid { grid }
    }
}

impl<'a> IntoIterator for &'a XmasGrid {
    type Item = XmasGridLineIter<'a>;
    type IntoIter = XmasGridIter<'a>;
    fn into_iter(self) -> XmasGridIter<'a> {
        XmasGridIter {
            xmas_grid: &self,
            iter_status: IterStatus::DiagonallyUp(0, GRIDSIZE - 1, 1, 1),
        }
    }
}

struct XmasGridIter<'a> {
    xmas_grid: &'a XmasGrid,
    iter_status: IterStatus,
}

impl<'a> Iterator for XmasGridIter<'a> {
    type Item = XmasGridLineIter<'a>;

    fn next(&mut self) -> Option<XmasGridLineIter<'a>> {
        if self.iter_status == IterStatus::Done {
            return None;
        }

        let result = match self.iter_status {
            IterStatus::Horizontal(y) => Some(XmasGridLineIter {
                xmas_grid: self.xmas_grid,
                direction: (1, 0),
                next_pos: (0, y),
            }),
            IterStatus::Vertical(x) => Some(XmasGridLineIter {
                xmas_grid: self.xmas_grid,
                direction: (0, 1),
                next_pos: (x, 0),
            }),
            IterStatus::DiagonallyUp(x, y, dx, dy) => Some(XmasGridLineIter {
                xmas_grid: self.xmas_grid,
                direction: (dx, dy),
                next_pos: (x, y),
            }),
            IterStatus::DiagonallyDown(x, y, dx, dy) => Some(XmasGridLineIter {
                xmas_grid: self.xmas_grid,
                direction: (dx, dy),
                next_pos: (x, y),
            }),
            IterStatus::Done => None,
        };

        println!("  NextLine: {:?} ", self.iter_status);
        self.iter_status = self.iter_status.get_next();

        //print!("[{}, {}]", result.next_pos.0, result.next_pos.1);

        result
    }
}

struct XmasGridLineIter<'a> {
    xmas_grid: &'a XmasGrid,
    direction: (i16, i16),
    next_pos: (usize, usize),
}

impl<'a> Iterator for XmasGridLineIter<'a> {
    type Item = (char, (usize, usize));

    fn next(&mut self) -> Option<(char, (usize, usize))> {
        if self.next_pos.0 >= GRIDSIZE || self.next_pos.1 >= GRIDSIZE {
            return None;
        }

        let result = (
            self.xmas_grid.grid[self.next_pos.0][self.next_pos.1],
            (self.next_pos.0, self.next_pos.1),
        );

        self.next_pos = (
            (self.next_pos.0 as i16 + self.direction.0) as usize,
            (self.next_pos.1 as i16 + self.direction.1) as usize,
        );

        Some(result)
    }
}

#[derive(PartialEq, Debug)]
enum IterStatus {
    Horizontal(usize),
    Vertical(usize),
    DiagonallyUp(usize, usize, i16, i16),
    DiagonallyDown(usize, usize, i16, i16),
    Done,
}

impl IterStatus {
    fn get_next(&self) -> Self {
        match self {
            Self::Horizontal(y) => {
                if *y >= GRIDSIZE {
                    println!(" Swiched!");
                    Self::Vertical(0)
                } else {
                    Self::Horizontal(*y + 1)
                }
            }
            Self::Vertical(x) => {
                if *x >= GRIDSIZE {
                    println!(" Swiched!");
                    Self::DiagonallyUp(0, GRIDSIZE - 1, 1, 1)
                } else {
                    Self::Vertical(*x + 1)
                }
            }
            Self::DiagonallyUp(x, y, dx, dy) => {
                if *y > 0 {
                    Self::DiagonallyUp(*x, *y - 1, *dx, *dy)
                } else if *y == 0 && *x < GRIDSIZE {
                    Self::DiagonallyUp(*x + 1, *y, *dx, *dy)
                } else {
                    println!(" Swiched!");
                    Self::DiagonallyDown(0, 0, 1, -1)
                }
            }
            Self::DiagonallyDown(x, y, dx, dy) => {
                if *y < GRIDSIZE - 1 {
                    Self::DiagonallyDown(*x, *y + 1, *dx, *dy)
                } else if *y == GRIDSIZE - 1 && *x < GRIDSIZE {
                    Self::DiagonallyDown(*x + 1, *y, *dx, *dy)
                } else {
                    println!(" Swiched!");
                    Self::Done
                }
            }
            Self::Done => panic!("Attempted to call get_next on IterStatus::Done"),
        }
    }
}

fn get_reader<P>(path: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file))
}

fn main() {
    let mut operation_state: Operation = Operation::None;
    let mut count: u16 = 0;
    let mut i = 0;

    let mut last_pos: (usize, usize) = (0, 0);
    let mut x_center_points: HashMap<(usize, usize), bool> = HashMap::new();

    let xmas_grid: XmasGrid = XmasGrid::new();

    for line in &xmas_grid {
        for char in line {
            i += 1;
            let (next_op, output) = operation_state.next(char.0);
            operation_state = next_op;
            let Some(op_result) = output else {
                last_pos = char.1;
                continue;
            };
            match op_result {
                OperationResult::Success => {
                    let exists = x_center_points.entry(last_pos).or_default();
                    if *exists {
                        count += 1;
                    } else {
                        *exists = true;
                    }
                    last_pos = char.1;
                }
            }
        }
        operation_state = Operation::None;
    }

    println!(
        " Received {} characters out of {}",
        i,
        (GRIDSIZE * GRIDSIZE) * 4
    );
    println!("Xmas occurs {count} times...");
}

/*
                let count: &mut u16 = occurances.entry(*i).or_default();
                *count += 1;

read text file into array of 140x140 characters.
create an iterator which iterates through all characters of the array first horizontally, then vertically, and finally diagonally left and right.

Part two todo:
Match for "MAS" instead of "XMAS"
If a match is found, get the center of that word by moving "back" one step.
Atempt to add those coordinates to a hashmap.
    if they exist already then a X has been made, and the total can be increased by one.



*/
