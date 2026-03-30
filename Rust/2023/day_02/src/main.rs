#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 8;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 0;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

Each line in the data represents one game, with a list of turns separated by ';'. Each turn
consists of a group of cubes of different colors.
Our task is to sum the game ids of each game that could have been played with a set amount of
cubes.

We don't actually need to care about the turns here, since we only need to ensure no part of any
turn grabs more cubes of any color than the set amount for said color.

Lets aim to solve it this way:

Create a hashmap with key: String, value: integer.
Insert the max amount allowed for each color. In my case: (red, 12), (green, 13) and (blue, 14).

Read one row of the data file at a time. {data_line}
First split {data_line} at ':'. The first part is {game_id_str} and the second part is {turns}
Then split {turns} at ',' or ';' . Each resulting {part} should be a value and the color name as
    a string.
Split {part} at ' '. The first non-empty part will be the amount value, and the second will be
the color name.
Get the value from the hashmap with key color name. Then compare said value with the amount value
    just found. If the amount value is less or equal then add the id from {game_id_str} to a
    total result value.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;

        Err("NotImplemented: This problem has not been solved yet!".into())
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
