mod pipe;

use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use pipe::{Direction, Pipe, Pos};

fn determine_start_pipe_type(grid: &Vec<Vec<Pipe>>, start_pos: &Pos) -> Pipe {
    if let Some(up_line) = grid.get(start_pos.y - 1) {
        let up_pipe = up_line.get(start_pos.x).unwrap();
        // Check top and left
        if let Some(left_pipe) = grid.get(start_pos.y).unwrap().get(start_pos.x - 1) {
            let mut match_count: u8 = 0;
            match up_pipe {
                Pipe::Vertical | Pipe::DownRight | Pipe::DownLeft => match_count += 1,
                _ => {}
            };
            match left_pipe {
                Pipe::Horizontal | Pipe::DownRight | Pipe::UpRight => match_count += 1,
                _ => {}
            }
            if match_count == 2 {
                return Pipe::UpLeft;
            }
        }
        // Check top and right
        if let Some(right_pipe) = grid.get(start_pos.y).unwrap().get(start_pos.x + 1) {
            let mut match_count: u8 = 0;
            match up_pipe {
                Pipe::Vertical | Pipe::DownRight | Pipe::DownLeft => match_count += 1,
                _ => {}
            };
            match right_pipe {
                Pipe::Horizontal | Pipe::DownLeft | Pipe::UpLeft => match_count += 1,
                _ => {}
            }
            if match_count == 2 {
                return Pipe::UpRight;
            }
        }
    }
    if let Some(down_line) = grid.get(start_pos.y + 1) {
        let down_pipe = down_line.get(start_pos.x).unwrap();
        // Check bottom and left
        if let Some(left_pipe) = grid.get(start_pos.y).unwrap().get(start_pos.x - 1) {
            let mut match_count: u8 = 0;
            match down_pipe {
                Pipe::Vertical | Pipe::UpRight | Pipe::UpLeft => match_count += 1,
                _ => {}
            };
            match left_pipe {
                Pipe::Horizontal | Pipe::DownRight | Pipe::UpRight => match_count += 1,
                _ => {}
            }
            if match_count == 2 {
                return Pipe::DownLeft;
            }
        }
        // Check bottom and right
        if let Some(right_pipe) = grid.get(start_pos.y).unwrap().get(start_pos.x + 1) {
            let mut match_count: u8 = 0;
            match down_pipe {
                Pipe::Vertical | Pipe::UpRight | Pipe::UpLeft => match_count += 1,
                _ => {}
            };
            match right_pipe {
                Pipe::Horizontal | Pipe::DownLeft | Pipe::UpLeft => match_count += 1,
                _ => {}
            }
            if match_count == 2 {
                return Pipe::DownRight;
            }
        }
    }
    unreachable!("How the heck did we get here?");
}

fn part1(grid: &Vec<Vec<Pipe>>, start_pos: &Pos) -> HashSet<Pos> {
    let mut part_of_the_loop: HashSet<Pos> = HashSet::new();
    part_of_the_loop.insert(start_pos.clone());

    let start_directions = grid
        .get(start_pos.y)
        .unwrap()
        .get(start_pos.x)
        .unwrap()
        .get_directions();

    let mut distance_counter: u32 = 1;
    let mut prev_direction = start_directions.get(0).unwrap().clone();
    let mut pos = start_pos.clone();
    prev_direction.set_next_pos(&mut pos);

    loop {
        part_of_the_loop.insert(pos.clone());
        prev_direction = grid
            .get(pos.y)
            .unwrap()
            .get(pos.x)
            .unwrap()
            .get_next_direction(&prev_direction);
        prev_direction.set_next_pos(&mut pos);
        if pos == *start_pos {
            break;
        }

        distance_counter += 1;
    }

    println!("Part 1: {:?}", (distance_counter + 1) / 2);

    part_of_the_loop
}

fn part2(grid: &Vec<Vec<Pipe>>,  grid_size_x: usize, grid_size_y: usize) {
    let mut horizontal_set: HashSet<Pos> = HashSet::new();
    for (y, line) in grid.iter().enumerate() {
        let mut within = false;
        let mut riding_start_direction: Option<Direction> = None;
        for x in 0..grid_size_x {
            let p = line.get(x).unwrap();
            match *p {
                Pipe::Vertical => match riding_start_direction {
                    Some(_) => unreachable!("Should not be riding on a pipe ({}, {}).", x, y),
                    None => within = !within,
                },
                Pipe::Horizontal => match riding_start_direction {
                    Some(_) => {}
                    None => unreachable!("We should be riding on a pipe ({}, {}).", x, y),
                },
                Pipe::UpRight => match riding_start_direction {
                    Some(_) => unreachable!(
                        "We can't get this pipe type while riding a pipe ({}, {}).",
                        x, y
                    ),
                    None => riding_start_direction = Some(Direction::Up),
                },
                Pipe::DownRight => match riding_start_direction {
                    Some(_) => unreachable!(
                        "We can't get this pipe type while riding a pipe ({}, {}).",
                        x, y
                    ),
                    None => riding_start_direction = Some(Direction::Down),
                },
                Pipe::UpLeft => match riding_start_direction {
                    Some(direction) => match direction {
                        Direction::Up => riding_start_direction = None,
                        Direction::Down => {
                            within = !within;
                            riding_start_direction = None
                        }
                        _ => unreachable!(),
                    },
                    None => unreachable!(
                        "Must be riding along a pipe for this one to come up ({}, {}).",
                        x, y
                    ),
                },
                Pipe::DownLeft => match riding_start_direction {
                    Some(direction) => match direction {
                        Direction::Up => {
                            within = !within;
                            riding_start_direction = None
                        }
                        Direction::Down => riding_start_direction = None,
                        _ => unreachable!(),
                    },
                    None => unreachable!(
                        "Must be riding along a pipe for this one to come up ({}, {}).",
                        x, y
                    ),
                },
                Pipe::Ground => {
                    if within {
                        horizontal_set.insert(Pos { x, y });
                    }
                }
            }
        }
    }

    let mut veritcal_set: HashSet<Pos> = HashSet::new();
    for x in 0..grid_size_x {
        let mut within = false;
        let mut riding_start_direction: Option<Direction> = None;
        for y in 0..grid_size_y {
            let p = grid.get(y).unwrap().get(x).unwrap();
            match *p {
                Pipe::Horizontal => match riding_start_direction {
                    Some(_) => unreachable!("Should not be riding on a pipe ({}, {}).", x, y),
                    None => within = !within,
                },
                Pipe::Vertical => match riding_start_direction {
                    Some(_) => {}
                    None => unreachable!("We should be riding on a pipe ({}, {}).", x, y),
                },
                Pipe::DownRight => match riding_start_direction {
                    Some(_) => unreachable!(
                        "We can't get this pipe type while riding a pipe ({}, {}).",
                        x, y
                    ),
                    None => riding_start_direction = Some(Direction::Right),
                },
                Pipe::DownLeft => match riding_start_direction {
                    Some(_) => unreachable!(
                        "We can't get this pipe type while riding a pipe ({}, {}).",
                        x, y
                    ),
                    None => riding_start_direction = Some(Direction::Left),
                },
                Pipe::UpLeft => match riding_start_direction {
                    Some(direction) => match direction {
                        Direction::Left => riding_start_direction = None,
                        Direction::Right => {
                            within = !within;
                            riding_start_direction = None
                        }
                        _ => unreachable!(),
                    },
                    None => unreachable!(
                        "Must be riding along a pipe for this one to come up ({}, {}).",
                        x, y
                    ),
                },
                Pipe::UpRight => match riding_start_direction {
                    Some(direction) => match direction {
                        Direction::Left => {
                            within = !within;
                            riding_start_direction = None
                        }
                        Direction::Right => riding_start_direction = None,
                        _ => unreachable!(),
                    },
                    None => unreachable!(
                        "Must be riding along a pipe for this one to come up ({}, {}).",
                        x, y
                    ),
                },
                Pipe::Ground => {
                    if within {
                        veritcal_set.insert(Pos { x, y });
                    }
                }
            }
        }
    }

    let mut count: u32 = 0;
    veritcal_set
        .intersection(&horizontal_set)
        .for_each(|_| count += 1);

    println!("Part 2: {}", count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: problem_10 [puzzle_input]");
        return;
    }

    let file = File::open(args.get(1).unwrap()).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line_result| line_result.unwrap())
        .collect();

    let grid_size_y = lines.len();
    let grid_size_x = lines.get(0).unwrap().len();
    let mut grid: Vec<Vec<Pipe>> = vec![vec![Pipe::Ground; grid_size_x]; grid_size_y];
    let mut start_pos: Pos = Pos { x: 0, y: 0 };

    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            'S' => start_pos = Pos { x, y },
            '|' => *grid.get_mut(y).unwrap().get_mut(x).unwrap() = Pipe::Vertical,
            '-' => *grid.get_mut(y).unwrap().get_mut(x).unwrap() = Pipe::Horizontal,
            'L' => *grid.get_mut(y).unwrap().get_mut(x).unwrap() = Pipe::UpRight,
            'J' => *grid.get_mut(y).unwrap().get_mut(x).unwrap() = Pipe::UpLeft,
            '7' => *grid.get_mut(y).unwrap().get_mut(x).unwrap() = Pipe::DownLeft,
            'F' => *grid.get_mut(y).unwrap().get_mut(x).unwrap() = Pipe::DownRight,
            '.' => *grid.get_mut(y).unwrap().get_mut(x).unwrap() = Pipe::Ground,
            _ => unreachable!("What the heck is this letter I dunno '{}'", c),
        })
    });

    *grid
        .get_mut(start_pos.y)
        .unwrap()
        .get_mut(start_pos.x)
        .unwrap() = determine_start_pipe_type(&grid, &start_pos);

    let part_of_the_loop = part1(&grid, &start_pos);

    for y in 0..grid_size_y {
        for x in 0..grid_size_x {
            if !part_of_the_loop.contains(&Pos { x, y }) {
                *grid.get_mut(y).unwrap().get_mut(x).unwrap() = Pipe::Ground;
            }
        }
    }

    part2(&grid,  grid_size_x, grid_size_y);
}
