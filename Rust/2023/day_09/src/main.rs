#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 114;
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

Our task here is to predict numbers using a specific method.
Each data line can be processed separately.
Each line contains a set of numbers.

We get the next number using the following method:
Repeat the following until the result row is all zeros or is empty.

For each value in values.skip(1) // Skip the first value since it has nothing to compare too.
Add the result of subtracting the previous value from the new value to a result list.

Once the bottom row has been found:
Add the last value of each row together to get the result.

Once could probably find a way to skip having to calculate all the numbers and only do the last
ones, but atm I don't see the time investment for that to be worth it.

Results:
1900381975 is too high
2474069222 is too high
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    fn worker(numbers: &[i64]) -> i64 {
        let mut differences = Vec::new();
        for i in 1..numbers.len() {
            differences.push(numbers[i] - numbers[i - 1]);
        }

        //println!("diffs: {:?}", differences);

        ({
            if differences.iter().all(|v| *v == 0) {
                0
            } else {
                worker(&differences)
            }
        }) + numbers[numbers.len() - 1]
    }

    fn process_line(data_line: &str) -> Result<i64, Box<dyn Error>> {
        let numbers = data_line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()?;
        let result = worker(&numbers);
        //println!("Result: {result}");
        Ok(result)
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut r = 0i64;
        for value in reader::get_lines(data_path)?.map(|line| process_line(&line)) {
            r += value?;
        }
        println!("r: {r}");
        //Ok(r as u64) // */
        Ok(reader::get_lines(data_path)?
            .map(|line| process_line(&line))
            .sum::<Result<i64, _>>()? as u64) // */
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
