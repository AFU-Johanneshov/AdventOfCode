use std::io;

mod operations;
use operations::Operation;
use operations::OperationResult;

use a_star_pathfinder::PathFinder;

mod vector;
use vector::VectorI16;

mod reader;
use reader::get_lines;
mod testing_debug;

#[derive(Debug)]
enum AdventError {
    IoError(io::Error),
}

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Corrupted,
}

impl Tile {
    fn char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Corrupted => '#',
        }
    }
}

const GRIDSIZE: usize = 71;
const DIRECTIONS: [VectorI16; 4] = [
    VectorI16 { x: 1, y: 0 },
    VectorI16 { x: 0, y: 1 },
    VectorI16 { x: -1, y: 0 },
    VectorI16 { x: 0, y: -1 },
];

#[derive(Debug)]
struct Map {
    grid: [[Tile; GRIDSIZE]; GRIDSIZE],
    size_override: usize,
    falling_bytes: Vec<VectorI16>,
    start_location: VectorI16,
    goal_location: VectorI16,
    age: usize,
}

impl Map {
    fn new(size_override: usize, falling_bytes: Vec<VectorI16>) -> Self {
        Map {
            grid: [[Tile::Empty; GRIDSIZE]; GRIDSIZE],
            size_override,
            falling_bytes,
            start_location: VectorI16 { x: 0, y: 0 },
            goal_location: VectorI16 {
                x: size_override as i16 - 1,
                y: size_override as i16 - 1,
            },
            age: 0,
        }
    }

    fn draw(&self) {
        println!("Map:");
        for y in 0..self.size_override {
            for x in 0..self.size_override {
                print!("{}", self.grid[x][y].char())
            }
            println!()
        }
    }

    fn out_of_bounds(&self, location: VectorI16) -> bool {
        location.x < 0
            || location.x as usize >= self.size_override
            || location.y < 0
            || location.y as usize >= self.size_override
    }

    fn step_time(&mut self) -> Result<(), ()> {
        if self.falling_bytes.len() > self.age {
            let location: VectorI16 = self.falling_bytes[self.age];
            self.age += 1;
            self.grid[location.x as usize][location.y as usize] = Tile::Corrupted;
            Ok(())
        } else {
            Err(())
        }
    }

    fn get_neighbours_closure(&self) -> impl Fn(&VectorI16) -> Vec<(VectorI16, u64)> + '_ {
        |location: &VectorI16| {
            let mut neighbours: Vec<(VectorI16, u64)> = Vec::new();

            for direction in DIRECTIONS {
                let next_location = *location + direction;
                if self.out_of_bounds(next_location)
                    || Tile::Corrupted
                        == self.grid[next_location.x as usize][next_location.y as usize]
                {
                    continue;
                }
                neighbours.push((next_location, 1));
            }

            neighbours
        }
    }

    fn get_heuristic_closure(&self) -> impl Fn(&VectorI16) -> u64 + '_ {
        |_location: &VectorI16| 0
    }

    fn get_is_goal_closure(&self) -> impl Fn(&VectorI16) -> bool + '_ {
        //
        |location: &VectorI16| *location == self.goal_location
    }
}

impl VectorI16 {
    fn from_string(string: String) -> Self {
        let mut state = Operation::None;
        let mut value_1 = 0;
        for char in string.chars() {
            let (next_state, potential_result) = state.next(char);
            state = next_state;

            if let Some(OperationResult::Integer(value)) = potential_result {
                value_1 = value;
            }
        }
        let (_, operation_result) = state.collect_operation();
        if let OperationResult::Integer(value_2) = operation_result {
            return VectorI16::from((value_1, value_2));
        }

        panic!(
            "Invalid string format!\nExpected:\nnumber,number\nbut received:\n{}",
            string
        );
    }
}

fn get_map(path: &str, size_override: usize) -> Result<Map, AdventError> {
    let lines = get_lines(path)?;
    let mut falling_bytes: Vec<VectorI16> = Vec::new();
    for line in lines {
        falling_bytes.push(VectorI16::from_string(line));
    }

    Ok(Map::new(size_override, falling_bytes))
}

fn calculate(
    path: &str,
    initial_delay: usize,
    size_override: usize,
) -> Result<VectorI16, AdventError> {
    let mut map = get_map(path, size_override)?;

    // keep this since we know already that a path exists at this time. 1024 for the full data and
    // 12 for the test_data.
    for _i in 0..initial_delay {
        let _ = map.step_time();
    }

    loop {
        let Ok(_) = map.step_time() else {
            panic!("Ran out of falling bytes before the path was blocked!");
        };

        let mut pathfinder = PathFinder::new(map.start_location, map.get_heuristic_closure());
        let result = pathfinder.calculate_path(
            map.get_neighbours_closure(),
            map.get_heuristic_closure(),
            map.get_is_goal_closure(),
        );

        if result.is_err() {
            break;
        };
    }

    Ok(map.falling_bytes[map.age - 1])
}

fn main() {
    let initial_delay = 1024;
    let size_override = 71;
    match calculate("data.txt", initial_delay, size_override) {
        Err(err) => println!("An error occured: {err:?}"),
        Ok(value) => println!("Result is: {},{}", value.x, value.y),
    }
}

#[test]
fn calculate_test() {
    let initial_delay = 12;
    let size_override = 7;
    match calculate("testdata.txt", initial_delay, size_override) {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, VectorI16 { x: 6, y: 1 }),
    }
}

/* Sudo code:

Challenge part 1:

70x70 grid.
list of coordinates which will be blocked in format: nr,nr

It sounds like we are supposed to place the first 1024 blocks and then find the shortest path through.
Use the pathfinder developed earlier.

Test file should be calculated after 12 blocks have fallen.

Challenge part 2:

Since there are more than 1024 values in the data file part two will likely require the the rest.
First guess is that we forgot to account for the falling bytes when calculating the path. So the new path would
require that we don't collide with another byte that fell after the 1024.

If that is the case we can represent tiles with integers instead, where the value represents WHEN that tile became corrupted.
0 means it got corrupted at the first iteration, 42 on the 42 iteration, and so on.
Then the pathfinder get neighbour closure simply compares the cost to reach that tile from the start with the tiles value.
If the tiles value is larger than the cost then that tile is still empty.

Okay, that guess was wrong. The new task is to figure out when a byte falls that blocks the last available path to the goal.
Basically, we calculate the path, if a path is found we step time once and calculate again.
When no path is found we return the last byte fallen.
A slight optimization is to start the simulation from a age of 1024 for the full data, and 12 for the test_data.

*/
