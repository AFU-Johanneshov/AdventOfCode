mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 7;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 505;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

We have three types of data per line.
First one section showing the desired light pattern [..#.#] where a . is off and # is on.
Then we have x sections showing the effects of a button press. [0,2,3] where the value shows which
light index it is connected to. Pressing the button will toggle all connected lights.
Lastly we have a section of joltage requirements which wont be used in Part One {3,6,4,9}

Our task is to figure out what is the lowest number of button presses required for the lights to
match the desired light pattern.

First though is just to brute force it. Not efficient, but should be good enough for this case.

Our task is to figure out what is the lowest number of button presses required for the lights to
match the desired light pattern.

First thought is just to brute force it. Not efficient, but should be good enough for this case.

Would work a bit like pathfinding.
We would store states in a queue, where the state is a light sequence and steps needed to get
there.

Then just dequeue the oldest state and create new states based on it using all available button
combinations. Compare the result states with the desired pattern. If a match is found return the
amount of steps to get there. If not then add all states back to the queue.
*/
mod part_one {
    use crate::reader;
    use std::{collections::VecDeque, error::Error};

    #[derive(Default, PartialEq, Eq, Debug)]
    struct Lights {
        lights: [bool; 10],
    }

    impl Lights {
        fn from_light_pattern(data: &str) -> Result<Lights, Box<dyn Error>> {
            let trimmed_data = &data[1..data.len() - 1];
            let mut new_lights = Lights::default();
            for (i, char) in trimmed_data.chars().enumerate() {
                new_lights.lights[i] = match char {
                    '#' => true,
                    '.' => false,
                    _ => return Err("Unexpected character in light pattern!".into()),
                };
            }
            Ok(new_lights)
        }

        fn from_button(data: &str) -> Result<Lights, Box<dyn Error>> {
            let trimmed_data = data[1..data.len() - 1].split(',');
            let mut new_lights = Lights::default();
            for i in trimmed_data {
                new_lights.lights[i.parse::<usize>()?] = true;
            }

            Ok(new_lights)
        }

        fn combine(&self, other: &Lights) -> Lights {
            let mut new_lights = Lights::default();
            for i in 0..10 {
                new_lights.lights[i] = self.lights[i] ^ other.lights[i];
            }
            new_lights
        }
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut total_steps = 0;
        for line in reader::get_lines(data_path)? {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let mut parts_iter = parts.iter();
            let parts_len = parts_iter.len();

            let mut buttons = Vec::new();
            let desired_pattern = Lights::from_light_pattern(
                parts_iter
                    .next()
                    .ok_or("Data string [{line}] does not follow the correct format!")?,
            )?;
            for part in parts_iter.take(parts_len - 2) {
                buttons.push(Lights::from_button(part)?);
            }

            let mut processing_queue = VecDeque::new();
            processing_queue.push_back((Lights::default(), 0));
            'outer: while let Some((lights, steps)) = processing_queue.pop_front() {
                for button in &buttons {
                    let new_lights = lights.combine(button);
                    if new_lights == desired_pattern {
                        total_steps += 1 + steps;
                        break 'outer;
                    }
                    processing_queue.push_back((new_lights, steps + 1));
                }
            }
        }

        Ok(total_steps)
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
    print!("\nPart One ");
    match part_one::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("FAILED with error:\n{}", err),
    }
    print!("\nPart Two ");
    match part_two::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("FAILED with error:\n{}", err),
    }
    println!();
}
