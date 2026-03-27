use std::error::Error;

use vector::{self, VectorI16};

mod data_parser;
mod operations;
mod reader;

#[derive(Clone, Copy, Debug)]
struct Visits {
    directions: [bool; 4],
}

impl Visits {
    fn new() -> Visits {
        Visits {
            directions: [false; 4],
        }
    }
    fn visit(&mut self, direction: VectorI16) -> bool {
        let dir_index = match (direction.x, direction.y) {
            (0, -1) => 0,
            (1, 0) => 1,
            (0, 1) => 2,
            (-1, 0) => 3,
            _ => panic!("Invalid direction! {direction:?}"),
        };
        if self.directions[dir_index] {
            return true;
        }
        self.directions[dir_index] = true;
        false
    }
}

#[derive(Clone, Copy, Debug)]
struct Ray {
    direction: VectorI16,
    location: VectorI16,
}

impl Ray {
    fn from(direction: (i16, i16), location: (usize, usize)) -> Ray {
        Ray {
            direction: VectorI16::from(direction),
            location: VectorI16::from(location),
        }
    }
    fn change_direction(&self, new_direction: VectorI16) -> Ray {
        Ray {
            direction: new_direction,
            location: self.location,
        }
    }

    fn step(&mut self) {
        self.location = self.location + self.direction;
    }

    fn starter_ray() -> Ray {
        Ray {
            direction: VectorI16::from((1, 0)),
            location: VectorI16::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum TileType {
    Empty,
    MirrorLeft,
    MirrorRight,
    SplitterVertical,
    SplitterHorizontal,
}

impl TileType {
    fn as_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::MirrorLeft => '\\',
            Self::MirrorRight => '/',
            Self::SplitterVertical => '|',
            TileType::SplitterHorizontal => '-',
        }
    }

    fn transform_ray(&self, ray: Ray) -> Vec<Ray> {
        let mut result = Vec::new();
        match self {
            Self::Empty => result.push(ray),
            Self::MirrorLeft => result
                .push(ray.change_direction(VectorI16::from((ray.direction.y, ray.direction.x)))),
            Self::MirrorRight => result.push(
                ray.change_direction(VectorI16::from((ray.direction.y, ray.direction.x)) * -1),
            ),
            Self::SplitterVertical => {
                if ray.direction.x != 0 {
                    result.push(ray.change_direction(VectorI16::from((0, 1))));
                    result.push(ray.change_direction(VectorI16::from((0, -1))));
                } else {
                    result.push(ray);
                }
            }
            TileType::SplitterHorizontal => {
                if ray.direction.y != 0 {
                    result.push(ray.change_direction(VectorI16::from((1, 0))));
                    result.push(ray.change_direction(VectorI16::from((-1, 0))));
                } else {
                    result.push(ray);
                }
            }
        }

        result
    }
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    tile_type: TileType,
    visits: Visits,
    energized: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            tile_type: TileType::Empty,
            visits: Visits::new(),
            energized: false,
        }
    }
}

impl Tile {
    fn new(tile_type: TileType) -> Tile {
        Tile {
            tile_type,
            visits: Visits::new(),
            energized: false,
        }
    }

    fn process_ray(&mut self, ray: Ray) -> Vec<Ray> {
        self.energized = true;
        self.tile_type.transform_ray(ray)
    }
}

#[derive(Clone, Copy, Debug)]
struct Grid {
    tiles: [[Tile; 110]; 110],
    actual_size: usize,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            tiles: [[Tile::default(); 110]; 110],
            actual_size: 0,
        }
    }

    fn set_tile(&mut self, location: (usize, usize), tile: Tile) {
        self.tiles[location.1][location.0] = tile;
    }

    fn read_tile(&self, location: (usize, usize)) -> &Tile {
        &self.tiles[location.1][location.0]
    }

    fn modify_tile(&mut self, location: (usize, usize)) -> &mut Tile {
        &mut self.tiles[location.1][location.0]
    }

    fn display(&self, show_energized: bool) {
        for y in 0..self.actual_size {
            for x in 0..self.actual_size {
                let tile = self.read_tile((x, y));
                let mut char = tile.tile_type.as_char();
                if show_energized && tile.energized {
                    char = '#';
                }
                print!("{char}");
            }
            println!();
        }
    }

    fn out_of_bounds(&self, vector: &VectorI16) -> bool {
        vector.x < 0
            || vector.y < 0
            || vector.x >= self.actual_size as i16
            || vector.y >= self.actual_size as i16
    }

    fn ray_trace(&mut self, ray: Ray) -> u64 {
        if self.out_of_bounds(&ray.location) {
            return 0;
        }

        let (x, y) = (ray.location.x as usize, ray.location.y as usize);
        let tile = self.modify_tile((x, y));

        if tile.visits.visit(ray.direction) {
            return 0;
        }

        let mut result = if tile.energized { 0 } else { 1 };
        for mut new_ray in tile.process_ray(ray) {
            new_ray.step();
            result += self.ray_trace(new_ray);
        }
        result
    }

    fn reset(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                tile.energized = false;
                tile.visits = Visits::new();
            }
        }
    }

    fn get_starter_rays(&self) -> Vec<Ray> {
        let mut result = Vec::new();

        for x in 0..self.actual_size {
            result.push(Ray::from((0, 1), (x, 0)));
            result.push(Ray::from((0, -1), (x, self.actual_size)));
        }

        for y in 0..self.actual_size {
            result.push(Ray::from((1, 0), (0, y)));
            result.push(Ray::from((-1, 0), (self.actual_size, y)));
        }

        result
    }

    fn ray_trace_all(self) -> u64 {
        let mut result = 0;
        let mut best_grid = self;
        let mut grid = self;

        for ray in grid.get_starter_rays() {
            let value = grid.ray_trace(ray);
            grid.display(true);
            println!("v: {value}");
            if value > result {
                result = value;
                best_grid = self;
            }
            grid.reset();
        }

        best_grid.display(true);
        result
    }
}

fn load_data(path: &str) -> Result<Grid, Box<dyn Error>> {
    let lines = reader::get_lines(path)?;

    let mut grid = Grid::new();
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid.set_tile(
                (x, y),
                match char {
                    '.' => Tile::new(TileType::Empty),
                    '\\' => Tile::new(TileType::MirrorLeft),
                    '/' => Tile::new(TileType::MirrorRight),
                    '|' => Tile::new(TileType::SplitterVertical),
                    '-' => Tile::new(TileType::SplitterHorizontal),
                    _ => panic!("Unexpected character in data file! [{char}]"),
                },
            );
        }
        grid.actual_size = y + 1;
    }

    Ok(grid)
}

fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let mut grid = load_data(data_path)?;
    //let result = grid.ray_trace(Ray::starter_ray());
    let result = grid.ray_trace_all();

    /*
    grid.display(false);
    println!();
    grid.display(true);*/
    Ok(result)
}

fn main() {
    match calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
}

#[test]
fn calculate_test() {
    let expected_value = 51;
    match calculate("testdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Program finished but result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Error occured:\n{}", err),
    }
}

/*

/
    > 1,0 | ^ 0,-1
    v 0,1 | < -1,0
    < -1,0 | v 0,1
    ^ 0,-1 | > 1,0

Swap x and y and multiply by -1

\
    > 1,0 | v 0,1
    ^ 0,-1 | < -1,0
    < -1,0 | ^ 0,-1

Swap x and y


|
    if x != 0 create two in both directions of y

-
    if y != 0 create two in both directions of x

*/
