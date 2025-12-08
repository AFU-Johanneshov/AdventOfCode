use std::error::Error;

mod data_parser;
mod operations;
mod reader;

/*
Part One:

We have a grid of empty spaces '.' and splitters '^'. The goal is to figure out how many
times a splitter is hit.
The beam starts at position 'S' and always travels downwards. If a splitter is hit the
beam stops and two new beams start on both sides of the splitter.

Splitters might split a beam into another beam. In those cases it still only counts as
one beam.

The "obvious" solution here would be to simulate the beam going downwards, and while the
goal could be reached by other methods as well, any other I can think of would be far
more complex.

So to simulate the beam:

Load the data into a grid
Set the tile at location 'S' to a beam

Iterate through the grid line by line until the second last line
    Iterate through each tile of the line
        If the tile is a beam:
            If the tile below the current is a:
            Empty space/Beam:
                Set tile below to a beam
            Splitter:
                Set the tiles next to the one below to beams
                Increase split counter by one
return split count



Part Two:



*/

fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines = reader::get_lines(data_path)?;

    todo!();
}

fn main() {
    match calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
}

#[test]
fn calculate_test() {
    let expected_value = 21;
    match calculate("testdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Program using testdata.txt finished but result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Error occured:\n{}", err),
    }
}

/*
#[test]
fn calculate_small_test() {
    let expected_value = 0;
    match calculate("smalltestdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Program using smalltestdata.txt finished but result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Error occured:\n{}", err),
    }
} // */
