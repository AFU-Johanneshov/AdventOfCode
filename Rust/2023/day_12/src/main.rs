#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 21;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 7221;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 525152;
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

For some reason I really struggled with planning a solution for this one. Seems like I was
overcomplicating things as usual.
I will be basing my solution on the work done by the reddit user "StaticMoose" in the following
thread:
https://www.reddit.com/r/adventofcode/comments/18hbbxe/2023_day_12python_stepbystep_tutorial_with_bonus/

Solution presented by StaticMoose worked perfectly. I had to rewrite the code in rust which
required some differences but the logic is the same.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    const SAFE: u8 = 0;
    const BROKEN: u8 = 1;
    const UNKNOWN: u8 = 2;

    fn parse_line(data_line: &str) -> Result<(Vec<u8>, Vec<usize>), Box<dyn Error>> {
        let mut line_parts = data_line.split(' ');
        let mut tiles = Vec::new();

        for c in line_parts.next().unwrap().chars() {
            tiles.push(match c {
                '.' => SAFE,
                '#' => BROKEN,
                '?' => UNKNOWN,
                _ => return Err(format!("Unexpected char {c} in data!").into()),
            });
        }

        let numbers: Vec<usize> = line_parts
            .next()
            .ok_or("Missing numbers in data!")?
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>())
            .collect::<Result<_, _>>()?;

        Ok((tiles, numbers))
    }

    fn handle_broken_tile(tiles: &[u8], numbers: &[usize]) -> Result<u64, Box<dyn Error>> {
        // This method is only called when we know numbers contain at least
        let group_len = numbers[0]; // 1 item, meaning this will never fail.

        if tiles.len() < group_len || tiles.iter().take(group_len).any(|t| *t == SAFE) {
            return Ok(0);
        }

        if tiles.len() == group_len {
            return Ok((numbers.len() == 1) as u64);
        }

        if tiles[group_len] == BROKEN {
            return Ok(0);
        }

        calculate_arrangements(&tiles[group_len + 1..], &numbers[1..])
    }

    fn handle_safe_tile(tiles: &[u8], numbers: &[usize]) -> Result<u64, Box<dyn Error>> {
        calculate_arrangements(&tiles[1..], numbers)
    }

    fn calculate_arrangements(tiles: &[u8], numbers: &[usize]) -> Result<u64, Box<dyn Error>> {
        if numbers.is_empty() {
            return Ok(tiles.iter().all(|t| *t != BROKEN) as u64);
        }

        if tiles.is_empty() {
            return Ok(0);
        }

        Ok(match tiles[0] {
            BROKEN => handle_broken_tile(tiles, numbers)?,
            SAFE => handle_safe_tile(tiles, numbers)?,
            UNKNOWN => handle_broken_tile(tiles, numbers)? + handle_safe_tile(tiles, numbers)?,
            _ => return Err(format!("Unepected tiletype {} in tiles slice!", tiles[0]).into()),
        })
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut result = 0;
        for line in reader::get_lines(data_path)? {
            let (groups, numbers) = parse_line(&line)?;
            result += calculate_arrangements(&groups, &numbers)?;
        }

        Ok(result)
    }
}

//

//

/*
Part Two
##################################################################################################

Part two actually requires the same calculation logic, but we need to do some work with the tiles
before passing it to the calculation step.

More precisely we need to expand the tiles and numbers by copying them 5 times. Basically once we
have loaded the tiles and numbers, we copy the two Vec's 5 times and just add them together.

Then after that we use the earlier logic to find our answer in our now much larger tile sets.

This is very likely to require changes though as the potential arrangements will be massive
compared to Part One.
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
