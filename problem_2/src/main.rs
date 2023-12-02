use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn part1(line: &String) -> u32 {
    let colon_split: Vec<&str> = line.split(":").collect();
    let game_id: u32 = (&(colon_split.get(0).unwrap())[5..]).parse().unwrap();
    let subsets: Vec<&str> = colon_split.get(1).unwrap().split(";").collect();

    for subset in subsets.iter() {
        let mut cubes_data: Vec<&str> = subset.split(",").collect();
        cubes_data.iter_mut().for_each(|data| {
            *data = data.trim();
        });

        for cube_data in cubes_data.iter() {
            let cube_split: Vec<&str> = cube_data.split(" ").collect();
            let count: u32 = cube_split.get(0).unwrap().parse().unwrap();
            let color = cube_split.get(1).unwrap();
            match *color {
                "red" => {
                    if count > MAX_RED {
                        println!("Game {game_id} not possible");
                        return 0;
                    }
                }
                "green" => {
                    if count > MAX_GREEN {
                        println!("Game {game_id} not possible");
                        return 0;
                    }
                }
                "blue" => {
                    if count > MAX_BLUE {
                        println!("Game {game_id} not possible");
                        return 0;
                    }
                }
                _ => unreachable!("What is this color?"),
            };
        }
    }

    println!("Game {game_id} is possible");
    game_id
}

fn part2(line: &String) -> u32 {
    let colon_split: Vec<&str> = line.split(":").collect();
    let game_id: u32 = (&(colon_split.get(0).unwrap())[5..]).parse().unwrap();
    let subsets: Vec<&str> = colon_split.get(1).unwrap().split(";").collect();

    let mut required_red: u32 = 0;
    let mut required_green: u32 = 0;
    let mut required_blue: u32 = 0;

    for subset in subsets.iter() {
        let mut cubes_data: Vec<&str> = subset.split(",").collect();
        cubes_data.iter_mut().for_each(|data| {
            *data = data.trim();
        });

        for cube_data in cubes_data.iter() {
            let cube_split: Vec<&str> = cube_data.split(" ").collect();
            let count: u32 = cube_split.get(0).unwrap().parse().unwrap();
            let color = cube_split.get(1).unwrap();
            match *color {
                "red" => {
                    if count > required_red {
                        required_red = count;
                    }
                }
                "green" => {
                    if count > required_green {
                        required_green = count;
                    }
                }
                "blue" => {
                    if count > required_blue {
                        required_blue = count;
                    }
                }
                _ => unreachable!("What is this color?"),
            };
        }
    }

    println!("Game {game_id} requires {required_red} red, {required_green} green and {required_blue} blue cubes");

    required_red * required_blue * required_green
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_part1: u32 = 0;
    let mut total_part2: u32 = 0;

    for line_result in reader.lines() {
        total_part1 += part1(line_result.as_ref().unwrap());
        total_part2 += part2(line_result.as_ref().unwrap());
    }

    println!("Result part1: {}", total_part1);
    println!("Result part2: {}", total_part2);
}
