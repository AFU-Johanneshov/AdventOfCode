mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 2;
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

We have a list of shapes that can be flipped and rotated. And another list where each row holds
the size of a grid and how many of each shape needs to be placed in it.
The task is to figure out which row is valid and which is not.
A row is valid if all the shapes required fits inside the grid without overlap.

I could try to brute force it but it won't be very efficient, and I am not even sure if the
calculation time would make it a real option. It is very possible it would take ages.
But at the same time I can't see what other alternative I have at this moment.

And seeing how the grids in the full data can be VERY big I really doubt a brute force solution
will ever work.

So, can the problem be divided into parts?
One issue with that would the the testdata. The grid and shape count is much smaller so proper
testing will be troublesome.

One way could be to try and pre-calculate which combinations of shapes creates a perfect square.
Then use that as a base to place a set of shapes in the most efficient way possible.

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
    print!("Running Program...\n\nPart One ");
    match part_one::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("FAILED with error:\n{}", err),
    }
    print!("\nPart Two ");
    match part_two::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}\n", value),
        Err(err) => println!("FAILED with error:\n{}\n", err),
    }
}
