use a_star_pathfinder::{PathFinderError, Paths};
use std::io;

mod reader;
use reader::get_lines;
mod maze;
use maze::{Maze, MazeBuilder, Node, Tile};
mod testing_debug;
mod vector;
use vector::VectorI16;

#[derive(Debug)]
enum AdventError {
    IoError(io::Error),
    CorruptData(String),
    GoalUnreachable(PathFinderError),
}

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<PathFinderError> for AdventError {
    fn from(err: PathFinderError) -> Self {
        Self::GoalUnreachable(err)
    }
}

fn get_maze(path: &str) -> Result<Maze, AdventError> {
    let lines = get_lines(path)?;

    let mut tile_grid = [[Tile::Wall; maze::GRIDSIZE]; maze::GRIDSIZE];
    let mut start = VectorI16::default();
    let mut goal = VectorI16::default();
    let mut size_override: usize = 0;
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => (), // Do nothing as a wall is already the default tile in the maze,
                '.' => tile_grid[x][y] = Tile::Empty,
                'S' => {
                    tile_grid[x][y] = Tile::Empty;
                    start = VectorI16::from((x, y));
                }
                'E' => {
                    tile_grid[x][y] = Tile::Empty;
                    goal = VectorI16::from((x, y));
                }
                _ => {
                    return Err(AdventError::CorruptData(format!(
                        "Encountered the unexpected character '{}' when reading the data file.",
                        char
                    )))
                }
            }
        }
        size_override = y + 1;
    }

    Ok(MazeBuilder::new()
        .grid(tile_grid)
        .start(start)
        .goal(goal)
        .size_override(size_override)
        .build())
}

fn display_maze(maze: &Maze, paths: &Paths<Node>) {
    let mut output_string =
        String::with_capacity((maze.size_override() * maze.size_override()) + maze.size_override());

    let maze_display_grid = maze.get_display_grid(paths);
    for y in 0..maze.size_override() {
        for x in 0..maze.size_override() + 1 {
            output_string.push(maze_display_grid[x][y]);
        }
    }

    println!("Maze: \n{}", output_string);
}

fn calculate(path: &str) -> Result<u64, AdventError> {
    let mut maze = get_maze(path)?;

    let paths = maze.calculate_path()?;

    display_maze(&maze, &paths);

    //todo!();
    Ok(maze.optimal_seats(paths))
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
        Ok(value) => assert_eq!(value, 64),
    }
}
#[test]
fn calculate_small_test() {
    match calculate("smalltestdata.txt") {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, 45),
    }
}

/* Sudo code:

Challenge part 1:

Pathfinder required!

Q1: How to handle duplicate paths? (Where a path ends up where it already have been)
//      The basic "don't return" policy might be the best option. This is where a a path only makes
//      sure it doesn't return to the node just before it.

Q2: Node format.
//      Each node should have a next function. Where the next function should return all
//      neighbouring nodes. Basically all potential movements from the current node.
//      An optimization would be to check beforehand if a neighbour is never going to result in a
//      valid path faster than the current.

Timeline:
1: Learn more about modules.
2: Create a new PathFinder module.
3: Learn more about traits.
4: The pathfinder module should have a node trait which is implemented on a type in the advent of code project. The node trait lets us use the pathfinding algorithm for any pathfinding task we want as long as we implement the required methods.

Challenge part 2:

Use new AllPathsFinder!

Calculate all the optimal paths, then iterate over them.
    Iterate through each path. Adding the vector of all connecting walls to the path into a hashset.

After all paths has been checked the length of the hashset is our result!


*/
