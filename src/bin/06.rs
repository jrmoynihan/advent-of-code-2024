use std::{collections::HashSet, ops::Add};

advent_of_code::solution!(6);

type Grid = Vec<Vec<char>>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}
impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn turn(&mut self) {
        match self {
            Direction::North => *self = Direction::East,
            Direction::East => *self = Direction::South,
            Direction::South => *self = Direction::West,
            Direction::West => *self = Direction::North,
        }
    }
    fn next_step(&self) -> Coord {
        match self {
            Direction::North => Coord { x: 0, y: -1 },
            Direction::East => Coord { x: 1, y: 0 },
            Direction::South => Coord { x: 0, y: 1 },
            Direction::West => Coord { x: -1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    coord: Coord,
    direction: Direction,
}
impl Location {
    fn next_step(&self) -> Location {
        match self.direction {
            Direction::North => Location {
                coord: self.coord + Direction::North.next_step(),
                direction: self.direction,
            },
            Direction::East => Location {
                coord: self.coord + Direction::East.next_step(),
                direction: self.direction,
            },
            Direction::South => Location {
                coord: self.coord + Direction::South.next_step(),
                direction: self.direction,
            },
            Direction::West => Location {
                coord: self.coord + Direction::West.next_step(),
                direction: self.direction,
            },
        }
    }
    fn turn(&mut self) -> &mut Self {
        self.direction.turn();
        self
    }
}
#[derive(Debug)]
struct State {
    grid: Grid,
    current_location: Location,
    next_location: Location,
    distinct_traveled_locations: HashSet<Location>,
    distinct_obstacles: HashSet<Location>,
}
impl State {
    fn new(grid: Grid, start_location: Location) -> Self {
        Self {
            grid,
            distinct_traveled_locations: HashSet::from([Location {
                coord: start_location.coord,
                direction: Direction::North,
            }]),
            distinct_obstacles: HashSet::new(),
            next_location: start_location,
            current_location: start_location,
        }
    }

    fn reset(&mut self) {
        let start = find_start(&self.grid).unwrap();
        self.distinct_traveled_locations.clear();
        self.distinct_obstacles.clear();
        self.current_location = Location {
            coord: start,
            direction: Direction::North,
        };
    }
    fn is_next_cell_beyond_grid(&mut self) -> bool {
        self.next_location = self.current_location.next_step();
        
        self.next_location.coord.x < 0
            || self.next_location.coord.y < 0
            || self.next_location.coord.x >= self.grid[0].len() as isize
            || self.next_location.coord.y >= self.grid.len() as isize
    }

    fn traverse(&mut self) -> bool {
        while !self.is_next_cell_beyond_grid() {
            let looped = self.try_step();
            if looped {
                return true;
            }
        }
        false
    }

    fn is_next_cell_obstacle(&mut self) -> bool {
        self.grid[self.next_location.coord.y as usize][self.next_location.coord.x as usize] == '#'
    }
    fn is_loop(&mut self) -> bool {
        self.distinct_obstacles.contains(&self.next_location)
    }
    fn try_step(&mut self) -> bool {
        //Check if the next coord is an obstacle
        if self.is_next_cell_obstacle() {
            if self.is_loop() {
                return true;
            } else {
                self.distinct_obstacles.insert(self.next_location);
            }
            self.current_location.turn();
        } else {
            self.current_location = self.next_location;
            self.distinct_traveled_locations.insert(self.next_location);
        }
        false
    }
    fn add_obstacle(&mut self, location: &Location) -> char {
        let previous = self.grid[location.coord.y as usize][location.coord.x as usize];
        self.grid[location.coord.y as usize][location.coord.x as usize] = '#';
        previous
    }
    fn remove_obstacle(&mut self, location: &Location, previous: char) {
        self.grid[location.coord.y as usize][location.coord.x as usize] = previous;
    }
}

fn make_grid(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}
fn find_start(grid: &Vec<Vec<char>>) -> Option<Coord> {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '^' {
                return Some(Coord {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = make_grid(input);
    let current_location: Location = Location {
        coord: find_start(&grid).unwrap(),
        direction: Direction::North,
    };
    println!("{:?}", current_location);
    let mut state = State::new(grid, current_location);
    state.traverse();
    // Filter to a unique set of *coordinates* in the distinct_locations (irrespective of direction facing while traversing)
    let unique_locations: HashSet<Coord> = state
        .distinct_traveled_locations
        .iter()
        .map(|l| l.coord)
        .collect();
    Some(unique_locations.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = make_grid(input);
    let current_location: Location = Location {
        coord: find_start(&grid).unwrap(),
        direction: Direction::North,
    };
    let mut state = State::new(grid, current_location);
    state.traverse();
    // Make a copy of the distinct positions
    let mut locations: Vec<Location> = state.distinct_traveled_locations.iter().cloned().collect();
    // Find the index of the original guard position
    let guard_position = locations
        .iter()
        .position(|p| p == &current_location)
        .unwrap();
    // Remove the original guard position
    locations.remove(guard_position);
    // Sort the positions
    locations.sort_by(|a, b| a.coord.x.cmp(&b.coord.x).then(a.coord.y.cmp(&b.coord.y)));

    // for (i, loc) in locations.iter().enumerate() {
    //     println!("{}: {:?}", i, loc);
    // }

    let mut looped_locations: HashSet<Coord> = HashSet::new();
    for location in &locations {
        state.reset();
        let original_char = state.add_obstacle(location);
        let looped = state.traverse();
        state.remove_obstacle(location, original_char);
        if looped {
            looped_locations.insert(location.coord);
        }
    }
    // for (i, loc) in looped_locations.iter().enumerate() {
    //     println!("looped - {}: {:?}", i, loc);
    // }
    Some(looped_locations.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
