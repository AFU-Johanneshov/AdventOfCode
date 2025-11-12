mod reader;
use reader::get_lines;

enum ReadMode {
    EmptySpace,
    File(u16),
}

impl ReadMode {
    fn get_cell(&self) -> Cell {
        match self {
            ReadMode::EmptySpace => Cell::Empty,
            ReadMode::File(id) => Cell::Full(*id),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Cell {
    Empty,
    Full(u16),
}

#[derive(Debug)]
struct File {
    id: u16,
    address: (usize, usize),
}

impl File {
    fn size(&self) -> usize {
        ////println!("{:?} size is: {}", self, self.address.1 - self.address.0);
        1 + self.address.1 - self.address.0 // Add 1 since a start and end point at the same index still equals to a size of one, not 0.
    }

    fn move_file(self, disk: &mut [Cell; 94192], target_address: usize) {
        for i in target_address..target_address + self.size() {
            let Cell::Empty = disk[i] else {
                panic!(
                    "Expected free cell on disk at index: {} but found: {:?}",
                    i, disk[i]
                );
            };

            disk[i] = Cell::Full(self.id);
        }

        for i in self.address.0..self.address.1 + 1 {
            let Cell::Full(_) = disk[i] else {
                panic!(
                    "Expected index: {} to contain a cell from file: {:?} but the cell was empty!",
                    i, self
                );
            };

            disk[i] = Cell::Empty;
        }
    }
}

fn get_disk(path: &str) -> ([Cell; 94192], usize) {
    let Ok(lines) = get_lines(path) else {
        panic!("Data file could not be read!");
    };

    /*
    let mut i = 0;
    for line in lines {
        for char in line.chars() {
            i += char.to_digit(10).unwrap();
        }
    }
    println!("Disk size: {i}");
    */

    // /*
    let mut disk: [Cell; 94192] = [Cell::Empty; 94192];
    let mut next_id = 0;
    let mut diskreader: ReadMode = ReadMode::File(0);
    let mut next_index = 0;

    for line in lines {
        for char in line.chars() {
            let Some(value) = char.to_digit(10) else {
                panic!("Unexpected character! Expected number but found: {char}");
            };

            for _ in 0..value {
                disk[next_index] = diskreader.get_cell();
                next_index += 1;
            }

            diskreader = match diskreader {
                ReadMode::EmptySpace => {
                    next_id += 1;
                    ReadMode::File(next_id)
                }
                ReadMode::File(_) => ReadMode::EmptySpace,
            };
        }
    }

    (disk, next_index)
}

fn compress_disk(disk: &mut [Cell; 94192], size_override: usize) {
    let mut compressed_index = 0;
    for uncompressed_index in (0..size_override).rev() {
        let Cell::Full(id) = disk[uncompressed_index] else {
            continue;
        };

        loop {
            if uncompressed_index <= compressed_index {
                return;
            }
            match disk[compressed_index] {
                Cell::Full(_) => compressed_index += 1,
                Cell::Empty => {
                    disk.swap(compressed_index, uncompressed_index);
                    compressed_index += 1;
                    break;
                }
            }
        }
    }
}

fn get_file(disk: &[Cell; 94192], id: u16, end_address: usize) -> File {
    let mut i = end_address;
    while i > 0 {
        match disk[i] {
            Cell::Empty => break,
            Cell::Full(cell_id) => {
                if cell_id != id {
                    break;
                }
            }
        };
        i -= 1;
    }

    File {
        id,
        address: (i + 1, end_address), // Add one to compensate for i being the index a different
                                       // cell was found. I.e. the file starts at the index after
                                       // that cell. i + 1.
    }
}

fn find_free_space(
    disk: &[Cell; 94192],
    required_size: usize,
    start_index: usize,
    size_override: usize,
) -> Option<usize> {
    let mut consecutive_free_spaces = 0;
    for i in start_index..size_override {
        let Cell::Empty = disk[i] else {
            consecutive_free_spaces = 0;
            continue;
        };
        consecutive_free_spaces += 1;
        if consecutive_free_spaces < required_size {
            continue;
        }
        return Some(i + 1 - consecutive_free_spaces); // Add 1 to ensure that the start index of
                                                      // the consecutive free spaces is correct.
    }
    None
}

fn reorder_disk(disk: &mut [Cell; 94192], size_override: usize, silent: bool) {
    //let mut free_space_index: usize = 0;
    //let mut search_index = size_override + 1;
    let mut file_id = size_override as u16;
    for search_index in (0..size_override).rev() {
        //search_index -= 1;
        let Cell::Full(id) = disk[search_index] else {
            continue;
        };

        if id >= file_id {
            continue;
        }

        file_id = id;
        let file = get_file(disk, id, search_index);
        if !silent {
            println!();
            println!("Found file: {:?}", file);
        }

        let Some(start_address) = find_free_space(disk, file.size(), 0, size_override) else {
            if !silent {
                println!("No free space");
            }
            continue;
        };

        if file.address.0 < start_address {
            if !silent {
                println!("Free space occurs after file address");
            }
            continue;
        }

        if !silent {
            println!("File moved to: {start_address}");
        }
        file.move_file(disk, start_address);
    }
}

fn disk_checksum(disk: &[Cell; 94192], size_override: usize) -> u64 {
    let mut checksum: u64 = 0;

    for (i, cell) in disk.iter().enumerate().take(size_override) {
        let Cell::Full(id) = cell else {
            continue;
        };

        checksum += i as u64 * *id as u64;
    }

    checksum
}

fn print_disk(&disk: &[Cell; 94192], size_override: usize) {
    println!();
    for cell in disk.iter().take(size_override) {
        print!(
            "{}",
            match cell {
                Cell::Full(id) => id.to_string(),
                Cell::Empty => ".".to_string(),
            }
        );
    }
    println!();
}

fn calculate(path: &str, silent: bool) -> u64 {
    // Start here:

    let (mut disk, size) = get_disk(path);
    //println!("{:?}", disk);
    //println!("Size: {size}");

    if !silent {
        print_disk(&disk, size);
    }
    //compress_disk(&mut disk, size);
    reorder_disk(&mut disk, size, silent);

    if !silent {
        print_disk(&disk, size);
    }

    disk_checksum(&disk, size)
}

fn main() {
    // Start here:
    let checksum = calculate("./data.txt", true);
    println!("Compressed disk checksum: {checksum}");
}

#[test]
fn calculate_test() {
    // Test case disk size is: 42
    let checksum = calculate("./data2.txt", false);
    assert_eq!(checksum, 2858);
}

/* Sudo code:

Challenge part 1:

Read the data file into the following data structure:

Alt:
Figure out the total disk size of the test cases first.
(Loop through each character of the data file, adding them all together for the total size.)
Then use that total size as a constant for a "disk" array.
The array should contain:
enum Cell
{
empty,
full(u16 id),
}



Challenge part 2:



*/
