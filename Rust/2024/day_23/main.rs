use std::io;

mod computer;
use computer::Computer;
use computer::ComputerError;
use computer::Identifier;

mod reader;
use reader::get_lines;
mod testing_debug;

#[derive(Debug)]
enum AdventError {
    IoError(io::Error),
    ComputerError(computer::ComputerError),
    CorruptData,
}

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<ComputerError> for AdventError {
    fn from(value: ComputerError) -> Self {
        Self::ComputerError(value)
    }
}

#[derive(Debug)]
struct NetworkGroup {
    computers: Vec<u16>,
    potential_connections: Vec<u16>,
}

impl NetworkGroup {
    fn new(computer: &Computer) -> NetworkGroup {
        NetworkGroup {
            computers: vec![computer.id],
            potential_connections: computer.connections.clone(),
        }
    }

    fn can_connect(&self, id: u16) -> bool {
        self.potential_connections.contains(&id)
    }

    fn copy_and_connect(&self, computer: &Computer) -> NetworkGroup {
        let mut potential_connections = Vec::new();
        let mut computers = self.computers.clone();
        for connection in &self.potential_connections {
            if computer.connections.contains(connection) {
                potential_connections.push(*connection);
            }
        }
        computers.push(computer.id);
        NetworkGroup {
            computers,
            potential_connections,
        }
    }
}

struct Network {
    computers: [Computer; 676],
}

impl Network {
    fn new() -> Network {
        Network {
            computers: core::array::from_fn(|i| Computer::new(i as u16)),
        }
    }

    fn add_connection_and_process_sets(
        &mut self,
        identifier_1: Identifier,
        identifier_2: Identifier,
    ) -> u64 {
        let mut result: u64 = 0;

        let (id_1, id_2) = (identifier_1.as_u16(), identifier_2.as_u16());
        let computer_2 = &self.computers[id_2 as usize];

        for computer_2_connection in &computer_2.connections {
            let computer_3 = &self.computers[*computer_2_connection as usize];
            for computer_3_connection in &computer_3.connections {
                if *computer_3_connection == id_1 {
                    result += 1;
                }
            }
        }

        self.connect_computers(id_1, id_2);
        result
    }

    fn connect_computers(&mut self, id_1: u16, id_2: u16) {
        self.computers[id_1 as usize].connections.push(id_2);
        self.computers[id_2 as usize].connections.push(id_1);
    }

    fn largest_group(&mut self) -> NetworkGroup {
        let mut computed_combinations: [bool; 676] = [false; 676];
        let (mut largest_group_size, mut largest_group): (u16, Option<NetworkGroup>) = (0, None);

        for computer in &self.computers {
            let groups: Vec<NetworkGroup> =
                self.groups_with_computer(computer, computed_combinations, largest_group_size);
            (largest_group_size, largest_group) =
                Network::update_largest_group(largest_group_size, largest_group, groups);
            computed_combinations[computer.id as usize] = true;
        }

        largest_group.unwrap()
    }

    fn update_largest_group(
        mut largest_group_size: u16,
        mut largest_group: Option<NetworkGroup>,
        new_groups: Vec<NetworkGroup>,
    ) -> (u16, Option<NetworkGroup>) {
        for group in new_groups {
            if group.computers.len() as u16 > largest_group_size {
                largest_group_size = group.computers.len() as u16;
                largest_group = Some(group);
            }
        }
        (largest_group_size, largest_group)
    }

    fn groups_with_computer(
        &self,
        computer: &Computer,
        computed_combinations: [bool; 676],
        current_largest_group: u16,
    ) -> Vec<NetworkGroup> {
        let mut groups: Vec<NetworkGroup> = Vec::new();
        groups.push(NetworkGroup::new(computer));
        for (index, id) in computer.connections.iter().enumerate().rev() {
            if computed_combinations[*id as usize] {
                continue;
            }
            let mut group_cache = Vec::new();
            for group in &groups {
                if group.can_connect(*id)
                    && group.computers.len() + index + 1 > current_largest_group as usize
                {
                    group_cache.push(group.copy_and_connect(&self.computers[*id as usize]));
                }
            }
            groups.append(&mut group_cache);
        }
        groups
    }
}

fn get_identifiers_from_line(line: String) -> Result<(Identifier, Identifier), AdventError> {
    let mut char_cache = [' '; 5];
    for (index, char) in line.chars().enumerate() {
        if index >= 5 {
            return Err(AdventError::CorruptData);
        }
        char_cache[index] = char;
    }
    Ok((
        Identifier::parse((char_cache[0], char_cache[1]))?,
        Identifier::parse((char_cache[3], char_cache[4]))?,
    ))
}

fn calculate(path: &str) -> Result<String, AdventError> {
    let lines = get_lines(path)?;

    let mut network = Network::new();

    for line in lines {
        let (identifier_1, identifier_2) = get_identifiers_from_line(line)?;
        let _ = network.add_connection_and_process_sets(identifier_1, identifier_2);
    }

    let mut computers = network.largest_group().computers;
    computers.sort();

    let mut result = String::new();
    for computer in computers {
        result.push_str(&Identifier::from(computer).to_string());
        result.push(',');
    }

    Ok(result)
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
        Ok(value) => assert_eq!(value, "co,de,ka,ta,"),
    }
}

#[test]
fn network_new_computer_ids() {
    let network = Network::new();
    assert_eq!(network.computers[42].id, 42);
}

/* Sudo code:

Challenge part 1:

Go thorugh the data file.
for each row:
    access the records for the second computer.
        for all connections away from that computer:
            if the connected computer has the first computer in its connection list:
                we have one successful set.

As long as the data doesn't have the same connection listed twice we should be safe to say we never count a set twice.

Experiment with hashing.
There are only a total of 676 different computers.
a-z is 0..25
first letter is a-z * 26
second letter is a-z.
Added together they might create a unique value based on the name?
don't forget to subtract with 26 if the values are 1..26, to

This works.
kh = (10*26) + 7 = 267

We could save the name as a number instead of chars.
Simply convert the two letters to a number right when reading it from file.

the number as chars could be found by:
    divide by 26 for the first letter index.
    modulo by 26 for the second letter index.


This would result in VERY fast indexing and accessing of computers.
Of course memory wise it might not be the most efficient as we can't know if all possible computers will be used.

a as u16 is 97, b is 98 ...
To get the position in the alphabet: Subtract the char as u16 by 97.



###########################
Now this is interesting...
First run after all tests pass returned the result 1476.
Which after being inserted to the website the following was returned:

That's not the right answer; your answer is too low. Curiously, it's the right answer for someone else; you might be logged in to the wrong account or just unlucky. In any case, you need to be using your puzzle input. If you're stuck, make sure you're using the full input data; there are also some general tips on the about page, or you can ask for hints on the subreddit. Please wait one minute before trying again.

Which I guess means the puzzle input is actually randomized individually for each participant! Clever!


I can't see any sets where one or more computers start with T in the failed list, meaning the issue is not with the way we check if a computer starts with T.
Which leaves the set matching logic as the root of the problem, or the way we read the data from file.
Important to note is that the test case does pass, and with the correct sets.
It is a much smaller scale test, but that would likely mean that the problem we are looking for lies in some edge case.
All lines of the file are read so that is not the issue.


1476 is too low



Challenge part 2:

So now I am supposed to figure out what set of interconnected computer is the largest.
Basically what is the largest set where every computer has a direct connection to all others in the set.

I am not getting anywhere today. I could likely get it "working" relatively quickly, but would it run well, be easy to understand/maintain? No.
Is that something that is worth to do anyway? Maybe. We shall see if any answers appear during the weekend...

// This is a interesting problem. AFTER I have solved it try asking chat-gpt. Would be interesting
// to see how the ai would handle this.

*/
