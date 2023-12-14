use std::{
    env,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Round,
    Cube,
    Empty,
}

impl Tile {
    fn new(c: char) -> Tile {
        match c {
            'O' => Tile::Round,
            '#' => Tile::Cube,
            '.' => Tile::Empty,
            _ => unreachable!("What the heck is this '{}' character?", c),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Round => write!(f, "O"),
            Tile::Cube => write!(f, "#"),
            Tile::Empty => write!(f, "."),
        }
    }
}

type Platform = Vec<Vec<Tile>>;

fn print_platform(platform: &Platform) {
    println!("---------------");
    platform.iter().for_each(|row| {
        row.iter().for_each(|tile| print!("{tile}"));
        println!();
    });
    println!("---------------");
}

fn part1(platform: &Platform) -> usize {
    let mut result = 0;
    let row_count = platform.len();
    let col_count = platform.get(0).unwrap().len();

    for col in 0..col_count {
        let mut load = row_count;
        for row in 0..row_count {
            match platform.get(row).unwrap().get(col).unwrap() {
                Tile::Round => {
                    result += load;
                    load -= 1;
                }
                Tile::Cube => {
                    load = row_count - row - 1;
                }
                Tile::Empty => {}
            }
        }
    }

    result
}

fn rotate(platform: &mut Platform) {
    let row_count = platform.len();
    let col_count = platform.get(0).unwrap().len();

    // North
    for col in 0..col_count {
        let mut row_pos = 0;
        for row in 0..row_count {
            match platform.get(row).unwrap().get(col).unwrap() {
                Tile::Round => {
                    *platform.get_mut(row).unwrap().get_mut(col).unwrap() = Tile::Empty;
                    *platform.get_mut(row_pos).unwrap().get_mut(col).unwrap() = Tile::Round;
                    row_pos += 1;
                }
                Tile::Cube => {
                    row_pos = row + 1;
                }
                Tile::Empty => {}
            }
        }
    }
    println!("North");
    print_platform(&platform);

    // West
    for row in 0..row_count {
        let mut col_pos = 0;
        for col in 0..col_count {
            match platform.get(row).unwrap().get(col).unwrap() {
                Tile::Round => {
                    *platform.get_mut(row).unwrap().get_mut(col).unwrap() = Tile::Empty;
                    *platform.get_mut(row).unwrap().get_mut(col_pos).unwrap() = Tile::Round;
                    col_pos += 1;
                }
                Tile::Cube => {
                    col_pos = col + 1;
                }
                Tile::Empty => {}
            }
        }
    }
    println!("West");
    print_platform(&platform);

    // South
    for col in 0..col_count {
        let mut row_pos = row_count - 1;
        for row in (0..row_count).rev() {
            match platform.get(row).unwrap().get(col).unwrap() {
                Tile::Round => {
                    *platform.get_mut(row).unwrap().get_mut(col).unwrap() = Tile::Empty;
                    *platform.get_mut(row_pos).unwrap().get_mut(col).unwrap() = Tile::Round;
                    row_pos -= 1;
                }
                Tile::Cube => {
                    row_pos = row - 1;
                }
                Tile::Empty => {}
            }
        }
    }
    println!("South");
    print_platform(&platform);

    // East
    for row in 0..row_count {
        let mut col_pos = col_count - 1;
        for col in (0..col_count).rev() {
            match platform.get(row).unwrap().get(col).unwrap() {
                Tile::Round => {
                    *platform.get_mut(row).unwrap().get_mut(col).unwrap() = Tile::Empty;
                    *platform.get_mut(row).unwrap().get_mut(col_pos).unwrap() = Tile::Round;
                    col_pos -= 1;
                }
                Tile::Cube => {
                    col_pos = col - 1;
                }
                Tile::Empty => {}
            }
        }
    }
    println!("East");
    print_platform(&platform);
}

fn part2(platform: Platform) -> usize {
    let mut history: Vec<Platform> = vec![];
    history.push(platform.to_vec());

    let mut platform = platform;
    let num_iters = 1000000000;

    for i in 0..num_iters {
        println!("After {} cycles", i);
        print_platform(&platform);
        rotate(&mut platform);

        if let Some(history_index) = history.iter().position(|plat| *plat == platform) {
            history.drain(0..history_index);
            let final_index: usize = (num_iters - (i + 1)) % history.len();
            println!("After {num_iters} cycles");
            print_platform(&history.get(final_index).unwrap());
            history
                .iter()
                .enumerate()
                .for_each(|(i, p)| println!("{i} {}", part1(&p)));
            return part1(&history.get(final_index).unwrap());
        }
        history.push(platform.to_vec());
    }
    return part1(&platform);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: problem_14 [puzzle_input_file]");
        return;
    }
    let file = File::open(args.get(1).unwrap()).unwrap();
    let platform: Platform = BufReader::new(file)
        .lines()
        .map(|line_result| line_result.unwrap().chars().map(|c| Tile::new(c)).collect())
        .collect();

    println!("Result part1: {}", part1(&platform));
    println!("Result part2: {}", part2(platform));
}
