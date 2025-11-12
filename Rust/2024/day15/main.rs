use std::fmt::{self};
use std::fs::File;
use std::io;
use std::io::{BufReader, Lines};
use std::iter::Flatten;
use std::{thread, time};

mod reader;
use reader::get_lines;
mod testing_debug;
mod vector;
use vector::VectorI16;

#[derive(Debug)]
enum AdventError {
    IoError(io::Error),
    InvalidDataFormat(String),
    MapError(MapError),
}

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<MapError> for AdventError {
    fn from(err: MapError) -> Self {
        Self::MapError(err)
    }
}

const GRIDSIZE: usize = 50;
const UP: VectorI16 = VectorI16 { x: 0, y: -1 };
const DOWN: VectorI16 = VectorI16 { x: 0, y: 1 };
const LEFT: VectorI16 = VectorI16 { x: -1, y: 0 };
const RIGHT: VectorI16 = VectorI16 { x: 1, y: 0 };

struct Instructions(Flatten<Lines<BufReader<File>>>);

#[derive(Copy, Clone)]
enum Tile {
    Wall,
    Empty,
    BoxPart(BoxPart),
    Robot,
}

impl Tile {
    fn is_box_part(&self) -> Option<BoxPart> {
        match self {
            Tile::Wall => None,
            Tile::Empty => None,
            Tile::Robot => None,
            Tile::BoxPart(box_part) => Some(*box_part),
        }
    }
}

#[derive(Copy, Clone)]
struct BoxPart {
    is_left_part: bool,
    location: VectorI16,
}

impl BoxPart {
    fn box_is_pushable(&self, map: &Map, direction: VectorI16) -> Result<bool, MapError> {
        let (box_left, box_right) = self.full_box(map)?;

        if box_left.location + direction != box_right.location
            && !map.tile_available(box_left.location + direction, direction)?
        {
            return Ok(false);
        }

        if box_right.location + direction != box_left.location
            && !map.tile_available(box_right.location + direction, direction)?
        {
            return Ok(false);
        }

        Ok(true)
    }

    fn push_box(&self, map: &mut Map, direction: VectorI16) -> Result<(), MapError> {
        let (mut box_left, mut box_right) = self.full_box(map)?;

        if box_left.location + direction != box_right.location {
            map.clear_tile_with_push(box_left.location + direction, direction)?;
        }

        if box_right.location + direction != box_left.location {
            map.clear_tile_with_push(box_right.location + direction, direction)?;
            //box_right.push(map, direction)?;
        }

        let _ = box_left.relocate(map, direction);

        Ok(())
    }

    fn relocate(&mut self, map: &mut Map, direction: VectorI16) -> Result<(), MapError> {
        let (mut cache_left, mut cache_right) = self.full_box(map)?;

        map.set_tile(cache_left.location, Tile::Empty)?;
        map.set_tile(cache_right.location, Tile::Empty)?;
        cache_left.location = cache_left.location + direction;
        cache_right.location = cache_right.location + direction;

        map.set_tile(cache_left.location, Tile::BoxPart(cache_left))?;
        map.set_tile(cache_right.location, Tile::BoxPart(cache_right))?;
        Ok(())
    }

    fn full_box(&self, map: &Map) -> Result<(BoxPart, BoxPart), MapError> {
        // Unwrap() is used here because in the context of this program the following would never
        // fail during operation. IF it where to fail then a crash is fine because there is
        // something very wrong that caused the box to "split".
        match self.is_left_part {
            true => Ok((
                map.get_tile(self.location)?.is_box_part().unwrap(),
                map.get_tile(self.location + RIGHT)?.is_box_part().unwrap(),
            )),
            false => Ok((
                map.get_tile(self.location + LEFT)?.is_box_part().unwrap(),
                map.get_tile(self.location)?.is_box_part().unwrap(),
            )),
        }
    }

    fn push(&mut self, map: &mut Map, direction: VectorI16) -> Result<(), MapError> {
        match map.get_tile(self.location + direction)? {
            Tile::Robot => return Err(MapError::PushBlocked),
            Tile::Wall => return Err(MapError::PushBlocked),
            Tile::BoxPart(box_part) => box_part.push_box(map, direction)?,
            Tile::Empty => {}
        };

        map.set_tile(self.location, Tile::Empty);
        self.location = self.location + direction;
        map.set_tile(self.location + direction, Tile::BoxPart(*self));

        Ok(())
    }
}

struct Map {
    grid: [[Tile; GRIDSIZE]; GRIDSIZE * 2],
    size_override: usize,
    robot_location: VectorI16,
}
#[derive(Debug)]
enum MapError {
    OutOfBounds(String),
    PushBlocked,
    NotBoxPart,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string =
            String::with_capacity((self.size_override * self.size_override) + self.size_override);
        for y in 0..self.size_override {
            for x in 0..self.size_override * 2 {
                string.push(match self.grid[x][y] {
                    Tile::Empty => '.',
                    Tile::Robot => '@',
                    Tile::Wall => '#',
                    Tile::BoxPart(box_part) => match box_part.is_left_part {
                        true => '[',
                        false => ']',
                    },
                });
            }
            string.push('\n');
        }

        write!(f, "Map: \n{}", string)
    }
}

impl Map {
    fn instruct_robot(&mut self, instruction: char) -> Result<(), MapError> {
        let direction = match instruction {
            '^' => UP,
            'v' => DOWN,
            '<' => LEFT,
            '>' => RIGHT,
            _ => panic!("Invalid instruction!"),
        };

        let next_location = self.robot_location + direction;

        match self.get_tile(next_location)?
        {
            Tile::Wall => return Err(MapError::PushBlocked),
            Tile::Robot => panic!("Robot at {:?} attempted to move to {:?} which was occupied by another robot! There should only be one robot at all times!", self.robot_location, next_location),
            Tile::BoxPart(box_part) => {
                if !box_part.box_is_pushable(&self, direction)?
                {
                    return Err(MapError::PushBlocked);
                }
                box_part.push_box(self, direction);
            },
            Tile::Empty => {},
        };

        self.set_tile(next_location, Tile::Robot)?;
        self.set_tile(self.robot_location, Tile::Empty)?;
        self.robot_location = next_location;

        Ok(())
    }

    fn tile_available(&self, location: VectorI16, direction: VectorI16) -> Result<bool, MapError> {
        match self.get_tile(location)? {
            Tile::Empty => Ok(true),
            Tile::Wall => Ok(false),
            Tile::Robot => Ok(false),
            Tile::BoxPart(box_part) => {
                if !box_part.box_is_pushable(&self, direction)? {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
        }
    }

    fn clear_tile_with_push(
        &mut self,
        location: VectorI16,
        direction: VectorI16,
    ) -> Result<(), MapError> {
        match self.get_tile(location)? {
            Tile::Robot => return Err(MapError::PushBlocked),
            Tile::Wall => return Err(MapError::PushBlocked),
            Tile::BoxPart(box_part) => box_part.push_box(self, direction)?,
            Tile::Empty => {}
        };
        Ok(())
    }

    fn boxes(&self) -> BoxesIter {
        BoxesIter {
            map: self,
            x: 0,
            y: 0,
        }
    }

    fn valid_location(&self, location: VectorI16) -> Result<VectorI16, MapError> {
        if location.x < 0
            || location.x > self.size_override as i16 * 2
            || location.y < 0
            || location.y > self.size_override as i16
        {
            return Err(MapError::OutOfBounds(format!(
                "Vector: {:?} is out of bounds: x: 0, y: 0 -> x: {}, y: {}",
                location, self.size_override, self.size_override
            )));
        }
        Ok(location)
    }

    fn get_tile(&self, location: VectorI16) -> Result<Tile, MapError> {
        let valid_location = self.valid_location(location)?;
        Ok(self.grid[valid_location.x as usize][valid_location.y as usize])
    }

    fn set_tile(&mut self, location: VectorI16, tile: Tile) -> Result<Tile, MapError> {
        let valid_location = self.valid_location(location)?;
        self.grid[valid_location.x as usize][valid_location.y as usize] = tile;
        Ok(tile)
    }
}

struct BoxesIter<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
}

impl<'a> Iterator for BoxesIter<'a> {
    type Item = VectorI16;

    fn next(&mut self) -> Option<Self::Item> {
        for _ in self.y..self.map.size_override {
            for _ in self.x..self.map.size_override * 2 {
                //println!("Checking: {}.{}", self.x, self.y);
                if let Tile::BoxPart(box_part) = self.map.grid[self.x][self.y] {
                    if box_part.is_left_part {
                        self.x += 1;
                        return Some(box_part.location);
                    }
                };

                self.x += 1;
            }
            self.x = 0;
            self.y += 1;
        }
        None
    }
}

fn read_data_file(path: &str) -> Result<(Map, Instructions), AdventError> {
    let mut lines = get_lines(path)?;

    let mut grid: [[Tile; GRIDSIZE]; GRIDSIZE * 2] = [[Tile::Empty; GRIDSIZE]; GRIDSIZE * 2];
    let mut robot_location: VectorI16 = VectorI16::default();

    let (mut x, mut y) = (0, 0);
    loop {
        let Some(line) = lines.next() else {
            return Err(AdventError::InvalidDataFormat(String::from(
                "End of file was reached before space for instructions was found!",
            )));
        };

        if line.is_empty() {
            break;
        }

        for char in line.chars() {
            let (tile1, tile2) = get_tile(char, VectorI16::from((x, y)), &mut robot_location)?;
            grid[x][y] = tile1;
            x += 1;
            grid[x][y] = tile2;
            x += 1;
        }

        if y > GRIDSIZE {
            break;
        }
        y += 1;
        x = 0;
    }

    Ok((
        Map {
            grid,
            size_override: y,
            robot_location,
        },
        Instructions(lines),
    ))
}

fn get_tile(
    char: char,
    location: VectorI16,
    robot_location: &mut VectorI16,
) -> Result<(Tile, Tile), AdventError> {
    let tile = match char {
        '#' => (Tile::Wall, Tile::Wall),
        'O' => (
            Tile::BoxPart(BoxPart {
                is_left_part: true,
                location,
            }),
            Tile::BoxPart(BoxPart {
                is_left_part: false,
                location: location + RIGHT,
            }),
        ),
        '.' => (Tile::Empty, Tile::Empty),
        '@' => {
            *robot_location = location;
            (Tile::Robot, Tile::Empty)
        }
        _ => {
            return Err(AdventError::InvalidDataFormat(format!(
                "Invalid character {} in map data.",
                char
            )))
        }
    };
    Ok(tile)
}

fn calculate(path: &str) -> Result<u64, AdventError> {
    let (mut map, instructions) = read_data_file(path)?;

    println!("{}", map);

    for line in instructions.0 {
        for char in line.chars() {
            match map.instruct_robot(char) {
                Ok(()) => {
                    println!("{}", map);
                    println!("----------------------------------------------------------------------------------------------------------------------");

                    thread::sleep(time::Duration::from_millis(1));
                }
                Err(map_error) => match &map_error {
                    MapError::OutOfBounds(str) => return Err(AdventError::from(map_error)),
                    MapError::NotBoxPart => return Err(AdventError::from(map_error)),
                    MapError::PushBlocked => {}
                },
            }
        }
    }

    let mut result: u64 = 0;
    for box_location in map.boxes() {
        result += box_location.y as u64 * 100 + box_location.x as u64;
    }

    println!("{}", map);

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
        Ok(value) => assert_eq!(value, 9021),
    }
}

#[test]
fn calculate_test_small() {
    match calculate("smalltestdata.txt") {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, 2028),
    }
}

/* Sudo code:

Challenge part 1:



Challenge part 2:

768274 is too low

*/
