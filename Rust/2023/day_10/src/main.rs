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
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 10;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 413;

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
        let map = get_map(data_path)?;

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

Part two gives us a quite different task. Instead of finding the length of the loop, we need to
find how many tiles are INSIDE the loop. Tiles only count as inside if they truly are on the
inside of the loop. Pockets formed by the loop turning invards parallel with itself forming a
small pocket visually inside still count as outside the loop.
One way to view it is that if we traverse the loop edge clockwise, then any tiles to the right is
inside, while any to the left is outside.

One way we could do this is to add a visited field at each tile in the map grid.
Then we iterate through all rows one at a time.
When we find a visited tile we toggle a "inside" bool. If the "inside" bool is true then any tile
we find that is not visited has to be inside the loop. Then all we need to do is to count them by
adding 1 to a total for each we find.
Once all rows has been processed we should have our answer in the total.
*/
mod part_two {
    use crate::reader;
    use core::panic;
    use std::error::Error;

    #[derive(Clone, Copy)]
    enum TileType {
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

    impl TileType {
        fn from_char(c: char) -> Result<TileType, Box<dyn Error>> {
            Ok(match c {
                '|' => TileType::Pipe([true, false, true, false]),
                '-' => TileType::Pipe([false, true, false, true]),
                'L' => TileType::Pipe([true, true, false, false]),
                'J' => TileType::Pipe([true, false, false, true]),
                '7' => TileType::Pipe([false, false, true, true]),
                'F' => TileType::Pipe([false, true, true, false]),
                '.' => TileType::Empty,
                'S' => TileType::Start,
                _ => return Err(format!("Invalid char {c} found in data!").into()),
            })
        }
    }

    #[derive(Clone, Copy)]
    struct Tile {
        kind: TileType,
        visited: bool,
        inside: bool,
    }

    impl Tile {
        fn new(kind: TileType) -> Tile {
            Tile {
                kind,
                visited: false,
                inside: false,
            }
        }
    }

    struct Map {
        grid: [[Tile; 142]; 142],
        start_pos: (usize, usize),
    }

    fn get_map(data_path: &str) -> Result<Map, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;

        let mut grid = [[Tile::new(TileType::Empty); 142]; 142];
        let mut start_pos = (0, 0);
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid[x + 1][y + 1] = Tile::new(TileType::from_char(c)?);
                if c == 'S' {
                    start_pos = (x + 1, y + 1);
                }
            }
        }

        Ok(Map { grid, start_pos })
    }

    fn scan_pipe_loop(map: &mut Map) -> Result<(), Box<dyn Error>> {
        for (dir_index, direction) in DIRECTIONS.iter().enumerate() {
            let mut current_pos = (
                map.start_pos.0 as i32 + direction.0,
                map.start_pos.1 as i32 + direction.1,
            );

            map.grid[current_pos.0 as usize][current_pos.1 as usize].visited = true;

            let mut dir = dir_index;
            loop {
                map.grid[current_pos.0 as usize][current_pos.1 as usize].visited = true;
                map.grid[current_pos.0 as usize][current_pos.1 as usize].inside = false;
                match map.grid[current_pos.0 as usize][current_pos.1 as usize].kind {
                    TileType::Empty => break,
                    TileType::Start => return Ok(()),
                    TileType::Pipe(pipe_connections) => {
                        let opposite = (dir + 2) % 4;
                        if !pipe_connections[opposite] {
                            break;
                        }

                        let old_dir = dir;
                        for (i, connection) in pipe_connections.iter().enumerate() {
                            if i != opposite && *connection {
                                dir = i;
                            }
                        }

                        // Mark the right side of the input pipe as inside
                        let right_side = DIRECTIONS[(old_dir + 1) % 4];
                        let neighbour = &mut map.grid[(current_pos.0 + right_side.0) as usize]
                            [(current_pos.1 + right_side.1) as usize];
                        if !neighbour.visited {
                            neighbour.inside = true;
                        }

                        // Mark the right side of the output pipe as inside
                        let right_side = DIRECTIONS[(dir + 1) % 4];
                        let neighbour = &mut map.grid[(current_pos.0 + right_side.0) as usize]
                            [(current_pos.1 + right_side.1) as usize];
                        if !neighbour.visited {
                            neighbour.inside = true;
                        }

                        current_pos = (
                            current_pos.0 + DIRECTIONS[dir].0,
                            current_pos.1 + DIRECTIONS[dir].1,
                        );
                    }
                }
            }
        }

        Err("No pipe loop found!".into())
    }

    fn inside_is_outside(map: &Map) -> bool {
        for row in map.grid {
            for tile in row {
                if tile.inside {
                    return true;
                } else if tile.visited {
                    return false;
                }
            }
        }
        panic!("No tiles are marked visited OR inside. This should not be possible.");
    }

    // TODO: Warning!
    // None of the data examples I have seen has any invalid paths conneected to S. Meaning we can
    // assume that we wont ever walk any invalid path. Which in turn means we can set the pipes we
    // visit to visited without fear of having to clear the visited status if the pipe we follow
    // reaches a dead end.
    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut map = get_map(data_path)?;

        scan_pipe_loop(&mut map)?;

        let mut tiles_inside = 0;
        if inside_is_outside(&map) {
            println!("Inside is outside!");
            for row in map.grid {
                let mut last_tile = Tile::new(TileType::Empty);
                let mut counting = false;
                for tile in row {
                    if last_tile.visited && !tile.visited && !tile.inside {
                        counting = true;
                        tiles_inside += 1;
                    } else if !last_tile.visited && !last_tile.inside && tile.visited {
                        counting = false;
                    } else if !tile.visited && counting {
                        tiles_inside += 1;
                    }
                    last_tile = tile;
                }
            }
        } else {
            println!("Inside is inside as expected.");
            for row in map.grid {
                //let mut last_tile = Tile::new(TileType::Empty);
                let mut counting = false;
                for tile in row {
                    if tile.visited {
                        counting = false;
                    } else if tile.inside {
                        counting = true;
                        tiles_inside += 1;
                    } else if counting {
                        tiles_inside += 1;
                    }
                }
            }
        }

        Ok(tiles_inside)
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
