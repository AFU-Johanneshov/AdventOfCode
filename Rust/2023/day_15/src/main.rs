#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 1320;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 498538;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 145;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 286278;

//

//

/*
Part One
##################################################################################################

This seems like a fairly simple problem. Read the data line, then for each section do the
following and sum the results:

Split the string at '=' or '-'
Have a mutable integer "r".
Then for each character in the first part of the split string:
    Increase "r" by the ascii code for the character.
    Set "r" to "r" multiplied by 17
    Set "r" to the remainder of dividing "r" by 256. (4 % 256)
When all characters have been processed return "r" as the section result.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    type Result<T> = std::result::Result<T, Box<dyn Error>>;

    pub fn calculate(data_path: &str) -> Result<u64> {
        Ok(reader::get_lines(data_path)?
            .next()
            .ok_or("Data file was empty!")?
            .split(',')
            .map(|s| s.chars().fold(0, |r, c| ((r + c as u64) * 17) % 256))
            .sum())
    }
}

//

//

/*
Part Two
##################################################################################################

This is a lot more complex than part one.

We need to start by again spliting the input string at each ',' to get each step.
But now we also need to get the label, operator and possible focal length number.
This time I feel a manual string scan might be better than to use split.
We basically iterate through the characters, and when we find a '=' or '-' we save the index range
before it as the label.

Next we want to run the hash algorithm from Part One on the label to get the box id for this step.

Then depending on which operator we found we do the following:
'-': Go to the box with the id we received from the hash algortihm.
     If the box contains a lens with the correct label then remove it, moving any lenses after it
     forward to fill the empty space.

'=': Get the last character in the step, which should be a 1 letter integer.
     This is the focal length.
     Once we have the focal length go to the box with the id we got from the hash algorithm.
     Check the lenses in the box, if we find a lens with a matching label then set its focal
     length to the one we got from this step.
     If no match is found then add a new lens with the label and focal length specified in the
     current step to the end of the lens list of the box.

When we have processed all steps we need to calculate and add together the focusing power of all
the lenses.

To calculate the focusing power of a lens we do the following:
Multiply
    (The box the lens is in + 1) *
    (The index of the lens in the lens list + 1) *
    The focal length of the lens.

Once we add all the focusing powers together we have our result.
*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    type Result<T> = std::result::Result<T, Box<dyn Error>>;

    struct Lens {
        label: String,
        focal_length: u8,
    }

    fn hash_algorithm(input: &str) -> usize {
        input.chars().fold(0, |r, c| ((r + c as usize) * 17) % 256)
    }

    fn remove_lens(box_array: &mut [Vec<Lens>; 256], label: &str) {
        let box_id = hash_algorithm(label);
        for i in 0..box_array[box_id].len() {
            if box_array[box_id][i].label == label {
                box_array[box_id].remove(i);
                return;
            }
        }
    }

    fn set_lens(box_array: &mut [Vec<Lens>; 256], label: &str, focal_length: char) {
        let box_id = hash_algorithm(label);
        for i in 0..box_array[box_id].len() {
            if box_array[box_id][i].label == label {
                box_array[box_id][i].focal_length = focal_length as u8 - 48;
                return;
            }
        }
        box_array[box_id].push(Lens {
            label: label.to_string(),
            focal_length: focal_length as u8 - 48,
        });
    }

    fn process_steps<'a>(steps: impl Iterator<Item = &'a str>) -> Result<[Vec<Lens>; 256]> {
        let mut box_array = [const { Vec::new() }; 256];

        for step in steps {
            for (i, c) in step.char_indices() {
                if c == '-' {
                    remove_lens(&mut box_array, &step[0..i]);
                    break;
                } else if c == '=' {
                    // For this to run step.chars() can't be empty. So unwrap will never fail.
                    set_lens(&mut box_array, &step[0..i], step.chars().last().unwrap());
                    break;
                }
            }
        }

        Ok(box_array)
    }

    fn calculate_focus_power(box_array: [Vec<Lens>; 256]) -> u64 {
        let mut total = 0;
        for (box_id, lens_box) in box_array.iter().enumerate() {
            for (lens_index, lens) in lens_box.iter().enumerate() {
                total += (box_id as u64 + 1) * (lens_index as u64 + 1) * lens.focal_length as u64;
            }
        }
        total
    }

    pub fn calculate(data_path: &str) -> Result<u64> {
        let line = reader::get_lines(data_path)?
            .next()
            .ok_or("The data file is empty!")?;
        let steps = line.split(',');

        let box_array = process_steps(steps)?;

        Ok(calculate_focus_power(box_array))
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
