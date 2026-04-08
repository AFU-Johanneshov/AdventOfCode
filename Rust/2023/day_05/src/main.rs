#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 35;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 403695602;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 46;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

Okay so we essentially have a repeating pattern that we need to handle.

The first row of the data file contains all the seeds, with the following rows following a set
pattern. Each part starts with a row containing the word "map". Then the following rows each
contain 3 values where the: 1st value = Destination range start, 2nd value = Source range start
and the 3rd value = range length.

Each line represents how to transform values within a set range. Lets say we have the row:
"42 64 8"
This means that any value between 64 and 64+8 needs to be shifted down to the start point of 42.
To do this we can subtract the source range start with the destination range start. Giving us the
difference. 64 - 42 = 22
We then use difference to shift the matching number to the destination range.
Lets say we have the value 68. 68 is within the range of 64 and 64+8. Then we subtract the
difference and return the value. 68 - 22 = 46.

If we create a "range" struct that does the above calculation then we can create a "map" struct
that contains a list of these ranges. Then we simply check each range in the map if the value
matches to get the transformed value. And if there are no matches we simply return the value
unchanged.

Then just chain these together according to the data file.
*/
mod part_one {
    use crate::reader;
    use std::{error::Error, str::FromStr};

    struct Range {
        shift_distance: i64,
        lower: u64,
        upper: u64,
    }

    impl Range {
        fn from_row(row: &str) -> Result<Range, Box<dyn Error>> {
            let values: Vec<u64> = parse_values(row)?;
            if values.len() != 3 {
                return Err(format!("Row [{row}] does not have exactly 3 values!").into());
            }

            let lower = values[1];
            let upper = values[1] + values[2];
            let shift_distance = values[1] as i64 - values[0] as i64;
            Ok(Range {
                shift_distance,
                lower,
                upper,
            })
        }

        fn attempt_transform(&self, value: u64) -> Option<u64> {
            if value < self.lower || value > self.upper {
                return None;
            }

            Some((value as i64 - self.shift_distance) as u64)
        }
    }

    #[derive(Default)]
    struct Map {
        ranges: Vec<Range>,
    }

    impl Map {
        fn read_next_map(lines: &mut dyn Iterator<Item = String>) -> Result<Map, Box<dyn Error>> {
            let _map_name_line = lines.next();

            let mut map = Map::default();
            for line in lines {
                if line.is_empty() {
                    break;
                }

                map.ranges.push(Range::from_row(&line)?);
            }

            Ok(map)
        }

        fn transform_value(&self, value: u64) -> u64 {
            for range in &self.ranges {
                if let Some(new_value) = range.attempt_transform(value) {
                    return new_value;
                }
            }
            value
        }
    }

    fn parse_values<T: FromStr>(row: &str) -> Result<Vec<T>, Box<dyn Error>>
    where
        <T as FromStr>::Err: std::error::Error,
        <T as FromStr>::Err: 'static,
    {
        Ok(row
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<T>())
            .collect::<Result<Vec<T>, _>>()?)
    }

    fn get_seeds(possible_row: Option<String>) -> Result<Vec<u64>, Box<dyn Error>> {
        parse_values(&possible_row.ok_or("Missing seeds row!")?)
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut lines = reader::get_lines(data_path)?;

        let seeds = get_seeds(lines.next())?;

        lines.next(); // Skip empty row after seeds line.

        let mut maps = Vec::new();
        for _ in 0..7 {
            maps.push(Map::read_next_map(&mut lines)?);
        }

        let mut highest = u64::MAX;
        for seed in seeds {
            let mut value = seed;
            for map in &maps {
                value = map.transform_value(value);
            }
            highest = highest.min(value);
        }

        Ok(highest)
    }
}

//

//

/*
Part Two
##################################################################################################

This is the classing part two puzzle. Basically do the same thing but with a LOT more values.
It is essentially a test of scalability. Is the code scalable enough to get the result when the
data amount is so much greater, or are major redesigns required?

In this case the main difference is that the seeds row doesn't contain singular seeds, but rather
seed ranges. The row consists of value pairs. Where the first value is the range start, and the
second value is the range length.

If the code is good enough then we should be able to simply copy part one and just make some
changes to the get_seeds() function.
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
