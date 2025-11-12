use std::io;

mod reader;
use reader::get_lines;
mod testing_debug;

#[derive(Debug)]
enum AdventError {
    IoError(io::Error),
}

const COLOR_COUNT: usize = 5;

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

#[derive(Debug)]
struct PatternPart {
    next: [Option<Box<PatternPart>>; COLOR_COUNT],
    is_endpoint: bool,
}

impl PatternPart {
    fn insert_raw_pattern(&mut self, pattern: &[usize]) {
        let Some(pattern_box) = &mut self.next[pattern[0]] else {
            let mut new_pattern_part = PatternPart {
                next: [const { Option::None }; COLOR_COUNT],
                is_endpoint: false,
            };
            if pattern.len() > 1 {
                new_pattern_part.insert_raw_pattern(&pattern[1..]);
            } else {
                new_pattern_part.is_endpoint = true;
            }

            self.next[pattern[0]] = Some(Box::new(new_pattern_part));
            return;
        };

        if pattern.len() > 1 {
            pattern_box.insert_raw_pattern(&pattern[1..]);
        } else {
            pattern_box.is_endpoint = true;
        }
    }
}

fn get_patterns(line: String) -> PatternPart {
    let mut patterns = PatternPart {
        next: [const { Option::None }; COLOR_COUNT],
        is_endpoint: false,
    };

    let mut cache_str = String::new();
    for c in line.chars() {
        if c != ' ' && c != ',' {
            cache_str.push(c);
        } else if !cache_str.is_empty() {
            patterns.insert_raw_pattern(&parse_line(&cache_str));
            cache_str.clear();
        }
    }
    patterns.insert_raw_pattern(&parse_line(&cache_str));

    patterns
}

fn parse_line(line: &str) -> Vec<usize> {
    let mut raw_patterns: Vec<usize> = Vec::with_capacity(line.len());
    for char in line.chars() {
        raw_patterns.push(match char {
            'w' => 0,
            'u' => 1,
            'b' => 2,
            'r' => 3,
            'g' => 4,
            _ => continue,
        });
    }
    raw_patterns
}

fn pattern_matcher(
    desired_pattern: &[usize],
    full_pattern_tree: &PatternPart,
    current_branch: &PatternPart,
    depth_branch_map: &mut [u64],
) -> u64 {
    let mut result: u64 = 0;
    if desired_pattern.is_empty() {
        return if current_branch.is_endpoint { 1 } else { 0 };
    }

    let Some(next_branch) = &current_branch.next[desired_pattern[0]] else {
        return 0;
    };

    result += pattern_matcher(
        &desired_pattern[1..],
        full_pattern_tree,
        next_branch,
        depth_branch_map,
    );

    if next_branch.is_endpoint {
        result += if depth_branch_map[desired_pattern.len() + 1] != 0 {
            depth_branch_map[desired_pattern.len() + 1]
        } else {
            let value = pattern_matcher(
                &desired_pattern[1..],
                full_pattern_tree,
                full_pattern_tree,
                depth_branch_map,
            );

            depth_branch_map[desired_pattern.len() + 1] = value;
            value
        };
    }

    result
}

fn calculate(path: &str) -> Result<u64, AdventError> {
    let mut lines = get_lines(path)?;
    let patterns = get_patterns(lines.next().expect("Data file read has no lines!"));
    let mut possible_patterns = 0;

    for line in lines {
        let parsed_line = &parse_line(&line);

        let mut depth_branch_map: [u64; 61] = [0; 61];
        possible_patterns +=
            pattern_matcher(parsed_line, &patterns, &patterns, &mut depth_branch_map);
    }

    Ok(possible_patterns)
}

fn main() {
    match calculate("data.txt") {
        Err(err) => println!("An error occured: {err:?}"),
        Ok(value) => println!("Result is: {}", value),
    }
}

#[test]
fn calculate_test() {
    match calculate("testdata.txt") {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, 16),
    }
}

/* Sudo code:

Challenge part 1:
This will require a LOT of comparing elements.
First thought was simply a recursive function which takes in a string slice, then checks each pattern in the list for matching the first letter, then the second and so on.

But, I rather try to challenge myself to make it in a harder but more optimized way.
Instead of storing the available patterns in a large list, we instead use a hashset storing


Challenge part 2:

195970913 TO LOW


*/
