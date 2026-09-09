#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 136;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 110677;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

Here we have a 2D grid where each tile can be three different things. Empty, Movable and
Stationary. Our goal is to "tilt" the grid, causing all Movable tiles to move up until hitting
either a Stationary tile or the edge.

I am planning that the way to solve this is to use a 2D array of "Tile" enums, or just some other
value that can represent at least 3 states.
We then load the provided grid into this 2D array.

Finally, we iterate through all rows starting at the top. If a Movable tile is found, keep moving
it upwards untill it either reaches the top of the array, or hits a Tile that is not Empty.
Once we find the highest spot for the Movable tile we take the row number and then count the
amount of rows in the 2D array below it. Then we add that value to a total.
(Since we iterate top to bottom any Movable tile we find when moving upwards is guaranteed to
already be at the highest position posible. So we can treat it as a Stationary tile.)

Once all rows has been processed return the total as the result.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    #[derive(Copy, Clone, PartialEq)]
    enum Tile {
        Empty,
        Stationary,
        Movable,
    }

    struct Grid {
        grid: [[Tile; 100]; 100],
        size: usize,
    }

    fn get_grid(data_path: &str) -> Result<Grid, Box<dyn Error>> {
        let mut grid = [[Tile::Empty; 100]; 100];
        let mut size = 0;
        for (y, line) in reader::get_lines(data_path)?.enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid[y][x] = match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Stationary,
                    'O' => Tile::Movable,
                    _ => return Err(format!("Found unexpected char [{}] in data file!", c).into()),
                }
            }
            size = y + 1;
        }
        Ok(Grid { grid, size })
    }

    fn move_up(grid: &mut Grid, x: usize, mut y: usize) -> u64 {
        grid.grid[y][x] = Tile::Empty;
        loop {
            if y == 0 || grid.grid[y - 1][x] != Tile::Empty {
                break;
            } else {
                y -= 1
            }
        }
        grid.grid[y][x] = Tile::Movable;
        (grid.size - y) as u64
    }

    fn process_grid(mut grid: Grid) -> u64 {
        println!("size: {}", grid.size);
        let mut result = 0;
        for y in 0..grid.size {
            for x in 0..grid.size {
                if grid.grid[y][x] == Tile::Movable {
                    result += move_up(&mut grid, x, y);
                }
            }
        }

        result
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let grid = get_grid(data_path)?;
        Ok(process_grid(grid))
    }
}

//

//

/*
Part Two
##################################################################################################

*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;

        Err("NotImplemented: This problem has not been solved yet!".into())
    }
}

//

//

// Default controller code. Is the same between projects.
// ###############################################################################################

fn main() {
    println!("Running Program...");

    if cfg!(feature = "bench") {
        println!("Benchmarks are enabled!\n");
    }

    println!("\nPart One {}\n", {
        match benchmark!("calculate", { part_one::calculate("data.txt") }) {
            Ok(value) => format!("Result:\n{}", value),
            Err(err) => format!("FAILED with error:\n{}", err),
        }
    });
    println!("\nPart Two {}\n", {
        match benchmark!("calculate", { part_two::calculate("data.txt") }) {
            Ok(value) => format!("Result:\n{}", value),
            Err(err) => format!("FAILED with error:\n{}", err),
        }
    });
}
