#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 46;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 0; //6978;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

This is a rather unique challenge, as I actually already completed part one in a previous attempt.
Which means we have access to both the expected value for part one, and the information for the
second puzzle.

We should try and make a solution for part one that will be as compatible with part two as
possible.

And in fact, part one and two both rely on the same logic for "simulating" the beams. The only
difference in part two is that we don't have a set start edge and instead need to check all edges
to find which one results in the most energized tiles. So our goal is to make a "simulate beam"
function.

We can also make this function take a reference to the map instead of cloning it. That way we
don't need to clone it for each simulation in part two.

We want to start by creating a grid where each cell can be 3 different variants.
- Empty
- Mirror
- Splitter

We will also likely want a "Direction" enum to make it easier to change and understand beam
directions.


Each variant also needs a energized_by: u16 value. The reason to have this as a u16 instead of a
bool is to avoid having to clear the beam simulation info from the map. Instead of comparing
true/false we check if the value matches a "beam_id" we received from the caller. This way we
avoid having to clone the original non-modified grid OR manually iterating through the grid
reseting all the values each time we simulate a new beam.
This should also be fairly easy to benchmark later.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    type Result<T> = std::result::Result<T, Box<dyn Error>>;

    pub fn calculate(data_path: &str) -> Result<u64> {
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

    type Result<T> = std::result::Result<T, Box<dyn Error>>;

    pub fn calculate(data_path: &str) -> Result<u64> {
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
