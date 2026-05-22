#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 21;
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

We are meant to calculate the number of possible arrangements of broken tiles for each row of the
data files.

The rows consist of two main parts. The first is made of the following characters:
'.' => Safe tiles
'#' => Broken tiles
'?' => Unknown tiles. (Could be broken or safe)
The second part is a set of numbers representing the amount of broken tiles and the order they are
in. Each number of tiles needs to be separated by at least 1 safe tile.
Now our goal is to use the numbers and the tiles we received to figure out how many possible
arrangements the broken tile order can be in.

This will get complicated.

One thing we might be able to do is to split the problem if we find a broken tile number that can
only ever be in one location. Then we could split the tile sequence before and after those tiles
adding one on both sides as padding. Then we split the numbers at the tile number in question and
calculate the two now separate tile and number sequnces. Once both are calculated multiply the
two results together to get the total potential arrangements.

But to actually get this done we need more than that.

Further planning required...

Looking at the data I noticed some things.
Firstly, just checking the first rows tells me that the majority of the broken tile groups can
actually only be in one location. Hinting again that what we need is to find a way to filter out
any parts that are locked to one position.
And since we are meant to count unique arrangements we can ignore them when counting since they
will never affect the amount of unique arrangements.

The question now is just how to find these "locked" groups of tiles.

Another way could be to try and clear the unknowns and replace them with safe/broken tiles.
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
