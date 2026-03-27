mod reader;
use reader::get_lines;

const GRIDSIZE: usize = 130;

#[derive(Copy, Clone)]
enum Cell {
    Obstacle(Collisions),
    Floor { visited: bool, block_checked: bool },
}

#[derive(Copy, Clone)]
struct Collisions {
    below: u16,
    above: u16,
    left: u16,
    right: u16,
}

impl Collisions {
    fn collide(&mut self, guard: &Guard) -> CollisionResult {
        match guard.dir {
            (0, 1) => {
                if self.below == guard.id {
                    CollisionResult::DuplicateCollision
                } else {
                    self.below = guard.id;
                    CollisionResult::NewCollision
                }
            }
            (1, 0) => {
                if self.left == guard.id {
                    CollisionResult::DuplicateCollision
                } else {
                    self.left = guard.id;
                    CollisionResult::NewCollision
                }
            }
            (0, -1) => {
                if self.above == guard.id {
                    CollisionResult::DuplicateCollision
                } else {
                    self.above = guard.id;
                    CollisionResult::NewCollision
                }
            }
            (-1, 0) => {
                if self.right == guard.id {
                    CollisionResult::DuplicateCollision
                } else {
                    self.right = guard.id;
                    CollisionResult::NewCollision
                }
            }
            (_, _) => panic!("Invalid direction received for collision!"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Guard {
    x: usize,
    y: usize,
    dir: (i32, i32),
    id: u16,
}

enum CollisionResult {
    NewCollision,
    DuplicateCollision,
}

enum MoveResult {
    NewVisit,
    ExitedWorld,
    Collision(CollisionResult),
    None,
}

impl Guard {
    fn take_step(&mut self, world_grid: &mut [[Cell; GRIDSIZE]; GRIDSIZE]) -> MoveResult {
        //

        let next_pos = self.next_position();

        if next_pos.0 >= GRIDSIZE || next_pos.1 >= GRIDSIZE {
            return MoveResult::ExitedWorld;
        }

        match world_grid[next_pos.0][next_pos.1] {
            Cell::Obstacle(mut collisions) => {
                let collision_result = collisions.collide(self);
                world_grid[next_pos.0][next_pos.1] = Cell::Obstacle(collisions);
                self.change_direction();
                MoveResult::Collision(collision_result)
            }
            Cell::Floor {
                visited,
                block_checked,
            } => {
                self.x = next_pos.0;
                self.y = next_pos.1;
                if visited {
                    MoveResult::None
                } else {
                    world_grid[next_pos.0][next_pos.1] = Cell::Floor {
                        visited: true,
                        block_checked,
                    };
                    MoveResult::NewVisit
                }
            }
        }
    }

    fn change_direction(&mut self) {
        self.dir = match self.dir {
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            (_, _) => panic!("Direction was manually changed to a invalid value!"),
        }
    }

    fn next_position(&self) -> (usize, usize) {
        (
            (self.x as i32 + self.dir.0) as usize,
            (self.y as i32 + self.dir.1) as usize,
        )
    }

    fn look_ahead(&self, world_grid: &[[Cell; GRIDSIZE]; GRIDSIZE]) -> Option<Cell> {
        let next_pos = self.next_position();

        if next_pos.0 >= GRIDSIZE || next_pos.1 >= GRIDSIZE {
            return None;
        }

        Some(world_grid[next_pos.0][next_pos.1])
    }

    fn will_loop(&mut self, world_grid: &mut [[Cell; GRIDSIZE]; GRIDSIZE]) -> bool {
        // TODO:
        // The start state can not be relied on! A loop can occur later, meaning the start state
        // will never be revisited!
        // Need a way to keep track of which tiles has been visited and which haven't. This
        // includes the direction when on those tiles too! A loop might pass over the same tile
        // twice but from different directions.
        //
        // There is also a possibility we miss loops that bounce on other sides of our obstacle.
        // Create a new cell type: CustomObstacle which is placed by the main function and later
        // reset back to a Floor after that loop iteration.
        //let start_state: Guard = self.clone();

        loop {
            match self.take_step(world_grid) {
                MoveResult::None => {}
                MoveResult::NewVisit => {}
                MoveResult::Collision(collision_result) => {
                    if matches!(collision_result, CollisionResult::DuplicateCollision) {
                        return true;
                    }
                }
                MoveResult::ExitedWorld => return false,
            };

            //println!("Guard: {self:?}");
        }
    }
}

fn get_world() -> ([[Cell; GRIDSIZE]; GRIDSIZE], Guard) {
    let Ok(lines) = get_lines("./data.txt") else {
        panic!(); // Panic if file not found. Not viable for production code but for prototying it
                  // is fine.
    };

    let mut world_grid = [[Cell::Floor {
        visited: false,
        block_checked: false,
    }; GRIDSIZE]; GRIDSIZE];

    let mut guard: Guard = Guard {
        x: 0,
        y: 0,
        dir: (0, 0),
        id: 0,
    };

    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '^' => {
                    world_grid[x][GRIDSIZE - y - 1] = Cell::Floor {
                        visited: true,
                        block_checked: true,
                    };
                    guard = Guard {
                        x,
                        y: GRIDSIZE - y - 1,
                        dir: (0, 1),
                        id: 1,
                    }
                }
                '#' => {
                    world_grid[x][GRIDSIZE - y - 1] = Cell::Obstacle(Collisions {
                        above: 0,
                        below: 0,
                        left: 0,
                        right: 0,
                    })
                }
                _ => {}
            }
        }
    }

    (world_grid, guard)
}

fn main() {
    // Start here:

    let (mut world_grid, mut guard) = get_world();
    let mut visited_cells = 1; // This is 1 as default because the system doesn't count the first
                               // cell.
    let mut custom_obstacles_count = 0;
    let mut search_guard: Guard;

    let mut next_guard_id = 2;

    loop {
        if let Some(cell) = guard.look_ahead(&world_grid) {
            match cell {
                Cell::Floor {
                    visited,
                    block_checked,
                } => {
                    search_guard = guard;
                    search_guard.id = next_guard_id;
                    next_guard_id += 1;

                    let obstacle_location: (usize, usize) = search_guard.next_position();
                    let mut checked = block_checked;
                    world_grid[obstacle_location.0][obstacle_location.1] =
                        Cell::Obstacle(Collisions {
                            below: 0,
                            above: 0,
                            left: 0,
                            right: 0,
                        });

                    if search_guard.will_loop(&mut world_grid) && !block_checked {
                        custom_obstacles_count += 1;
                    }

                    world_grid[obstacle_location.0][obstacle_location.1] = Cell::Floor {
                        visited,
                        block_checked: true,
                    };
                }
                Cell::Obstacle(_) => {}
            };
        }

        match guard.take_step(&mut world_grid) {
            MoveResult::None => {}
            MoveResult::NewVisit => {
                visited_cells += 1;
            }
            MoveResult::Collision(..) => {}
            MoveResult::ExitedWorld => break,
        }

        //println!("Step {next_guard_id}, with {custom_obstacles_count} locations found...");
    }

    println!("There are {custom_obstacles_count} floor locations that if blocked would result in a loop.");
    println!("The guard visted {visited_cells} cells before leaving the area.");
}

/* Sudo code:

Challenge part 1:
Read data.txt into a grid of 130x130 cells where a cell is either a "obstacle" with a collisions struct (Used in part 2), or a "floor" with a tuple of (bool: visited, bool: block_checked (Used in part 2))
The guard spawn position is saved as a normal floor piece in the grid, but saved with visited set as true instead of the default false.
    A guard struct contains the data required for the guard movement, and is created when creating the grid.
    Guard
    {
        int x,
        int y,
        int xdir,
        int ydir,
        int id (Used in part 2)
    }

Once the grid is fully loaded:

loop
    set a variable to the guards position + the guards direction. (next_pos)
    if next_pos is out of bounds break loop.

    Get the cell at coordinates next_pos from the world grid.
        if a obstacle turn 90 degrees and continue with next iteration of loop. (Note the way you read the file and how the coordinates increase. Turning right has to be represented correctly from the readers perspective.)
        if a Floor
            set guard position to next_pos.
            if cell at next_pos has been visited:
                increase result total by one.
                set cell.visited at next_pos to true.

print result total to get the answer.



Challenge part 2:

collisions struct
{
    int above,
    int below,
    int left,
    int right,
}

When reading the grid and creating the guard set the guard id to 1.

Variable nextid = 2
variable custom_obstacles = 0

add to part 1 loop:

loop
    set a variable to the guards position + the guards direction. (next_pos)
    if next_pos is out of bounds break loop.

    if cell at next_pos is a floor:
        if cell at next_pos.block_checked is false
            cache cell.
            set cell at next pos to a new obstacle
            copy guard struct to a new variable (search_guard)
            set search_guard id to nextid++
            loop
                set a variable to search_guards position + its direction. (next_pos2)
                if next_pos2 is out of bounds break loop.
                if cell at next_pos2 is obstacle:
                    if cell.collisions.direction == search_guard.id // Bsically, if this guard has collided with the same obstacle from the same direction before we are now stuck in a lop.
                        increase custom_obstacles by one
                        break loop.

            reset cell at next_pos back to the cached cell.
            set cell.block_checked to true // This prevents the program from attempting to place a obstacle where it would have blocked the guard at a earlier point than which is being tested at the moment.




    /*
     * PART 1 CODE
    */

print custom_obstacles to get the answer.


*/
