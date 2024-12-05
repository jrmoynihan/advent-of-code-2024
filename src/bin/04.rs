advent_of_code::solution!(4);

enum Direction {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}
impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::NorthWest => (-1, -1),
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
        }
    }
}

fn offset_coord(coord: (isize, isize), direction: &Direction, distance: usize) -> (isize, isize) {
    let offset = direction.offset();
    (
        coord.0 + (offset.0 * distance as isize),
        coord.1 + (offset.1 * distance as isize),
    )
}
fn make_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
fn make_directions() -> Vec<Direction> {
    vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthWest,
    ]
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = make_grid(input);
    let directions = make_directions();
    let mut count = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'X' {
                for direction in &directions {
                    let (mx, my) = offset_coord((x as isize, y as isize), direction, 1);
                    let (ax, ay) = offset_coord((x as isize, y as isize), direction, 2);
                    let (sx, sy) = offset_coord((x as isize, y as isize), direction, 3);
                    if mx >= 0
                        && my >= 0
                        && ax >= 0
                        && ay >= 0
                        && sx >= 0
                        && sy >= 0
                        && sx < grid[0].len() as isize
                        && sy < grid.len() as isize
                    {
                        if grid[my as usize][mx as usize] == 'M'
                            && grid[ay as usize][ax as usize] == 'A'
                            && grid[sy as usize][sx as usize] == 'S'
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    dbg!(&count);
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = make_grid(input);
    let directions = make_directions();
    let mut count = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'A' {
                let neighbors = directions
                    .iter()
                    .map(|d| offset_coord((x as isize, y as isize), d, 1))
                    .collect::<Vec<(isize, isize)>>();
                dbg!(&neighbors);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
