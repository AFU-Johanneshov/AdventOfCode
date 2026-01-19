mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 479;

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

Or maybe we could make "educated" guesses if they fit or not. If estimated correctly it might
be good enough to be correct all the time. Maybe...

0: 7
1: 5
2: 7
3: 6
4: 7
5: 7

161 + 205 + 189 + 180 + 203 + 217 = 1155
39 * 42 = 936

Edit:

:/
I am mad.
This puzzle is extremely difficult and basically impossible unless you have prior knowledge about
advanced programming and computer science stuff. At least if you look at the stupid example it
gives you. I am NEVER trusting the examples to represent the actual problem again.
Never assume that the data follows the same rules as the example data.

Spent so much time trying to figure out wtf it is one should do here since supposedly the
solutions should be possible to made rather simple.

Edit 2:
So since the puzzle is actually really simple the data structures I made are overkill, so quite a
lot can be done to reduce code amount.
To be honest we don't really need neither Shape or Region. Since shapes doesn't matter at all.
Its even more simple than that actually.
We can just skip the shapes entirely, and then just calculate the area of the region, then sum
all the shape counts * 9 together. If the answer is less or equal to the area is it a valid
region.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut result = 0; //                    skip(30) is used to ignore the shape lines.
        for region_line in reader::get_lines(data_path)?.skip(30) {
            let mut nums = region_line
                .split(|c: char| !c.is_ascii_digit()) // Split at any non-numeric char.
                .filter(|part| !part.is_empty()) // Skip if the result is empty.
                .map(|part| part.parse::<u32>().unwrap()); // part is always a number.
            let area = nums.next().ok_or("Invalid data!")? * nums.next().ok_or("Invalid data!")?;
            result += if area >= nums.sum::<u32>() * 9 { 1 } else { 0 }
        }
        Ok(result)
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

    //

    //

    // Debug-Only code
    // -------------------------------------------------------------------------------------------
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    mod debug {
        //#[deprecated(note = "Debug-only method; guard with #[cfg(debug_assertions)]")]
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
