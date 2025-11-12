use std::io;
use std::{thread, time};

mod operations;
use operations::Operation;
use operations::OperationResult;

mod reader;
use reader::get_lines;
mod testing_debug;

mod vector;
use vector::VectorI16;

#[derive(Debug)]
enum AdventError {
    IoError(io::Error),
}

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

const MAXROBOTCOUNT: usize = 500;
const GRIDSIZEX: usize = 101;
const GRIDSIZEY: usize = 103;

#[derive(Debug, Default, Copy, Clone)]
struct Robot {
    position: VectorI16,
    velocity: VectorI16,
}

impl Robot {
    fn get_quadrant(&self) -> u8 {
        todo!();
    }

    fn apply_velocity_seconds(&mut self, seconds: i16) {
        self.position = self.position + (self.velocity * seconds);
    }

    fn constrain_to_map(&mut self, map: &Map) {
        /*let mut x = self.position.x % map.world_size_x as i16;
        let mut y = self.position.y % map.world_size_y as i16;
        if x < 0 {
            x += map.world_size_x as i16;
        }
        if y < 0 {
            y += map.world_size_y as i16;
        }*/

        self.position = VectorI16 {
            x: wp(self.position.x, map.world_size_x as i16),
            y: wp(self.position.y, map.world_size_y as i16),
        }
    }
}

fn wp(p: i16, s: i16) -> i16 {
    ((p % s) + s) % s
}

impl From<[i16; 4]> for Robot {
    fn from(values: [i16; 4]) -> Self {
        Robot {
            position: VectorI16::from((values[0], values[1])),
            velocity: VectorI16::from((values[2], values[3])),
        }
    }
}

struct Map {
    robots: [Robot; MAXROBOTCOUNT],
    robot_count: usize,
    world_size_x: usize,
    world_size_y: usize,
}

impl Map {
    fn display(&self) {
        let mut grid: [[bool; GRIDSIZEY]; GRIDSIZEX] = [[false; GRIDSIZEY]; GRIDSIZEX];
        for i in 0..self.robot_count {
            let robot = self.robots[i];
            grid[robot.position.x as usize][robot.position.y as usize] = true;
        }

        for line in grid {
            for point in line {
                let c = if point { 'X' } else { ' ' };
                print!("{}", c);
            }
            print!("\n");
        }
    }
    fn robots_in_quadrants(&self) -> [u64; 4] {
        //let t = self.world_size_y as f64 / 2.0;

        let mut quadrant_robots: [u64; 4] = [0; 4];

        let center_x = self.world_size_x / 2;

        let q0_max_x = center_x - 1;
        let q1_min_x = if self.world_size_x % 2 != 0 {
            center_x + 1
        } else {
            center_x + 2
        };
        testonly_println!("q0_max_x: {}, q1_min_x: {}", q0_max_x, q1_min_x);

        let center_y = self.world_size_y / 2;

        let q2_max_y = center_y - 1;
        let q3_min_y = if self.world_size_y % 2 != 0 {
            center_y + 1
        } else {
            center_y + 2
        };
        testonly_println!("q2_max_y: {}, q3_min_y: {}", q2_max_y, q3_min_y);

        for robot_index in 0..self.robot_count {
            let mut robot = self.robots[robot_index];
            testonly_print!("\nRobot at: {}.{} ", robot.position.x, robot.position.y);
            robot.constrain_to_map(self);
            testonly_print!("Constrained to: {}.{} ", robot.position.x, robot.position.y);

            let x = match robot.position.x as usize {
                value if value <= q0_max_x => 0,
                value if value >= q1_min_x => 1,
                _ => {
                    continue;
                }
            };
            let y = match robot.position.y as usize {
                value if value <= q2_max_y => 0,
                value if value >= q3_min_y => 2,
                _ => {
                    continue;
                }
            };
            testonly_print!("Is in quadrant: {}", x + y);
            quadrant_robots[x + y] += 1;
        }

        quadrant_robots
    }
}

fn get_map(path: &str, size_override: (usize, usize)) -> Result<Map, AdventError> {
    let lines = get_lines(path)?;

    let mut robots: [Robot; MAXROBOTCOUNT] = [Robot::default(); MAXROBOTCOUNT];
    let mut cache: [i16; 4] = [0; 4];
    let mut cache_index: usize = 0;
    let mut line_index: usize = 0;
    let mut operation: Operation = Operation::None;
    for line in lines {
        for char in line.chars() {
            //
            let (next_op, potential_result) = operation.next(char);
            operation = next_op;

            let Some(result) = potential_result else {
                continue;
            };

            let OperationResult::Integer(value) = result else {
                testonly_print!(" {:?} ", result);
                continue;
            };

            cache[cache_index] = value as i16;
            cache_index = (cache_index + 1) % 4;
        }

        let (next_op, result) = operation.collect_operation();
        operation = next_op;

        let OperationResult::Integer(value) = result else {
            continue;
        };

        cache[cache_index] = value as i16;
        cache_index = (cache_index + 1) % 4;

        testonly_println!(
            "Robot: {}.{} with velocity {}.{}",
            cache[0],
            cache[1],
            cache[2],
            cache[3]
        );
        robots[line_index] = Robot::from(cache);
        line_index += 1;
    }

    Ok(Map {
        robots,
        robot_count: line_index,
        world_size_x: size_override.0,
        world_size_y: size_override.1,
    })
}

fn calculate(path: &str, size_override: (usize, usize)) -> Result<u64, AdventError> {
    let mut map = get_map(path, size_override)?;

    //map.display();

    // Since there was no indication of how the christmas tree image was supposed to look like, I
    // felt it was easiest to figure out a potential pattern and search that by printing the map.

    // A strange pattern appears at:
    // 4 107 210 313 ...
    // With 103 added between each one.
    // Running this results in moving 103 seconds ahead, printing the map each time. With a
    // starting time at 4 seconds.
    // After some iterations a christmas tree was found.
    let t = 4;
    for i in 0..map.robot_count {
        for _ in 0..t {
            let mut robot = map.robots[i];
            robot.apply_velocity_seconds(1);
            robot.constrain_to_map(&map);
            map.robots[i] = robot;
        }
    }
    map.display();
    println!("Seconds: {}", t);
    println!("------------------------------------------------------------------------------------------------------");
    thread::sleep(time::Duration::from_millis(1000));

    for seconds in 1..3000 {
        for i in 0..map.robot_count {
            let mut robot = map.robots[i];
            robot.apply_velocity_seconds(103);
            robot.constrain_to_map(&map);
            map.robots[i] = robot;
        }

        map.display();
        println!("Seconds: {}", t + (seconds * 103));
        println!("------------------------------------------------------------------------------------------------------");
        thread::sleep(time::Duration::from_millis(500));
    }

    let quadrants = map.robots_in_quadrants();
    println!("q: {:?}", quadrants);

    Ok(quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3])
}

fn main() {
    match calculate("data.txt", (GRIDSIZEX, GRIDSIZEY)) {
        Err(err) => println!("An error occured: {err:?}"),
        Ok(value) => println!("Result is: {}", value),
    }
}

// Test disabled due to the nature of part 2.
//#[test]
fn calculate_test() {
    match calculate("testdata.txt", (11, 7)) {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, 12),
    }
}

/* Sudo code:

Challenge part 1:



Challenge part 2:



*/
