use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign, Neg, Sub},
};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}
impl Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
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
impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, other: Coord) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl Neg for Coord {
    type Output = Coord;
    fn neg(self) -> Coord {
        Coord {
            x: -self.x,
            y: -self.y,
        }
    }
}

type CharCoords = Vec<(Coord, char)>;
fn get_coords(input: &str) -> CharCoords {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        Coord {
                            x: x as isize,
                            y: y as isize,
                        },
                        c,
                    )
                })
                .collect::<Vec<(Coord, char)>>()
        })
        .collect::<Vec<(Coord, char)>>()
}

// Needs:
// A hashmap of all the letters and their coordinates
// A hashset of the antinode coordinates
// A way to calculate the distance between two nodes of the same letter (frequency)
// So, for each letter (frequency) pair, calculate distance to all possible antinodes -> insert into HashSet

pub fn part_one(input: &str) -> Option<u32> {
    let nodes_grouped_by_character = shared_setup(input);
    let x_bounds = 0..input.lines().next().unwrap().chars().count() as isize;
    let y_bounds = 0..input.lines().count() as isize;
    let mut antinodes = HashSet::new();

    // Now that the nodes are grouped by letter, we can iterate over each combination of coords for a given "frequnecy" (character)
    for (_character, coords) in nodes_grouped_by_character {
        for (a, b) in coords.iter().tuple_combinations() {
            // Calculate Manhattan distance between the two nodes
            let delta = *b - *a; // The vectored distance between the two nodes
            antinodes.insert(*a - delta); // The first antinode coord is the node minus the distance
            antinodes.insert(*b + delta); // The second antinode coord is the node plus the distance
        }
    }
    // Retain the antinodes that are inside the grid
    antinodes.retain(|coord| x_bounds.contains(&coord.x) && y_bounds.contains(&coord.y));

    Some(antinodes.len() as u32)
}

fn shared_setup(input: &str) -> HashMap<char, Vec<Coord>> {
    let coords = get_coords(input);

    let nodes_grouped_by_character = coords
        .into_iter()
        .filter(|(_, c)| c.is_alphanumeric())
        .fold(HashMap::new(), |mut map, (coord, c)| {
            map.entry(c).or_insert(vec![]).push(coord);
            map
        });

    nodes_grouped_by_character
}

pub fn part_two(input: &str) -> Option<u32> {
    let nodes_grouped_by_character = shared_setup(input);
    let x_bounds = 0..input.lines().next().unwrap().chars().count() as isize;
    let y_bounds = 0..input.lines().count() as isize;
    let mut antinodes = HashSet::new();

    // Now that the nodes are grouped by letter, we can iterate over each combination of coords for a given "frequnecy" (character)
    for (_character, coords) in nodes_grouped_by_character {
        for (a, b) in coords.iter().tuple_combinations() {
            // Calculate Manhattan distance between the two nodes
            let delta = *b - *a; // The vectored distance between the two nodes
                                 // Keep checking every node in that vector to see if it's an antinode
            let mut potential_antinode = *a;
            while x_bounds.contains(&potential_antinode.x)
                && y_bounds.contains(&potential_antinode.y)
            {
                antinodes.insert(potential_antinode);
                potential_antinode += delta;
            }

            let delta = -delta;
            let mut potential_antinode = *a;
            while x_bounds.contains(&potential_antinode.x)
                && y_bounds.contains(&potential_antinode.y)
            {
                antinodes.insert(potential_antinode);
                potential_antinode += delta;
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
