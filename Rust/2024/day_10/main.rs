mod reader;
use reader::get_lines;

macro_rules! testonly_println {
    ($($x:tt)*) => {
        #[cfg(test)]
        println!($($x)*);
    };
}

#[derive(Copy, Clone, Debug)]
struct Trailhead {
    coordinates: Vector,
    score: u8,
}

#[derive(Copy, Clone, Debug)]
struct Vector {
    x: i16,
    y: i16,
}

struct Map {
    grid: [[Cell; GRIDSIZE]; GRIDSIZE],
    size_override: usize,
    trailheads: [Trailhead; HEADCOUNT],
    trailheads_count: usize,
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    height: u8,
    reached_by: u8,
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Vector {
    fn neighbour(&self, direction_index: u8) -> Vector {
        let direction = match direction_index {
            0 => Vector { x: 0, y: 1 },
            1 => Vector { x: 1, y: 0 },
            2 => Vector { x: 0, y: -1 },
            3 => Vector { x: -1, y: 0 },
            _ => panic!("Invalid direction index: {direction_index}"),
        };

        *self + direction
    }

    fn to_grid_coordinates(&self, size_override: usize) -> Option<(usize, usize)> {
        if self.x < 0
            || self.y < 0
            || self.x > size_override as i16
            || self.y > size_override as i16
        {
            return None;
        }
        Some((self.x as usize, self.y as usize))
    }
}

const GRIDSIZE: usize = 48;
const HEADCOUNT: usize = 233;

fn get_map(path: &str) -> Map {
    let Ok(lines) = get_lines(path) else {
        panic!("Data file could not be read!");
    };

    let mut trailheads_count: usize = 0;
    let mut trailheads = [Trailhead {
        coordinates: Vector { x: 0, y: 0 },
        score: 0,
    }; HEADCOUNT];
    let mut grid: [[Cell; GRIDSIZE]; GRIDSIZE] = [[Cell {
        height: 0,
        reached_by: 0,
    }; GRIDSIZE]; GRIDSIZE];

    let mut size_override = 0;

    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            let Some(digit) = char.to_digit(10) else {
                panic!(
                    "Non-digit character was found in the data file at position: x:{}, y:{}",
                    x, y
                );
            };
            grid[x][y] = Cell {
                height: digit as u8,
                reached_by: 0,
            };
            if digit == 0 {
                trailheads[trailheads_count] = Trailhead {
                    coordinates: Vector {
                        x: x as i16,
                        y: y as i16,
                    },
                    score: 0,
                };
                trailheads_count += 1;
            }
        }

        size_override = y;
    }

    testonly_println!(
        "get_map: \nTrailheads_count: {} \nSize_override: {}\n",
        trailheads_count,
        size_override
    );

    Map {
        grid,
        size_override,
        trailheads,
        trailheads_count,
    }
}

fn pathfinder(
    grid: &mut [[Cell; GRIDSIZE]; GRIDSIZE],
    size_override: usize,
    vector: Vector,
    depth: u8,
    id: u8,
) -> u16 {
    let Some((x, y)) = vector.to_grid_coordinates(size_override) else {
        return 0;
    };

    let cell = &mut grid[x][y];
    if cell.height == 9 {
        //return if cell.reached_by == id { 0 } else { 1 };
        if cell.reached_by == id {
            return 0;
        }
        //cell.reached_by = id;
        return 1;
    }

    let mut result: u16 = 0;

    for dir_index in 0..4 {
        let neighbour = vector.neighbour(dir_index);
        let Some((x, y)) = neighbour.to_grid_coordinates(size_override) else {
            continue;
        };
        if grid[x][y].height == depth + 1 {
            result += pathfinder(
                grid,
                size_override,
                vector.neighbour(dir_index),
                depth + 1,
                id,
            );
        }
    }

    result
}

fn calculate(path: &str) -> u64 {
    // Start here:
    let mut map = get_map(path);

    let mut result: u64 = 0;

    for (id, trailhead) in map.trailheads.iter().take(map.trailheads_count).enumerate() {
        let t = pathfinder(
            &mut map.grid,
            map.size_override,
            trailhead.coordinates,
            0,
            id as u8 + 1, // Add one to offset to not get corrupted results due to the default id
                          // being 0
        ) as u64;
        testonly_println!(
            "Ran pathfinder for trailhead: {:?}\n with the result: {}",
            trailhead,
            t
        );
        result += t;
    }

    result
}

fn main() {
    // Start here:
    let result = calculate("data.txt");
    println!("result: {result}");
}

#[test]
fn calculate_test() {
    let result = calculate("testdata.txt");
    assert_eq!(result, 81);
}

/* Sudo code:

Challenge part 1:
For speed write program to check size of data.txt grid.
Use that size to create arrays instead of lists for processing.
Also count the amount of trailheads (0's)

Load the data file into a two dimensional array of integers.
Add the trailheads coordinates to a separate array.

iterate through the trailheads array.

recursive function: (coordinates, depth: integer ) -> integer
    if grid[coordinates] is a 9
        return 1

    int success_count = 0
    for the four coordinates around the current:
        if it contains depth + 1
            call this function with those coordinates and depth + 1
            success_count += result of above function.



Challenge part 2:



*/
