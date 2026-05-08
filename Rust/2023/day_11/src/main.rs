#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 374;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 9177603;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 8410;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

This time we have a 2d map of space. The map contains empty spaces '.' and galaxies '#'. Our goal
is to calculate the sum of all the shortest paths between all galaxies.
However, there is a complication in that any row or column of the map that doesn't have a galaxy
is supposed to double in size.

This makes a solution using fixed arrays less feasible, but not impossible.
One way to do this is to load the map into a 2d array of tiles. Each tile is either empty or a
galaxy. To compensate for the double size we add a size field to the empty variant with a default
value of 1.

Then we make a function that iterates through all rows and columns of the map. Any row or column
found to only contain empty spaces has all its empty spaces size increased by one.

This should allow for a path calculating algorithm to know if a empty space is expanded or not.
However, I am not 100% sure this would actually work, and I would expect that there are flaws that
needs to be fixed.

But there is another way. Having a expanded 2d array is actually not needed here. Since what we
actually want to calculate is just the difference in position between galaxies, with their
positions adjusted for the row and column expansions.
This means we only need the 2d array to figure out which rows and columns are empty. We can then
simply "move" any galaxies that are after a empty row or column by one in either the x or y axis.

Once the galaxy list has been adjusted we just iterate through the list, getting the difference
between each galaxy and finally adding the differences together to get our result.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    struct Map {
        grid: Vec<Vec<bool>>,
    }

    struct Galaxies(Vec<(i32, i32)>);

    fn verify(map: Map) -> Result<Map, Box<dyn Error>> {
        let row_len = map.grid.first().ok_or("Error: Map is empty!")?.len();
        if map.grid.iter().any(|r| r.len() != row_len) {
            return Err("Error: Map row lengths differ!".into());
        }
        Ok(map)
    }

    fn load_map(data_path: &str) -> Result<(Map, Galaxies), Box<dyn Error>> {
        let mut map = Map { grid: Vec::new() };
        let mut galaxies = Vec::new();
        for (y, line) in reader::get_lines(data_path)?.enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                row.push(match c {
                    '.' => false,
                    '#' => {
                        galaxies.push((x as i32, y as i32));
                        true
                    }
                    _ => return Err(format!("Unexpected char {c} in data string!").into()),
                });
            }
            map.grid.push(row);
        }

        Ok((verify(map)?, Galaxies(galaxies)))
    }

    fn move_galaxies(x: i32, y: i32, galaxies: &mut Galaxies) {
        for galaxy in galaxies.0.iter_mut() {
            if galaxy.0 > x {
                galaxy.0 += 1;
            }
            if galaxy.1 > y {
                galaxy.1 += 1;
            }
        }
    }

    fn adjust_galaxy_positions(map: &Map, galaxies: &mut Galaxies) {
        let mut y_offset = 0;
        for (y, row) in map.grid.iter().enumerate() {
            if row.iter().all(|b| !b) {
                move_galaxies(i32::MAX, y as i32 + y_offset, galaxies);
                y_offset += 1;
            }
        }

        let mut x_offset = 0;
        'outer: for x in 0..map.grid[0].len() {
            for row in &map.grid {
                if row[x] {
                    continue 'outer;
                }
            }
            move_galaxies(x as i32 + x_offset, i32::MAX, galaxies);
            x_offset += 1;
        }
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let (map, mut galaxies) = load_map(data_path)?;

        adjust_galaxy_positions(&map, &mut galaxies);

        let mut total = 0;
        for i in 0..galaxies.0.len() - 1 {
            let origin = galaxies.0[i];
            for other in galaxies.0.iter().skip(i) {
                total += ((origin.0 - other.0).abs() + (origin.1 - other.1).abs()) as u64;
            }
        }

        Ok(total)
    }
}

//

//

/*
Part Two
##################################################################################################

How nice. This time it might actually be possible to only make a tiny change to the Part One code
for part two to pass.

Basically the difference in part two is that instead of each empty row/column increasing in size
by 1, they are instead meant to increase by 1 000 000.

Using the math based approach as we did in part one, it should be as simple as to just update the
adjust_galaxy_positions and move_galaxies functions to increase the offset and position by one
million instead of 1.

Unfortunately no example is provided for the 1 000 000 increase, only for 10 and 100.
The test will therefor be set to the expected answer for 100. Once that passes, simply add four
zeros and see if the full data result is correct. If so, then update the test to match the
received value.
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
