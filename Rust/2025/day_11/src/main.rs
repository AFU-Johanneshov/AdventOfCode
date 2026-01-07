mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 5;
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

We have a list where each line contains one main id and x amount of other id's that is points to.
Id's are made out of three lowercase letters.

Out task is to figure out how many unique paths exists that lead from the id "you" to id "out"

At this time I do not know for sure if any infinite loops exists, but it is very likely. So some
kind of safeguard against it will probably be needed.

One way is to make a pathfinder that goes from you to out, adding to a counter for each path
found.
However I think another more interesting way to do it is to go backwards. This could be done by:

Use a hashmap with ids as keys and store a list of ids that connect to that one together with a
integer representing the possible paths from that id to out. Default is 0 except for out which
must start with 1.

With a recursive function start at id out:
Get the connected ids and paths to out for the current id.
Using a recursive function iterate through all ids connected to the current one.
    if the id has been visited before in this path then continue with the next id.
    Add the paths to out value to the paths to out value of the id.
    Call this function on that id.

Once the recursive function finishes then check the paths to out value of "you" and you should
have your answer.


To ensure no loops occur we can use a shared hashset with ids as keys.
At the start of the recursive function add the current id to the hashset.
And at the end remove it again.
To check if a id has been visited before simply check if the value exists in the hashset.
    (To decrease the amount of uses of the hashset the addition of ids to the hashset could be
     combined with the visited check)
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
