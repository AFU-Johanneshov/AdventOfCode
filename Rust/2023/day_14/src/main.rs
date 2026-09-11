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
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 64;
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

Part two requires that we do similar calculations, but this time not just up but the other three
directions as well.
Basically, we are meant to "rotate" the grid 4 times to see where the blocks end up. Then do the
same math to get the result as Part One.

The problem here is that we are not just going to rotate it once, but rather 1 000 000 000 times.
Now, I am fairly sure that calculating all 1 billion rotations would take too long. So we need to
figure out some way to reduce how many calculations are made.

Regardless, first we need to use the Part One code, but change the move_up function to take in a
direction to move instead.
Once we have that we focus on correctly doing one full rotation.

After that we can start looking into ways to reach 1 billion rotations.

My first question is, will the rotations eventually fall into a pattern where each rotation
results in the same layout as the start of the rotations?
If that is the case then we should be able to just return once we find that the layout between
two rotations haven't changed.

Update:
It seems like I was partially correct. The grid doesn't end in a state where any rotation results
in the same state. But it seems like after a few rotations only a few tiles keep changing.

So what I am thinking is that we keep a dictionary of the identifiers and the rotation nr it was
found at. Once the same identifier is found again we know we have found a loop.
When a loop is found we should be able to use the start index and length of the loop to calculate
which index in the loop would line up with exactly 1000000000.

I am thinking once a loop is found we do first subtract 1000000000 with the start index of the
loop.
Then simply do "remaining % loop_length". The value we get out should be the index in the loop
that lines up with 1000000000. So then we just need to rotate the grid that many times again to
get the grid that should appear at 1000000000.
*/
mod part_two {
    use crate::reader;
    use std::{error::Error, thread, time::Duration};

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

    impl Grid {
        fn out_of_bounds(&self, x: i8, y: i8) -> bool {
            x < 0 || x >= self.size as i8 || y < 0 || y >= self.size as i8
        }

        fn identifier(&self) -> [u128; 100] {
            let mut identifier = [0; 100];
            for (y, row) in self.grid.iter().enumerate() {
                let mut result = 0;
                for (i, tile) in row.iter().enumerate() {
                    let bit = match tile {
                        Tile::Empty => 0,
                        _ => 1,
                    };
                    result += (bit as u128) << i;
                }
                identifier[y] = result;
            }
            identifier
        }
    }

    struct Direction {
        x: i8,
        y: i8,
    }

    impl Default for Direction {
        fn default() -> Self {
            Direction { x: 0, y: -1 }
        }
    }

    impl Direction {
        fn next(self) -> Direction {
            match (self.x, self.y) {
                (0, -1) => Direction { x: -1, y: 0 },
                (-1, 0) => Direction { x: 0, y: 1 },
                (0, 1) => Direction { x: 1, y: 0 },
                (1, 0) => Direction { x: 0, y: -1 },
                _ => panic!("This can only occur if the universe demand it."),
            }
        }

        fn vector(&self) -> (i8, i8) {
            (self.x, self.y)
        }

        fn is_vertical(&self) -> bool {
            self.y != 0
        }

        fn is_negative(&self) -> bool {
            self.x == -1 || self.y == -1
        }
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

    fn try_move_tile(grid: &mut Grid, mut x: usize, mut y: usize, direction: &Direction) {
        if grid.grid[y][x] != Tile::Movable {
            return;
        }

        grid.grid[y][x] = Tile::Empty;
        let (d_x, d_y) = direction.vector();

        loop {
            let (n_x, n_y) = (x as i8 + d_x, y as i8 + d_y);
            if grid.out_of_bounds(n_x, n_y) || grid.grid[n_y as usize][n_x as usize] != Tile::Empty
            {
                break;
            } else {
                x = n_x as usize;
                y = n_y as usize;
            }
        }
        grid.grid[y][x] = Tile::Movable;
    }

    fn tilt(grid: &mut Grid, direction: &Direction) {
        let scan_range: Box<dyn Iterator<Item = _>> = if direction.is_negative() {
            Box::new(0..grid.size)
        } else {
            Box::new((0..grid.size).rev())
        };
        if direction.is_vertical() {
            for y in scan_range {
                for x in 0..grid.size {
                    try_move_tile(grid, x, y, direction);
                }
            }
        } else {
            for x in scan_range {
                for y in 0..grid.size {
                    try_move_tile(grid, x, y, direction);
                }
            }
        }
    }

    fn rotate(grid: &mut Grid) {
        let mut direction = Direction::default();
        for _ in 0..4 {
            tilt(grid, &direction);
            direction = direction.next();
        }
    }

    fn is_identical(ident1: [u128; 100], ident2: [u128; 100]) -> bool {
        !ident1.iter().zip(ident2).any(|(a, b)| *a != b)
    }

    fn print_ident(ident: [u128; 100]) {
        println!("");
        for v in ident {
            print!("{v}");
        }
        println!();
    }

    fn print_grid(grid: &Grid) {
        println!();
        for row in grid.grid {
            for tile in row {
                let c = match tile {
                    Tile::Empty => '.',
                    Tile::Stationary => '#',
                    Tile::Movable => 'O',
                };
                print!("{c}");
            }
            println!();
        }
    }

    fn process_grid(mut grid: Grid) -> u64 {
        let mut identifier = [0; 100];
        for i in 0..1000000000 {
            rotate(&mut grid);
            let new_ident = grid.identifier();
            if is_identical(identifier, new_ident) {
                println!("Ident: {:?}\nNew_Ident: {:?}", identifier, new_ident);
                panic!("Found identical after {i} rotations!");
            }
            //print_ident(new_ident);
            print_grid(&grid);
            identifier = new_ident;
            thread::sleep(Duration::from_millis(100));
        }

        println!("size: {}", grid.size);
        let mut result = 0;
        /*
        for y in 0..grid.size {
            for x in 0..grid.size {
                if grid.grid[y][x] == Tile::Movable {
                    //result += move_up(&mut grid, x, y);
                }
            }
        } */

        result
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let grid = get_grid(data_path)?;
        Ok(process_grid(grid))
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
