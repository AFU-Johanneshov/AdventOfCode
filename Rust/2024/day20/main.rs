use std::collections::HashMap;
use std::io;

mod vector;
use vector::VectorI16;

mod reader;
use reader::get_lines;
mod testing_debug;

#[derive(Debug)]
enum AdventError {
    IoError(io::Error),
    CorruptedDataFile,
}

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

const GRIDSIZE: usize = 140;

const DIRECTIONS: [VectorI16; 4] = [
    VectorI16 { x: 1, y: 0 },
    VectorI16 { x: 0, y: 1 },
    VectorI16 { x: -1, y: 0 },
    VectorI16 { x: 0, y: -1 },
];

#[derive(Debug, Clone, Copy)]
struct ValidLocation(VectorI16);

impl ValidLocation {
    fn new(vector: VectorI16) -> Option<Self> {
        if Self::out_of_bounds(vector) {
            return None;
        }
        Some(ValidLocation(vector))
    }

    fn out_of_bounds(location: VectorI16) -> bool {
        location.x >= GRIDSIZE as i16
            || location.x < 0
            || location.y >= GRIDSIZE as i16
            || location.y < 0
    }

    fn extract(&self) -> VectorI16 {
        self.0
    }

    fn steps_to(&self, other: &ValidLocation) -> u32 {
        let other = other.extract();
        (self.0.x.abs_diff(other.x) + self.0.y.abs_diff(other.y)) as u32
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Blocked,
    Empty { time: u32 },
    Start { time: u32 },
    Goal { time: u32 },
}

struct Map {
    grid: [[Tile; GRIDSIZE]; GRIDSIZE],
    start: ValidLocation,
    goal: ValidLocation,
}

fn get_map(path: &str) -> Result<Map, AdventError> {
    let lines = get_lines(path)?;

    let mut grid: [[Tile; GRIDSIZE]; GRIDSIZE] = [[Tile::Blocked; GRIDSIZE]; GRIDSIZE];
    let mut start: ValidLocation = ValidLocation::new(VectorI16::from((0, 0)))
        .expect("0,0 should always be a valid location!");
    let mut goal: ValidLocation = ValidLocation::new(VectorI16::from((0, 0)))
        .expect("0,0 should always be a valid location!");
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => grid[x][y] = Tile::Empty { time: 0 },
                'S' => {
                    start = ValidLocation::new(VectorI16::from((x, y)))
                        .expect("Unexpected invalid location for Start! x:{x}, y:{y}");
                    grid[x][y] = Tile::Start { time: 0 };
                }
                'E' => {
                    goal = ValidLocation::new(VectorI16::from((x, y)))
                        .expect("Unexpected invalid location for Start! x:{x}, y:{y}");
                    grid[x][y] = Tile::Goal { time: 0 }
                }
                '#' => {} // Blocked is the default already.
                _ => return Err(AdventError::CorruptedDataFile),
            }
        }
    }

    Ok(Map { grid, start, goal })
}

fn get_neighbours(current: ValidLocation) -> Vec<ValidLocation> {
    let mut result = Vec::with_capacity(4);
    for dir in DIRECTIONS {
        let new = current.extract() + dir;
        if let Some(valid_location) = ValidLocation::new(new) {
            result.push(valid_location);
        }
    }
    result
}

fn view<'a>(map: &'a [[Tile; GRIDSIZE]; GRIDSIZE], location: &ValidLocation) -> &'a Tile {
    &map[location.0.x as usize][location.0.y as usize]
}

fn edit(map: &mut [[Tile; GRIDSIZE]; GRIDSIZE], location: &ValidLocation, tile: Tile) {
    map[location.0.x as usize][location.0.y as usize] = tile;
}

fn scan_map(map: &mut [[Tile; GRIDSIZE]; GRIDSIZE], current: ValidLocation) {
    let mut cache = vec![current];
    let mut iteration_count: u32 = 1;

    while !cache.is_empty() {
        let mut next = Vec::new();
        for location in &cache {
            for neighbour in get_neighbours(*location) {
                match view(map, &neighbour) {
                    Tile::Empty { time } => {
                        if *time != 0 {
                            continue;
                        }
                        edit(
                            map,
                            &neighbour,
                            Tile::Empty {
                                time: iteration_count,
                            },
                        );
                        next.push(neighbour);
                    }
                    Tile::Start { time } => {
                        if *time != 0 {
                            continue;
                        }
                        edit(
                            map,
                            &neighbour,
                            Tile::Start {
                                time: iteration_count,
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        cache = next;
        iteration_count += 1;
    }
}

fn process_neighbours_with_cheats(
    location: &ValidLocation,
    new_map: &mut [[Tile; GRIDSIZE]; GRIDSIZE],
    existing_map: &[[Tile; GRIDSIZE]; GRIDSIZE],
    iteration_count: u32,
    expected_time: u32,
    cheats: &mut HashMap<u32, u32>,
    next: &mut Vec<ValidLocation>,
) {
    for neighbour in get_neighbours(*location) {
        let Tile::Empty { time } = view(new_map, &neighbour) else {
            continue;
        };
        if *time != 0 {
            continue;
        }
        edit(
            new_map,
            &neighbour,
            Tile::Empty {
                time: iteration_count,
            },
        );
        next.push(neighbour);
        cheat(
            existing_map,
            neighbour,
            iteration_count,
            expected_time,
            cheats,
        );
    }
}

fn scan_map_and_cheat(
    new_map: &mut [[Tile; GRIDSIZE]; GRIDSIZE],
    existing_map: &[[Tile; GRIDSIZE]; GRIDSIZE],
    current: ValidLocation,
    expected_time: u32,
) -> HashMap<u32, u32> {
    let mut cache = vec![current];
    let mut iteration_count: u32 = 1;
    let mut cheats: HashMap<u32, u32> = HashMap::new();

    cheat(existing_map, current, 0, expected_time, &mut cheats);

    while !cache.is_empty() {
        let mut next = Vec::new();
        for location in &cache {
            process_neighbours_with_cheats(
                location,
                new_map,
                existing_map,
                iteration_count,
                expected_time,
                &mut cheats,
                &mut next,
            );
        }
        cache = next;
        iteration_count += 1;
    }

    cheats
}

fn cheat(
    map: &[[Tile; GRIDSIZE]; GRIDSIZE],
    start_point: ValidLocation,
    time_at_cheat_origin: u32,
    expected_time: u32,
    cheats: &mut HashMap<u32, u32>,
) {
    let x_min = (start_point.extract().x - 20).clamp(0, GRIDSIZE as i16);
    let x_max = (start_point.extract().x + 21).clamp(0, GRIDSIZE as i16);

    let y_min = (start_point.extract().y - 20).clamp(0, GRIDSIZE as i16);
    let y_max = (start_point.extract().y + 21).clamp(0, GRIDSIZE as i16);
    // The reason I add 21 and not 20 is because of the way ranges work. In this case the range is
    // between inclusive A and exclusive B. Meaning that if we want to iterate from index 5 to 9 we
    // either have to make B inclusive by using ..=B, or we add one extra to the max value.
    // Here we add one because making it exclusive would require we instead modify the clamp to
    // a max of GRIDSIZE - 1. Which would still work but also require more explaination.

    for y in y_min..y_max {
        for x in x_min..x_max {
            let goal_point =
                ValidLocation::new(VectorI16::from((x, y))).expect("This should never fail.");

            let (Tile::Empty { time } | Tile::Goal { time }) = view(map, &goal_point) else {
                continue;
            };

            let cheat_steps = start_point.steps_to(&goal_point);
            if cheat_steps > 20 {
                continue;
            }

            if time + time_at_cheat_origin + cheat_steps < expected_time {
                *cheats
                    .entry(expected_time - (time + time_at_cheat_origin + cheat_steps))
                    .or_default() += 1;
            }
        }
    }
}

fn calculate(path: &str, minimum_cheat_save: u32) -> Result<u64, AdventError> {
    println!("Working...");

    let mut map = get_map(path)?;
    let mut new_grid = map.grid;

    scan_map(&mut map.grid, map.goal);

    let Tile::Start {
        time: best_default_cost,
    } = view(&map.grid, &map.start)
    else {
        panic!("Start does not exist!");
    };

    let cheats = scan_map_and_cheat(&mut new_grid, &map.grid, map.start, *best_default_cost);

    let mut result: u64 = 0;

    for (cheat_time_save, cheat_count) in cheats.iter() {
        if *cheat_time_save >= minimum_cheat_save {
            testonly_println!(
                "There are {} cheats that save {} picoseconds.",
                *cheat_count,
                *cheat_time_save
            );
            result += *cheat_count as u64;
        }
    }

    Ok(result)
}

fn main() {
    match calculate("data.txt", 100) {
        Err(err) => println!("An error occured: {err:?}"),
        Ok(value) => println!("Result is: {}", value),
    }
}

#[test]
fn calculate_test() {
    match calculate("testdata.txt", 50) {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, 285),
    }
}

/* Sudo code:

Challenge part 1:
// Two obvious options here are to custom make something for this exact scenario, or to use the
// existing pathfinding algorithm I made in a different crate.
//
// 1:
// We should start by using the normal pathfinder to find the shortest path available where x is
// the length.
// Then we need to figure out a way to make the pathfinder continue searching paths until the
// length exceeds x - 100. Meaning find all paths which are shorter than the default time - 100
// picoseconds.
//
// Node structure should be the following:
// Location: VectorI16,
// Time: u64,
// Cheated_at: VectorI16,
//
// Node should impl eq where only location and Cheated_at is used for comparison.
//
// In the get_neighbours closure:
// Each neighbour node Time should be the parent node + 1.
// When checking for potential neighbours, if the neighbour is the goal we instead of returning a
// cost of 1 with the neighbour we return: ((x - 100) - Node.Time) or 1 if result would be less
// than 1.
// This would result in the internal pathfinder shortest path cost being set to x - 100. Meaning it
// would continue to find paths until they are higher than x - 100.
//
// But unfortunately there is issues with this too. Firstly, it would calculate a LOT of redundant
// paths.
//
//
//
// 2:
// It might be more viable to do a custom implementation.
// Have two grids of Tiles where a tile is blocked OR holds a u64 value of the time required to reach that tile
// from a set point.
//
// Use a recursive function to map out the first grid from the goal.
// Then use a modified version of that code to move through the second grid.
// {
//  for neighbours {
//      if neighbour is empty and neighbour.time > new_time
//          if neighbour.time < new_time do nothing
//          else call this function on that tile with self.time + 1
//      else if neighbour is blocked
//
//  }
// }
//

Challenge part 2:

// Okay so part two is basically the same just with longer cheat duration.
// Collision is no longer disabled for 2 picoseconds, but rather 20. Meaning that activating the
// cheat can result in many more different endpoints.

// I think the path to go here is to focus on extending the cheat function. Everything else should
// be fine to keep as is.
// The question is how to go ahead. We could go with a recursive function, or loop based like the
// scan_map function. We know the path the cheat takes doesn't matter. During the cheat any wall is
// treated like empty space anyway, and the cheats are identified by the start and end point.
// Meaning we don't actually need to "pathfind" the cheat itself.
//
// What I am thinking should be the solution is to iterate through all tiles up to 20 steps away.
// Any tile which is empty is then compared the same way as part 1 of this challenge.
// Of course the cheat cost needs to be taken into account here as it is no longer a locked 1.
// Which would be a simple comparison of the start and goal point. The difference between them
// added together would be the cheat cost. start.x - goal.x + start.y - goal.y. Of course the
// differences before added together would need to be absolute.

 973713 is too low
...
1087584 is too high

*/
