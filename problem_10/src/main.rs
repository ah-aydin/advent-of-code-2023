mod pipe;

use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use pipe::{Pipe, Pos};

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

fn part1(grid: &Vec<Vec<Pipe>>, start_pos: &Pos) {
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
}

fn part2(grid: &Vec<Vec<Pipe>>, start_pos: &Pos) {}

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

    part1(&grid, &start_pos);
    part2(&grid, &start_pos);
}
