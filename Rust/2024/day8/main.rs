use std::collections::HashMap;

mod reader;
use reader::get_lines;

const GRIDSIZE: usize = 50;

#[derive(Copy, Clone, Debug)]
struct Vector {
    x: i16,
    y: i16,
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
    fn out_of_bounds(&self, size_override: usize) -> bool {
        self.x < 0 || self.x >= size_override as i16 || self.y < 0 || self.y >= size_override as i16
    }
}

#[derive(Debug)]
struct Antenna {
    frequency: char,
    coordinates: Vector,
}

fn calculate(path: &str, size_override: usize) -> u32 {
    // Start here:
    let mut world_grid: [[bool; GRIDSIZE]; GRIDSIZE] = [[false; GRIDSIZE]; GRIDSIZE];

    let mut total: u32 = 0;

    let antennas = get_antennas(path);
    for frequency in &antennas {
        for (antenna_nr, antenna) in frequency.1.iter().enumerate() {
            for i in antenna_nr + 1..frequency.1.len() {
                let antenna_2: &Antenna = &frequency.1[i];
                let difference_vector: Vector = antenna_2.coordinates - antenna.coordinates;

                let mut next_antinode: Vector = antenna.coordinates;
                while !next_antinode.out_of_bounds(size_override) {
                    if valid_antinode(&mut world_grid, size_override, next_antinode) {
                        total += 1;
                    }
                    next_antinode = next_antinode - difference_vector;
                    println!("Next: {:?}", next_antinode);
                }

                next_antinode = antenna_2.coordinates;
                while !next_antinode.out_of_bounds(size_override) {
                    if valid_antinode(&mut world_grid, size_override, next_antinode) {
                        total += 1;
                    }
                    next_antinode = next_antinode + difference_vector;
                }
            }
        }
    }

    print_grid(&world_grid, size_override);
    println!();

    total
}

fn valid_antinode(
    world_grid: &mut [[bool; GRIDSIZE]; GRIDSIZE],
    size_override: usize,
    coordinates: Vector,
) -> bool {
    if coordinates.out_of_bounds(size_override) {
        return false;
    }
    if world_grid[coordinates.x as usize][coordinates.y as usize] {
        false
    } else {
        world_grid[coordinates.x as usize][coordinates.y as usize] = true;
        true
    }
}

fn get_antennas(path: &str) -> HashMap<char, Vec<Antenna>> {
    let Ok(lines) = get_lines(path) else {
        panic!("Data file could not be read!");
    };

    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
    let mut point: Vector = Vector { x: 0, y: 0 };

    for line in lines {
        point.x = 0;
        for char in line.chars() {
            if char != '.' {
                antennas.entry(char).or_default().push(Antenna {
                    frequency: char,
                    coordinates: point,
                });
            }
            point.x += 1
        }
        point.y += 1
    }

    antennas
}

fn print_grid(world_grid: &[[bool; GRIDSIZE]; GRIDSIZE], size_override: usize) {
    for y in 0..size_override {
        println!();
        for x in 0..size_override {
            if world_grid[x][y] {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
    println!();
}

fn main() {
    // Start here:
    println!(
        "Total unique antinodes are: {}",
        calculate("./data.txt", 50)
    );
}

#[test]
fn calculate_test() {
    assert_eq!(calculate("./data2.txt", 12), 34);
}

/* Sudo code:

Challenge part 1:

Loading:
Create hashmap of antenna frequencies.
Read the map:
    for each antenna found:
        add the antenna to the hashmap with the frequency as the key.

Processing:
// Take the vector between antenna one and two. One antinode is at the coordinates you get by
// subtracting the vector from the first antenna. The second antinode is at the
// coordinates of the vector plus the coordinates of antenna 2.
// All that is left then is to:
// 1: Ensure the coordinates are within the grid.
// 2: Ensure the coordinates haven't been counted as a antinode yet.
// If both are true then one can be added to the total.


Challenge part 2:



*/
