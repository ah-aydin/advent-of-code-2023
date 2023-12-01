use std::fs::File;
use std::io::{BufRead, BufReader};

fn check3lettered(slice: &str) -> Option<u32> {
    match slice {
        "one" => Some(1),
        "two" => Some(2),
        "six" => Some(6),
        _ => None,
    }
}

fn check4lettered(slice: &str) -> Option<u32> {
    match slice {
        "four" => Some(4),
        "five" => Some(5),
        "nine" => Some(9),
        _ => None,
    }
}

fn check5lettered(slice: &str) -> Option<u32> {
    match slice {
        "three" => Some(3),
        "seven" => Some(7),
        "eight" => Some(8),
        _ => None,
    }
}

fn part1(line: &String) -> u32 {
    let mut code: u32 = 0;
    for c in line.chars() {
        if c.is_numeric() {
            code = c.to_digit(10).unwrap() * 10;
            break;
        }
    }
    for c in line.chars().rev() {
        if c.is_numeric() {
            code += c.to_digit(10).unwrap();
            break;
        }
    }

    code
}

fn part2(line: &String) -> u32 {
    let mut code: u32 = 0;
    let len = line.len();
    for (i, c) in line.char_indices() {
        if c.is_numeric() {
            code = c.to_digit(10).unwrap() * 10;
            break;
        } else if "otfsen".contains(c) {
            let remaining = len - (i + 1);
            match remaining {
                0 | 1 | 2 => {}
                3 => {
                    if let Some(n) = check3lettered(&line[i..(i + 3)]) {
                        code = n * 10;
                        break;
                    }
                }
                4 => {
                    if let Some(n) = check3lettered(&line[i..(i + 3)]) {
                        code = n * 10;
                        break;
                    }
                    if let Some(n) = check4lettered(&line[i..(i + 4)]) {
                        code = n * 10;
                        break;
                    }
                }
                5 | _ => {
                    if let Some(n) = check3lettered(&line[i..(i + 3)]) {
                        code = n * 10;
                        break;
                    }
                    if let Some(n) = check4lettered(&line[i..(i + 4)]) {
                        code = n * 10;
                        break;
                    }
                    if let Some(n) = check5lettered(&line[i..(i + 5)]) {
                        code = n * 10;
                        break;
                    }
                }
            };
        }
    }

    for (i, c) in line.char_indices().rev() {
        if c.is_numeric() {
            code += c.to_digit(10).unwrap();
            break;
        } else if "eorxnt".contains(c) {
            let remaining = i;
            match remaining {
                0 | 1 | 2 => {}
                3 => {
                    if let Some(n) = check3lettered(&line[(i - 2)..(i + 1)]) {
                        code += n;
                        break;
                    }
                }
                4 => {
                    if let Some(n) = check3lettered(&line[(i - 2)..(i + 1)]) {
                        code += n;
                        break;
                    }
                    if let Some(n) = check4lettered(&line[(i - 3)..(i + 1)]) {
                        code += n;
                        break;
                    }
                }
                5 | _ => {
                    if let Some(n) = check3lettered(&line[(i - 2)..(i + 1)]) {
                        code += n;
                        break;
                    }
                    if let Some(n) = check4lettered(&line[(i - 3)..(i + 1)]) {
                        code += n;
                        break;
                    }
                    if let Some(n) = check5lettered(&line[(i - 4)..(i + 1)]) {
                        code += n;
                        break;
                    }
                }
            };
        }
    }

    code
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let mut total_part1: u32 = 0;
    let mut total_part2: u32 = 0;
    for line_result in BufReader::new(file).lines() {
        total_part1 += part1(line_result.as_ref().unwrap());
        total_part2 += part2(line_result.as_ref().unwrap());
    }

    println!("Result part1: {}", total_part1);
    println!("Result part2: {}", total_part2);
}
