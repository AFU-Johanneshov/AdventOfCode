use std::io;

mod reader;
use reader::get_lines;

mod vector;
use vector::Vector;

mod testing_debug;

const GRIDSIZE: usize = 140;

struct PlotMap {
    map: [[Plot; GRIDSIZE]; GRIDSIZE],
    size: usize,
}

impl PlotMap {
    fn get_plot(&self, coordinates: Vector) -> Result<Plot, AdventError> {
        if coordinates.out_of_bounds(self.size) {
            Err(AdventError::OutOfBounds(String::from(
                "Requested plot at {coordinates:?} is outside of the grid!",
            )))
        } else {
            Ok(self.map[coordinates.x as usize][coordinates.y as usize])
        }
    }

    fn set_plot(&mut self, coordinates: Vector, plot: Plot) -> Result<bool, AdventError> {
        if coordinates.out_of_bounds(self.size) {
            Err(AdventError::OutOfBounds(String::from(
                "Plot placement at {coordinates:?} is outside of the grid!",
            )))
        } else {
            self.map[coordinates.x as usize][coordinates.y as usize] = plot;
            Ok(true)
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Plot {
    plant: char,
    in_region: bool,
}

#[derive(Debug)]
struct Region {
    area: u16,
    corners: u16,
    plant: char,
}

#[derive(Debug)]
enum AdventError {
    IoError(io::Error),
    OutOfBounds(String),
}

impl From<io::Error> for AdventError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

fn get_map(path: &str) -> Result<PlotMap, AdventError> {
    let lines = get_lines(path)?;
    let mut plot_map = PlotMap {
        map: [[Plot {
            plant: ' ',
            in_region: false,
        }; GRIDSIZE]; GRIDSIZE],
        size: GRIDSIZE,
    };

    let mut size: usize = 0;

    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            let _ = plot_map.set_plot(
                Vector::from((x, y)),
                Plot {
                    plant: char,
                    in_region: false,
                },
            )?;
        }
        size = y;
    }

    plot_map.size = size + 1;

    Ok(plot_map)
}

fn scan_region(
    map: &mut PlotMap,
    coordinates: Vector,
    plant: char,
) -> Result<(u16, u16), AdventError> {
    let mut plot: Plot = map.get_plot(coordinates)?;
    plot.in_region = true;
    let _ = map.set_plot(coordinates, plot)?;

    let (mut area, mut corners) = (1, 0);
    corners += get_corners(coordinates, map, plant);

    let (result_area, result_corners) = scan_neighbours(coordinates, map, plant)?;
    area += result_area;
    corners += result_corners;

    Ok((area, corners))
}

fn get_corners(coordinates: Vector, map: &PlotMap, plant: char) -> u16 {
    let mut corners: u16 = 0;

    // Set perimeter_to_neighbour based on the plot to the left to make sure the first neighbour is
    // compared to the last one. With the loop approach we would otherwise only compare 3 times
    // instead of 4.
    let directions: [Vector; 4] = Vector::directions();
    let mut perimeter_to_neighbour: bool = match map.get_plot(coordinates + directions[3]) {
        Ok(plot) => plot.plant != plant,
        Err(_) => true,
    };
    for (direction_index, neighbour) in directions.iter().enumerate() {
        let Ok(plot) = map.get_plot(*neighbour) else {
            if perimeter_to_neighbour {
                corners += 1;
            }
            perimeter_to_neighbour = true;
            continue;
        };

        let perimeter_found: bool = plot.plant != plant;
        let perform_inwards_corner_scan: bool = perimeter_found != perimeter_to_neighbour;

        if perimeter_found {
            if perimeter_to_neighbour {
                corners += 1;
            }
            perimeter_to_neighbour = true;
        } else {
            perimeter_to_neighbour = false;
        }

        if perform_inwards_corner_scan {
            corners += corner_scan(*neighbour, direction_index, map, plant);
        }
    }
    corners
}

fn scan_neighbours(
    coordinates: Vector,
    map: &mut PlotMap,
    plant: char,
) -> Result<(u16, u16), AdventError> {
    let (mut area, mut corners) = (0, 0);
    for neighbour in coordinates.neighbours() {
        let Ok(plot) = map.get_plot(neighbour) else {
            continue;
        };
        if plot.in_region || plot.plant != plant {
            continue;
        }

        let (result_area, result_corners) = scan_region(map, neighbour, plant)?;
        area += result_area;
        corners += result_corners;
    }
    Ok((area, corners))
}

fn corner_scan(neighbour: Vector, direction_index: usize, map: &PlotMap, plant: char) -> u16 {
    let diagonal_neighbour = neighbour
        + Vector::directions()[match direction_index {
            0 => 3,
            _ => direction_index - 1,
        }];

    let Ok(plot) = map.get_plot(diagonal_neighbour) else {
        return 0;
    };
    if plot.plant == plant && !plot.in_region {
        1
    } else {
        0
    }
}

fn calculate_cost(regions: Vec<Region>) -> u64 {
    let mut result: u64 = 0;

    for region in regions {
        result += region.corners as u64 * region.area as u64;
    }
    result
}

fn calculate(path: &str) -> Result<u64, AdventError> {
    let mut plot_map = get_map(path)?;

    let mut regions: Vec<Region> = Vec::new();

    for y in 0..plot_map.size {
        for x in 0..plot_map.size {
            let plot = plot_map.get_plot(Vector::from((x, y)))?;
            if plot.in_region {
                continue;
            }

            let (area, corners) = scan_region(&mut plot_map, Vector::from((x, y)), plot.plant)?;
            regions.push(Region {
                area,
                corners,
                plant: plot.plant,
            });
        }
    }

    Ok(calculate_cost(regions))
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
        Ok(value) => assert_eq!(value, 1206),
    }
}

#[test]
fn calculate_test_small() {
    match calculate("smalltestdata.txt") {
        Err(err) => panic!("An error occured: {err:?}"),
        Ok(value) => assert_eq!(value, 80),
    }
}

/* Sudo code:

Challenge part 1:
The main challenge is, how do we actually count the perimeter without buying fences for the same border twice?
Although it is not clear if that is actually required.

My thought process is that we iterate through the plot map one plot at a time.
If the current plot is part of a region then do nothing and go to the next.
else
call scan with current plot coordinates and add the result as a region to a list of found regions.
scan(coordinates, plant) -> (area, perimeter)
    coordinates.plot.in_region = true
    area = 1
    perimeter = 0
    for each direction:
        if plot.plant != plant
            add 1 to perimeter and continue.
        else if plot.in_region == true
            do nothing and continue.
        else
            call scan with plot and plant and add the result to area and perimeter.

    return area and perimeter.

after iteration is done go through the list and process the cost for all the fences.


Challenge part 2:

Instead of counting the perimeter lenght we need to count the amount of sides the region has.

Potential solutions:

---------------------------------------------------------------------------------------------------------------------------------------------

# Side scan
Add a hashmap to each scan phase. (HashMap: Key: vector, Value: (horizontal: bool, vertical: bool))
When a perimeter is found during the scan phase run a side scan with from that location parallel with that perimeter.
Edit the hashmap with the resulting vector as the key. Set the horizontal or vertical to true depending on the direction
of the side scan.
Side scan:
Decrease the location vector by one in the side scan angle. (horizontal/vertical)
Check if the location still is the same plant. If not return the previous vector.
Check if the location at the other side of the perimeter direction is still NOT the same plant.
If not then return the previous vector.
Repeat until either of the conditions fail OR the edge of the grid is reached.

+ Should be able to handle any edge cases and get the correct side count every time.
- Not the fastest or most efficient. Will perform one full recursive side scan for each perimeter found.

---------------------------------------------------------------------------------------------------------------------------------------------

# Corner scan
Count the total amount of corners instead of following each side.

Corner scan:
If plot has at least 1 perimeter side:
// Outwards corners
Check all four neighbours. For each connection between potential perimeters to the neighbours add 1 to corners.
// Inwards corners:
For any perimeter direction which does not connect to another perimeter:
    Get the vector of the neighbour on the other side of the parameter.
    Move one step over that vector in the direction in question.
    Get the plot at that location.
    If plot is of same plant as the source plot AND plot is NOT in a region:
        Add one to corners
    else do nothing.

// The reason the inwards corner can be done in this way is thanks to the in_region plot variable.
// Each inward corner has with this search logic TWO plots that will find the same corner. But as
// long as the corner processing is done before the recursive region scan continues it will always
// only find the corner once even though the corner scan will check it twice. Since if the corner
// has been found already, the next scan will check the same plot that the first corner scan found
// it from. Which will be marked as in-region.


+ Much more efficient. Still needs to do processing at each perimeter, but with a constant time complexity regardless of data file size.
+ Less memory usage since there is no need to cache corners to prevent double counting.

---------------------------------------------------------------------------------------------------------------------------------------------


*/
