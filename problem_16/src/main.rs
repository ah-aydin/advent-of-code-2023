use std::{
    env,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Clone)]
/// Stores the directions that are coming inwards the tile
struct Energied {
    energized: bool,
    directions: Vec<Direction>,
}

impl Energied {
    fn new() -> Energied {
        Energied {
            energized: false,
            directions: vec![],
        }
    }

    fn add_direction(&mut self, direction: Direction) {
        self.energized = true;
        self.directions.push(direction);
    }

    fn contains_direction(&self, direction: &Direction) -> bool {
        self.directions.contains(direction)
    }
}

impl Display for Energied {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.energized {
            true => write!(f, "#"),
            false => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone)]
struct Pos {
    row: i64,
    col: i64,
}

impl Pos {
    fn within_bounds(&self, row_count: i64, col_count: i64) -> bool {
        0 <= self.row && self.row < row_count && 0 <= self.col && self.col < col_count
    }
}

#[derive(Debug, Clone)]
struct Beam {
    direction: Direction,
    pos: Pos,
}

impl Beam {
    fn new(direction: Direction, row: i64, col: i64) -> Beam {
        Beam {
            direction,
            pos: Pos { row, col },
        }
    }

    fn next_pos(&self) -> Pos {
        match self.direction {
            Direction::Up => Pos {
                row: self.pos.row - 1,
                col: self.pos.col,
            },
            Direction::Left => Pos {
                row: self.pos.row,
                col: self.pos.col - 1,
            },
            Direction::Down => Pos {
                row: self.pos.row + 1,
                col: self.pos.col,
            },
            Direction::Right => Pos {
                row: self.pos.row,
                col: self.pos.col + 1,
            },
        }
    }
}

#[derive(Debug, Clone)]
enum Tile {
    Horizontal,
    Vertical,
    Empty,
    Forward,  // /
    Backward, // \
}

impl Tile {
    fn new(c: char) -> Tile {
        match c {
            '-' => Tile::Horizontal,
            '|' => Tile::Vertical,
            '.' => Tile::Empty,
            '/' => Tile::Forward,
            '\\' => Tile::Backward,
            _ => unreachable!("What the heck is this '{}' character?", c),
        }
    }

    fn get_next_directions(&self, direction: Direction) -> Vec<Direction> {
        match (self, direction) {
            (Tile::Horizontal, Direction::Up) | (Tile::Horizontal, Direction::Down) => {
                vec![Direction::Left, Direction::Right]
            }
            (Tile::Vertical, Direction::Right) | (Tile::Vertical, Direction::Left) => {
                vec![Direction::Up, Direction::Down]
            }
            (Tile::Horizontal, Direction::Left)
            | (Tile::Forward, Direction::Down)
            | (Tile::Backward, Direction::Up) => {
                vec![Direction::Left]
            }
            (Tile::Horizontal, Direction::Right)
            | (Tile::Forward, Direction::Up)
            | (Tile::Backward, Direction::Down) => {
                vec![Direction::Right]
            }
            (Tile::Vertical, Direction::Up)
            | (Tile::Forward, Direction::Right)
            | (Tile::Backward, Direction::Left) => {
                vec![Direction::Up]
            }
            (Tile::Vertical, Direction::Down)
            | (Tile::Forward, Direction::Left)
            | (Tile::Backward, Direction::Right) => {
                vec![Direction::Down]
            }
            (Tile::Empty, direction) => vec![direction],
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: problem_16 [puzzle_input_file]");
        return;
    }
    let lines: Vec<String> = BufReader::new(File::open(args.get(1).unwrap()).unwrap())
        .lines()
        .map(|line_result| line_result.unwrap())
        .collect();

    let row_count = lines.len();
    let col_count = lines.get(0).unwrap().len();

    let mut energized: Vec<Vec<Energied>> = vec![vec![Energied::new(); col_count]; row_count];
    energized
        .get_mut(0)
        .unwrap()
        .get_mut(0)
        .unwrap()
        .add_direction(Direction::Right);

    let grid: Vec<Vec<Tile>> = lines
        .iter()
        .map(|line| line.chars().map(|c| Tile::new(c)).collect())
        .collect();

    let mut beams: Vec<Beam> = vec![Beam::new(Direction::Right, 0, 0)];
    let beam = beams.pop().unwrap();
    let pos = beam.pos;
    let directions = grid
        .get(pos.row as usize)
        .unwrap()
        .get(pos.col as usize)
        .unwrap()
        .get_next_directions(beam.direction);

    directions.iter().for_each(|direction| {
        beams.push(Beam::new(direction.clone(), pos.row, pos.col));
    });
    println!("{:?}", energized.get(0).unwrap().get(0).unwrap());

    while !beams.is_empty() {
        let beam = beams.pop().unwrap();
        let next_pos = beam.next_pos();
        if !next_pos.within_bounds(i64::from(row_count as u32), i64::from(col_count as u32)) {
            continue;
        }
        if energized
            .get(next_pos.row as usize)
            .unwrap()
            .get(next_pos.col as usize)
            .unwrap()
            .contains_direction(&beam.direction)
        {
            continue;
        }
        energized
            .get_mut(next_pos.row as usize)
            .unwrap()
            .get_mut(next_pos.col as usize)
            .unwrap()
            .add_direction(beam.direction.clone());

        grid.get(next_pos.row as usize)
            .unwrap()
            .get(next_pos.col as usize)
            .unwrap()
            .get_next_directions(beam.direction)
            .iter()
            .for_each(|direction| {
                beams.push(Beam::new(direction.clone(), next_pos.row, next_pos.col))
            });
    }

    energized.iter().for_each(|line| {
        line.iter().for_each(|item| print!("{item}"));
        println!();
    });

    println!(
        "Result part1: {}",
        energized
            .iter()
            .map(|row| row
                .iter()
                .map(|item| match item.energized {
                    true => 1,
                    false => 0,
                })
                .sum::<usize>())
            .sum::<usize>()
    );
}
