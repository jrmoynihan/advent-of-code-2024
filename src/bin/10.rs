use grid::Grid;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
    rc::Rc,
    sync::{Arc, Mutex},
};

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    row: isize,
    col: isize,
    value: u32,
}
impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord {
            row: self.row + other.row,
            col: self.col + other.col,
            value: self.value,
        }
    }
}
impl Coord {
    fn get_adjacent_perpendicular_neighbors(&self, grid: &Grid<u32>) -> Vec<Coord> {
        let mut neighbors = Vec::new();
        let down = *self + Direction::Down.to_coord();
        let up = *self + Direction::Up.to_coord();
        let left = *self + Direction::Left.to_coord();
        let right = *self + Direction::Right.to_coord();

        if let Some(v) = grid.get(down.row, down.col) {
            neighbors.push(Coord {
                row: down.row,
                col: down.col,
                value: *v,
            });
        }
        if let Some(v) = grid.get(up.row, up.col) {
            neighbors.push(Coord {
                row: up.row,
                col: up.col,
                value: *v,
            });
        }
        if let Some(v) = grid.get(left.row, left.col) {
            neighbors.push(Coord {
                row: left.row,
                col: left.col,
                value: *v,
            });
        }
        if let Some(v) = grid.get(right.row, right.col) {
            neighbors.push(Coord {
                row: right.row,
                col: right.col,
                value: *v,
            });
        }

        neighbors
    }
}
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn to_coord(&self) -> Coord {
        match self {
            Direction::Up => Coord {
                row: 0,
                col: -1,
                value: 0,
            },
            Direction::Down => Coord {
                row: 0,
                col: 1,
                value: 0,
            },
            Direction::Left => Coord {
                row: -1,
                col: 0,
                value: 0,
            },
            Direction::Right => Coord {
                row: 1,
                col: 0,
                value: 0,
            },
        }
    }
}

#[derive(Debug)]
struct QueueItem {
    coord: Coord,
    path: Vec<Coord>,
}

pub fn shared(input: &str, find_all_paths: bool) -> Vec<Vec<Coord>> {
    let cols = input.lines().next().unwrap().len();
    let input = input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect_vec();
    let grid = Grid::from_vec(input, cols);
    let mut all_paths = Vec::new();

    // Breadth-first search approach
    grid.indexed_iter().for_each(|(coord, v)| {
        if v == &0 {
            let mut queue = VecDeque::new();
            let coord = Coord {
                row: coord.0 as isize,
                col: coord.1 as isize,
                value: *v,
            };

            queue.push_back(QueueItem {
                coord,
                path: vec![coord],
            });

            let mut visited = HashSet::new();
            if !find_all_paths {
                visited.insert(coord);
            }

            while let Some(item) = queue.pop_front() {
                // println!("Current item: {:?}", item);
                // If we've reached 9, we've found a complete path
                if item.coord.value == 9 {
                    // println!("Found complete path: {:?}", item.path);
                    all_paths.push(item.path);
                    continue;
                }
                // Look for the next number in sequence
                let target_value = item.coord.value + 1;

                let neighbors = item.coord.get_adjacent_perpendicular_neighbors(&grid);
                // println!("Neighbors: {:?}", neighbors);
                for neighbor in neighbors {
                    // println!(
                    //     "Neighbor ({}) == target_value ({}): {:?}",
                    //     neighbor.value,
                    //     target_value,
                    //     neighbor.value == target_value
                    // );
                    // println!("Neighbor visited: {:?}", visited.contains(&neighbor));

                    if neighbor.value == target_value {
                        if !find_all_paths && !visited.contains(&neighbor) {
                            visited.insert(neighbor);
                            let mut new_path = item.path.clone();
                            new_path.push(neighbor);
                            queue.push_back(QueueItem {
                                coord: neighbor,
                                path: new_path,
                            });
                        } else if find_all_paths {
                            let mut new_path = item.path.clone();
                            new_path.push(neighbor);
                            queue.push_back(QueueItem {
                                coord: neighbor,
                                path: new_path,
                            });
                        }
                        // println!("Pushing neighbor to queue: {:?}", neighbor);
                    }
                }
                // println!("Queue:");
                // for item in queue.iter() {
                //     println!("path (length {}): {:?}", item.path.len(), item.path);
                // }
            }
        }
    });
    // let guard = all_paths.lock().unwrap();
    // let paths = guard.to_vec();
    all_paths
}

pub fn part_one(input: &str) -> Option<usize> {
    let all_paths = shared(input, false);
    Some(all_paths.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let all_paths = shared(input, true);
    let ratings = all_paths.len();
    Some(ratings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
