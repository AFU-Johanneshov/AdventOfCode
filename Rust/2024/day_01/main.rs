use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::id;

fn get_lists(path: &str) -> Result<(Vec<i32>, Vec<i32>), std::io::Error> {
    let file = File::open(path)?;

    let reader = io::BufReader::new(file).lines();

    let mut result: (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());

    for line in reader.flatten() {
        let ints: (i32, i32) = (line[0..5].parse().unwrap(), line[8..13].parse().unwrap());
        //println!("ints: {ints:?}");
        result.0.push(ints.0);
        result.1.push(ints.1);
    }

    Ok(result)

    //todo!();
}

fn main() {
    match get_lists("data.txt") {
        Ok(mut lists) => {
            lists.0.sort();
            lists.1.sort();

            let mut distance: u32 = 0;

            let distances: Vec<u32> = lists
                .0
                .iter()
                .enumerate()
                .map(|(index, value)| lists.1[index].abs_diff(*value))
                .collect();

            for i in distances {
                distance += i;
            }

            println!("Answer 1 is: {}", distance);

            let mut occurances: HashMap<i32, u16> = HashMap::new();

            for i in lists.1.iter() {
                let count: &mut u16 = occurances.entry(*i).or_default();
                *count += 1;
            }

            let mut similarity: i32 = 0;

            for i in lists.0.iter() {
                if let Some(count) = occurances.get(i) {
                    similarity += i * *count as i32
                }
            }

            println!("Answer 2 is: {}", similarity);
        }
        Err(err) => {
            println!("Error reading lists: {err}");
        }
    };
}

/*
Todo list:

Read the two lists into two arrays of 1000 elements each.
Sort the lists by size.
iterate through the lists one index at a time and compare the two values of the lists.
    calculate the difference between the two values.
    Add the result difference to a total.

When finished iterating through the list the total is the answer.


Todo list 2:

Read the two list ino two vec.
transfer the elements of the right list to a hashmap.
    Where the element is the key. And a internal value of how many times that element occurs.

Iterate through the left list.
    For every element look at the hashmap with the element as a key.
        if the hashmap contains it, multiply the element with the amount of times it occurs.
        add to total.

When finished iterating through the list the total is the answer.

*/
