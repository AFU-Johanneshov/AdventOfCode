mod reader;
use std::collections::HashMap;

use reader::get_lines;

macro_rules! testonly_println {
    ($($x:tt)*) => {
        #[cfg(test)]
        println!($($x)*);
    };
}

macro_rules! testonly_print {
    ($($x:tt)*) => {
        #[cfg(test)]
        print!($($x)*);
    };
}

const BLINKS: u8 = 75;

fn get_stones(path: &str) -> HashMap<u64, u64> {
    let Ok(mut lines) = get_lines(path) else {
        panic!("Data file could not be read!");
    };

    let mut stones: HashMap<u64, u64> = HashMap::new();
    let mut stone_string: String = String::new();

    for char in lines.next().expect("File was empty!").chars() {
        if char.is_numeric() {
            stone_string.push(char);
            continue;
        }
        *stones.entry(stone_string.parse().unwrap()).or_default() += 1;
        stone_string = String::new();
    }

    *stones.entry(stone_string.parse().unwrap()).or_default() += 1;

    stones
}
fn split(stone: u64) -> (u64, u64) {
    let string = stone.to_string();
    let stone1 = string[0..string.len() / 2].parse().unwrap();
    let stone2 = string[string.len() / 2..string.len()].parse().unwrap();
    (stone1, stone2)
}

fn even_digits(stone: u64) -> bool {
    let digit_count = stone.ilog10() + 1;

    //testonly_println!("{} digit_count is: {}", self.0, digit_count);

    if digit_count % 2 == 0 {
        return true;
    }
    false
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut result: HashMap<u64, u64> = HashMap::new();

    for (stone, count) in stones {
        match stone {
            0 => *result.entry(1).or_default() += count,
            s if even_digits(stone) => {
                let (s1, s2) = split(stone);
                *result.entry(s1).or_default() += count;
                *result.entry(s2).or_default() += count;
            }
            _ => *result.entry(stone * 2024).or_default() += count,
        };
    }

    result
}

fn calculate(path: &str) -> u64 {
    // Start here:

    let mut stones: HashMap<u64, u64> = get_stones(path);

    for i in 0..BLINKS {
        stones = blink(stones);
    }

    stones.values().sum()
}

fn main() {
    // Start here:
    //let result = calculate("data.txt");
    let result = calculate("data.txt");
    println!("result: {result}");
}

#[test]
fn calculate_test() {
    let result = calculate("testdata.txt");
    assert_eq!(result, 55312);
}

/* Sudo code:

Challenge part 1:

result:
172284 is too high


Challenge part 2:
Calculating 25 blinks is relatively easy. Results in a little over 200 000 stones.
But calculating 75 blinks is a very different Challenge. At that point the amount of
stones at the end will be extremely high. Too high to brute force.

Some sort of caching of already calculated chains must be made.
Since a 0 will always have the same chain of numbers behind it we should be able to make a shortcut where we only
calculate that chain once, and then use it whenever a 0 shows up.

Chain_0
1  0
1  1
1  2024
2  20 24
4  2 NEW_CHAIN_0 2 4
4  4048 4048 8096
7  40 48 40 48 80 96
14 4 NEW_CHAIN_0 4 8 4 NEW_CHAIN_0 4 8 8 NEW_CHAIN_0 9 6
16 8096 8096 16192 8096 8096 16192 16192 18216 12144
27 80 96 80 96 32772608 80 96 80 96 32772608 32772608 36869184 24579456

Had to get some guidance on this because of tunnel vision.
I have been trying to brute force it in different ways, but my old friend hashing is back to save the day.

Instead of counting them one branch at a time, we go back to one blink at a time. But instead of going through all the
stones one by one, we use a hashmap to store the current stone numbers and how many of them exists.
That way we don't calculate any duplicate stones.

*/
