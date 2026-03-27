use std::collections::HashSet;

use super::vector::VectorI16;
use a_star_pathfinder::{AllPathsFinder, PathFinderError, Paths};

pub const GRIDSIZE: usize = 141;
const STARTDIRECTION: Direction = Direction(VectorI16 { x: 1, y: 0 });

#[derive(Copy, Clone)]
pub enum Tile {
    Wall,
    Empty,
}

impl Tile {
    fn is_empty(&self) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Empty => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Ord, Eq)]
pub struct Direction(VectorI16);

impl Direction {
    fn east() -> Self {
        Self(VectorI16::from((1, 0)))
    }
    fn south() -> Self {
        Self(VectorI16::from((0, 1)))
    }
    fn west() -> Self {
        Self(VectorI16::from((-1, 0)))
    }
    fn north() -> Self {
        Self(VectorI16::from((0, -1)))
    }

    fn next(self) -> Self {
        match self.0 {
            VectorI16 { x: 1, y: 0 } => Direction::south(),
            VectorI16 { x: 0, y: 1 } => Direction::west(),
            VectorI16 { x: -1, y: 0 } => Direction::north(),
            VectorI16 { x: 0, y: -1 } => Direction::east(),
            _ => panic!("Called .next() on an invalid direction! Was it changed externally?"),
        }
    }

    fn previous(self) -> Self {
        match self.0 {
            VectorI16 { x: 1, y: 0 } => Direction::north(),
            VectorI16 { x: 0, y: 1 } => Direction::east(),
            VectorI16 { x: -1, y: 0 } => Direction::south(),
            VectorI16 { x: 0, y: -1 } => Direction::west(),
            _ => panic!(
                "Called .previous() on an invalid direction {:?}! Was it changed externally?",
                self.0
            ),
        }
    }

    fn char(&self) -> char {
        match self.0 {
            VectorI16 { x: 1, y: 0 } => '>',
            VectorI16 { x: 0, y: 1 } => 'v',
            VectorI16 { x: -1, y: 0 } => '<',
            VectorI16 { x: 0, y: -1 } => '^',
            _ => panic!(
                "Called .char() on an invalid direction {:?}! Was it changed externally?",
                self.0
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Ord, Eq)]
pub struct Node {
    facing: Direction,
    position: VectorI16,
}

pub struct Maze {
    grid: [[Tile; GRIDSIZE]; GRIDSIZE],
    size_override: usize,
    start: Node,
    goal: Node,
}

impl Maze {
    /*pub fn has_path(&self) -> bool {
        !self.path.is_empty()
    }*/

    pub fn size_override(&self) -> usize {
        self.size_override
    }

    pub fn out_ouf_bounds(&self, position: VectorI16) -> bool {
        position.x < 0
            || position.x as usize >= self.size_override
            || position.y < 0
            || position.y as usize >= self.size_override
    }

    pub fn read_tile(&self, position: VectorI16) -> Option<Tile> {
        if self.out_ouf_bounds(position) {
            println!("out_ouf_bounds! {:?}", position);
            return None;
        }
        Some(self.grid[position.x as usize][position.y as usize])
    }

    fn get_neighbours_closure(&self) -> impl Fn(&Node) -> Vec<(Node, u64)> + '_ {
        |node: &Node| {
            let mut neighbours = Vec::with_capacity(3);

            // Move forward 1 step.
            if let Some(tile) = self.read_tile(node.position + node.facing.0) {
                if tile.is_empty() {
                    neighbours.push((
                        Node {
                            position: node.position + node.facing.0,
                            facing: node.facing,
                        },
                        1,
                    ));
                }
            }

            // Turn clockwise 90 degrees.
            if let Some(tile) = self.read_tile(node.position + node.facing.next().0) {
                if tile.is_empty() {
                    neighbours.push((
                        Node {
                            position: node.position,
                            facing: node.facing.next(),
                        },
                        1000,
                    ));
                }
            }

            // Turn counter clockwise 90 degrees.
            if let Some(tile) = self.read_tile(node.position + node.facing.previous().0) {
                if tile.is_empty() {
                    neighbours.push((
                        Node {
                            position: node.position,
                            facing: node.facing.previous(),
                        },
                        1000,
                    ));
                }
            }

            neighbours
        }
    }

    fn get_heuristic_closure(&self) -> impl Fn(&Node) -> u64 + '_ {
        //
        |node: &Node| 0
    }

    fn get_is_goal_closure(&self) -> impl Fn(&Node) -> bool + '_ {
        //
        |node: &Node| node.position == self.goal.position
    }

    pub fn calculate_path(&mut self) -> Result<Paths<Node>, PathFinderError> {
        let neighbours_closure = self.get_neighbours_closure();
        let heuristic_closure = self.get_heuristic_closure();
        let is_goal_closure = self.get_is_goal_closure();

        println!(
            "Stats: Start: {:?},\nGoal: {:?},\nSize override: {:?}",
            self.start.position, self.goal.position, self.size_override
        );

        let mut pathfinder = AllPathsFinder::new(self.start, &heuristic_closure);

        //let path = pathfinder.calculate_path(neighbours_closure, heuristic_closure, is_goal_closure)?;
        let paths = match pathfinder.calculate_all_optimal_paths(
            neighbours_closure,
            heuristic_closure,
            is_goal_closure,
        ) {
            Ok(result) => result,
            Err(err) => {
                //println!("Nodes searched: {}", pathfinder.view().0.len());
                return Err(err);
            }
        };

        Ok(paths)
    }

    pub fn get_display_grid(&self, paths: &Paths<Node>) -> [[char; GRIDSIZE]; GRIDSIZE + 1] {
        let mut display_grid: [[char; GRIDSIZE]; GRIDSIZE + 1] = [['#'; GRIDSIZE]; GRIDSIZE + 1];

        println!("so: {}", self.size_override);
        for y in 0..self.size_override {
            for x in 0..self.size_override {
                display_grid[x][y] = match self.grid[x][y] {
                    Tile::Wall => '#',
                    Tile::Empty => ' ',
                }
            }
            display_grid[self.size_override][y] = '\n';
        }

        for path in paths.all_paths() {
            for node in path.nodes.iter().rev() {
                display_grid[node.position.x as usize][node.position.y as usize] =
                    node.facing.char();
            }
        }

        display_grid[self.start.position.x as usize][self.start.position.y as usize] = 'S';
        display_grid[self.goal.position.x as usize][self.goal.position.y as usize] = 'E';

        display_grid
    }

    pub fn optimal_seats(&self, paths: Paths<Node>) -> u64 {
        let mut path_tiles: HashSet<VectorI16> = HashSet::new();
        for path in paths.all_paths() {
            for node in path.nodes {
                //
                path_tiles.insert(node.position);
            }
        }

        path_tiles.len() as u64
    }
}

pub struct MazeBuilder {
    grid: [[Tile; GRIDSIZE]; GRIDSIZE],
    size_override: usize,
    start_position: VectorI16,
    goal_position: VectorI16,
}

impl MazeBuilder {
    pub fn new() -> Self {
        Self {
            grid: [[Tile::Wall; GRIDSIZE]; GRIDSIZE],
            size_override: 0,
            start_position: VectorI16::default(),
            goal_position: VectorI16::default(),
        }
    }

    pub fn grid(mut self, grid: [[Tile; GRIDSIZE]; GRIDSIZE]) -> Self {
        self.grid = grid;
        self
    }

    pub fn size_override(mut self, size_override: usize) -> Self {
        self.size_override = size_override;
        self
    }

    pub fn start(mut self, start: VectorI16) -> Self {
        self.start_position = start;
        self
    }

    pub fn goal(mut self, goal: VectorI16) -> Self {
        self.goal_position = goal;
        self
    }

    pub fn build(&self) -> Maze {
        Maze {
            grid: self.grid,
            size_override: self.size_override,
            start: Node {
                position: self.start_position,
                facing: STARTDIRECTION,
            },
            goal: Node {
                position: self.goal_position,
                facing: STARTDIRECTION,
            },
        }
    }
}
