#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 8;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 6968;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

Here we have a 2D map of pipes and empty spaces. Pipes are not always connected to other pipes.

Our goal is to find a loop of pipes that starts at the start coordinates marked by S. S is
actually a pipe of unknown shape. So what we need to do is check the 4 tiles surrounding the start
coordinates, then following each pipe untill either a dead end is reached or it returns to the
start coordinates.

One way to handle this is to:

For each direction from start:
    int iterations = 0;
    loop
        Check the next coordinates:
        If the pipe is NOT connected in the current direction or the tile is not a pipe:
            break loop.

        if the tile is the start coordinates:
            return iterations;

        Set the current coordinates to the coordinates of the new pipe.
        set direction to the direction of the pipe that is not opposite to the current.
            (This means we check the two connecting directions of the pipe, and pick the one
             that is not pointing where we came from.)

        add 1 to iterations
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    #[derive(Clone, Copy)]
    enum Tile {
        Empty,
        /// The bool array represents the 4 possible connection directions.
        /// 0 = Up
        /// 1 = Right
        /// 2 = Down
        /// 3 = Left
        Pipe([bool; 4]),
        Start,
    }

    const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    impl Tile {
        fn from_char(c: char) -> Result<Tile, Box<dyn Error>> {
            Ok(match c {
                '|' => Tile::Pipe([true, false, true, false]),
                '-' => Tile::Pipe([false, true, false, true]),
                'L' => Tile::Pipe([true, true, false, false]),
                'J' => Tile::Pipe([true, false, false, true]),
                '7' => Tile::Pipe([false, false, true, true]),
                'F' => Tile::Pipe([false, true, true, false]),
                '.' => Tile::Empty,
                'S' => Tile::Start,
                _ => return Err(format!("Invalid char {c} found in data!").into()),
            })
        }
    }

    struct Map {
        grid: [[Tile; 142]; 142],
        start_pos: (usize, usize),
    }

    fn get_map(data_path: &str) -> Result<Map, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;

        let mut grid = [[Tile::Empty; 142]; 142];
        let mut start_pos = (0, 0);
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid[x + 1][y + 1] = Tile::from_char(c)?;
                if c == 'S' {
                    start_pos = (x + 1, y + 1);
                }
            }
        }

        Ok(Map { grid, start_pos })
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut map = get_map(data_path)?;

        for (dir_index, direction) in DIRECTIONS.iter().enumerate() {
            let mut current_pos = (
                map.start_pos.0 as i32 + direction.0,
                map.start_pos.1 as i32 + direction.1,
            );

            let mut dir = dir_index;
            let mut steps = 1;
            loop {
                match map.grid[current_pos.0 as usize][current_pos.1 as usize] {
                    Tile::Empty => break,
                    Tile::Start => return Ok(steps / 2),
                    Tile::Pipe(pipe_connections) => {
                        let opposite = (dir + 2) % 4;
                        if !pipe_connections[opposite] {
                            break;
                        }
                        for (i, connection) in pipe_connections.iter().enumerate() {
                            if i != opposite && *connection {
                                dir = i;
                            }
                        }

                        current_pos = (
                            current_pos.0 + DIRECTIONS[dir].0,
                            current_pos.1 + DIRECTIONS[dir].1,
                        );
                    }
                }
                steps += 1;
            }
        }

        Err("No result found!".into())
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
