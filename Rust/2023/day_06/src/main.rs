#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 288;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 32076;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 71503;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 34278221;

//

//

/*
Part One
##################################################################################################

The datafile has two parts.
The first row holds the duration of each race.
And the second row holds the record distance, I.e. the distance we need to beat.

Our goal is to calculate how many different durations we can hold the button that still wins us
the race.

What I am thinking is we start to figure out the shortest time we need to hold the button that
still results in a win, and then after we calculate the longest possible duration. Since any
duration between those two will also result in a win, we don't need to check them.
So we can figure out how many options we have by getting the difference between the shortest and
longest possible durations.

Then simply multiply all the resulting values together.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    fn get_values(possible_row: Option<String>) -> Result<Vec<u64>, Box<dyn Error>> {
        Ok(possible_row
            .ok_or("Missing data row!")?
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()?)
    }

    fn is_winner(race_time: u64, distance: u64, hold_time: u64) -> bool {
        (race_time - hold_time) * hold_time > distance
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut lines = reader::get_lines(data_path)?;
        let times = get_values(lines.next())?;
        let distances = get_values(lines.next())?;

        let mut result = 1;

        for (race_time, distance) in times.iter().zip(distances) {
            let (mut lower, mut higher) = (0, 0);

            // Find the lowest hold time that results in a new record.
            for hold_time in 1..*race_time {
                if is_winner(*race_time, distance, hold_time) {
                    lower = hold_time;
                    break;
                }
            }

            // Find the highest hold time that results in a new record.
            for hold_time in (1..*race_time).rev() {
                if is_winner(*race_time, distance, hold_time) {
                    higher = hold_time;
                    break;
                }
            }

            result *= higher - lower + 1;
        }

        Ok(result)
    }
}

//

//

/*
Part Two
##################################################################################################

Part two is rather simple actually. The main issue now is just that we need to do the same as part
one, but with one loooong race instead of multiple shorter ones.

The main thing we need to change is how we read the data. According to the instructions we need to
ignore the spaces between the numbers. Instead merging them into a single large number.

Once that is done for both rows we have a standard race with a time and distance.

The method used to solve part one above should be fast enough to solve this larger race too
without any issues.
But, it would be interesting to try and find a different way to figure out the lowest and highest
possible hold time. I feel like it is possible to do using math alone, but I am a bit to tired to
figure that out at the moment.
*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    fn get_value(possible_row: Option<String>) -> Result<u64, Box<dyn Error>> {
        Ok(String::from_iter(
            possible_row
                .ok_or("Missing data row!")?
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty()),
        )
        .parse::<u64>()?)
    }

    fn is_winner(race_time: u64, distance: u64, hold_time: u64) -> bool {
        (race_time - hold_time) * hold_time > distance
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut lines = reader::get_lines(data_path)?;
        let race_time = get_value(lines.next())?;
        let distance = get_value(lines.next())?;

        let (mut lower, mut higher) = (0, 0);

        // Find the lowest hold time that results in a new record.
        for hold_time in 1..race_time {
            if is_winner(race_time, distance, hold_time) {
                lower = hold_time;
                break;
            }
        }

        // Find the highest hold time that results in a new record.
        for hold_time in (1..race_time).rev() {
            if is_winner(race_time, distance, hold_time) {
                higher = hold_time;
                break;
            }
        }

        Ok(higher - lower + 1)
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
