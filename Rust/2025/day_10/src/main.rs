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

#### Improvements after completion...

A smarter more efficient way of doing it would be to track how many button presses each state
takes. For example, pressing button 1 and then button 2 will yield the same result as pressing
button 2 first and then nr 1.

So instead of tracking each idividual path to get to that point we could just track that point
and how many different button presses got us there, not their order.
Add a "buttonpresses" array of u8 values to the queue. The steps value can be left alone.
We can add a hashmap that takes in the lights array as key and holds steps as a value.
Before pushing a new state to the queue check the hashmap if that light pattern has been
reached already with fewer steps.
Although it might be enough to use a hashset, since we should alwyas be taking the shorter
steps first. Meaning if the hashmap contained the light pattern the steps there would always
be the same or less. I.e. the new light pattern will never be added.

Result: Adding a hashset and discarding duplicates improved processing time by a massive
amount. In debug mode execution went from 3.200 seconds to 0.097 seconds.
Release mode went from 0.310 seconds to 0.033 seconds.

We can then
*/
mod part_one {
    use crate::reader;
    use std::{
        collections::{HashSet, VecDeque},
        error::Error,
    };

    #[derive(Default, PartialEq, Eq, Debug, Hash, Clone, Copy)]
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
        /*
        for line in reader::get_lines(data_path)? {
            let mut parts = line.split(' ');

            let desired_pattern = Lights::from_light_pattern(
                parts
                    .next()
                    .ok_or_else(|| format!("Invalid format: {line}"))?,
            )?;

            let buttons = parts
                .map(Lights::from_button)
                .collect::<Result<Vec<_>, _>>()?;
        }*/

        for line in reader::get_lines(data_path)? {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let mut parts_iter = parts.iter();
            let parts_len = parts_iter.len();

            let desired_pattern =
                Lights::from_light_pattern(parts_iter.next().expect("This should never be None"))?;

            let buttons = parts_iter
                .take(parts_len - 2)
                .map(|p| Lights::from_button(p))
                .collect::<Result<Vec<_>, _>>()?;
            // */
            /*
            let mut buttons = Vec::new();
            for part in parts_iter.take(parts_len - 2) {
                buttons.push(Lights::from_button(part)?);
            } // */
            let mut pattern_lookup = HashSet::new();
            let mut processing_queue = VecDeque::from(vec![(Lights::default(), 0)]);
            'outer: while let Some((lights, steps)) = processing_queue.pop_front() {
                for button in &buttons {
                    let new_lights = lights.combine(button);
                    if !pattern_lookup.insert(new_lights) {
                        continue;
                    }
                    if new_lights == desired_pattern {
                        total_steps += 1 + steps;
                        println!(
                            "Result found with {} left in queue at {} steps",
                            processing_queue.len(),
                            steps + 1
                        );
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
