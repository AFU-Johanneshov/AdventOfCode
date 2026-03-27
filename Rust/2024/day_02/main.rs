use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::result;

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./data.txt") {
        let mut count: u16 = 0;
        for line in lines.flatten() {
            //println!("{}", line);

            let (mut data, length) = extract_report(&line);
            let increasing = get_increasing(&data);
            let safe = get_safety(&mut data, length, increasing, false);
            if safe {
                count += 1;
            }
        }
        println!("Out of all the reports exactly {} are safe...", count);
    }
}

fn extract_report(s: &str) -> ([u8; 8], u8) {
    let mut result: [u8; 8] = [0; 8];
    let mut count: usize = 0;
    let mut lastchars: String = String::new();
    for c in s.chars() {
        if !c.is_whitespace() {
            lastchars.push(c);
            continue;
        }

        result[count] = lastchars.parse().unwrap();
        count += 1;
        lastchars = String::new();
    }

    if !lastchars.is_empty() {
        result[count] = lastchars.parse().unwrap();
        count += 1;
    }

    (result, count as u8)
}

fn get_increasing(data: &[u8; 8]) -> bool {
    data[1] as i16 - data[0] as i16 > 0
}

fn delete_index(data: &mut [u8; 8], index: usize) {
    for i in 1 + index..8 {
        data[i - 1] = data[i];
    }
    data[7] = 0;
}

fn get_safety(data: &mut [u8; 8], length: u8, increasing: bool, dampener_used: bool) -> bool {
    let mut lastvalue = data[0];
    //let mut dampener_used = false;
    for i in 1..length {
        let diff: i16 = data[i as usize] as i16 - lastvalue as i16;
        let abs_diff = diff.abs();
        if !(1..=3).contains(&abs_diff) || increasing && diff < 0 || !increasing && diff > 0 {
            if dampener_used {
                return false;
            }
            //println!("");

            // I am certainly not happy with this but seeing as I couldn't figure it out I had to
            // do the brute force method. Extremely inefficient but at least it works....
            for i in 0..8 {
                let mut data = data.clone();
                delete_index(&mut data, i as usize);
                let result = get_safety(&mut data, length - 1, increasing, true);
                if result {
                    return true;
                }
                let result = get_safety(&mut data, length - 1, !increasing, true);
                if result {
                    return true;
                }
            }
            return false;

            if i == 2 {
                println!("Case index 2!");
                let mut data = data.clone();
                println!("SPEC Delete index: {} from {:?}", 0, data);
                delete_index(&mut data, 0);
                println!("SPEC With result: {data:?}");
                let result = get_safety(&mut data, length - 1, !increasing, true);
                println!("SPEC result: {result}");
                if result {
                    return true;
                }
            }

            let mut clone = data.clone();
            println!("Delete index: {} from {:?}", i, clone);
            delete_index(&mut clone, i as usize);
            println!("With result: {clone:?}");
            let result = get_safety(&mut clone, length - 1, increasing, true);
            println!("result: {result}");
            if result {
                return true;
            }

            println!("Delete index: {} from {:?}", i - 1, data);
            delete_index(data, i as usize - 1);
            println!("With result: {data:?}");
            let result = get_safety(data, length - 1, increasing, true);
            println!("result: {result}");
            return result;
        }
        lastvalue = data[i as usize];
    }
    true
}

/*

Todo list:

Read lines from data.txt one by one.
For each line:
    create integer array from the string. (Max size of 8)
    bool increasing
    int lastvalue = first value of the array.
    iterate through the rest of the array: i
        int diff = i - lastvalue.
        if first iteration:
            increasing = true if diff > 0 else false.

        if diff(aboslute) is greater than 3 or less than 1 return safe = false

        if increasing && diff < 0 || !increasing && diff > 0 return safe = false

    if iteration finishes then safe = true

Get record:
int array with size 8. array.
int count;
string lastchars
foreach char in input string.
    if char is NOT whitespace
        add char to lastchars.
        continue;

    array[count] = lastchars.parse_integer.
    count++;
    lastchars = ""
}

if lastchars.length > 0
    array[count] = lastchars.parse_integer.
    count++;
    lastchars = ""

return array and count.
*/
