use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

type Gear = (usize, usize);

fn is_symbol(ch: &char) -> bool {
    !ch.is_numeric() && *ch != '.'
}

fn is_gear(ch: &char) -> bool {
    *ch == '*'
}

fn check_for_symbol(
    machine: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> bool {
    // Up
    if row != 0 {
        let up_row = row - 1;
        if is_symbol(&machine[up_row][col]) {
            return true;
        }
        // Up right
        if col != cols - 1 && is_symbol(&machine[up_row][col + 1]) {
            return true;
        }
        // Up left
        if col != 0 && is_symbol(&machine[up_row][col - 1]) {
            return true;
        }
    }

    // Down
    if row != rows - 1 {
        let down_row = row + 1;
        if is_symbol(&machine[down_row][col]) {
            return true;
        }
        // Down right
        if col != cols - 1 && is_symbol(&machine[down_row][col + 1]) {
            return true;
        }
        // Down left
        if col != 0 && is_symbol(&machine[down_row][col - 1]) {
            return true;
        }
    }

    // Left
    if col != 0 && is_symbol(&machine[row][col - 1]) {
        return true;
    }

    // Right
    if col != cols - 1 && is_symbol(&machine[row][col + 1]) {
        return true;
    }

    false
}

fn get_surrounding_gears(
    machine: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> Vec<Gear> {
    let mut gears: Vec<Gear> = vec![];

    // Up
    if row != 0 {
        let up_row = row - 1;
        if is_gear(&machine[up_row][col]) {
            gears.push((up_row, col));
        }
        // Up right
        if col != cols - 1 && is_gear(&machine[up_row][col + 1]) {
            gears.push((up_row, col + 1));
        }
        // Up left
        if col != 0 && is_gear(&machine[up_row][col - 1]) {
            gears.push((up_row, col - 1));
        }
    }

    // Down
    if row != rows - 1 {
        let down_row = row + 1;
        if is_gear(&machine[down_row][col]) {
            gears.push((down_row, col));
        }
        // Down right
        if col != cols - 1 && is_gear(&machine[down_row][col + 1]) {
            gears.push((down_row, col + 1));
        }
        // Down left
        if col != 0 && is_gear(&machine[down_row][col - 1]) {
            gears.push((down_row, col - 1));
        }
    }

    // Left
    if col != 0 && is_gear(&machine[row][col - 1]) {
        gears.push((row, col - 1));
    }

    // Right
    if col != cols - 1 && is_gear(&machine[row][col + 1]) {
        gears.push((row, col + 1));
    }

    gears
}

fn part1(machine: &Vec<Vec<char>>, rows: usize, cols: usize) {
    let mut total: u64 = 0;
    let mut number = String::from("");
    let mut found_symbol = false;

    for i in 0..rows {
        for j in 0..cols {
            let ch = machine[i][j];
            if ch.is_numeric() {
                number = format!("{}{}", number, ch);
                if check_for_symbol(machine, i, j, rows, cols) {
                    found_symbol = true;
                }
            } else {
                if found_symbol {
                    let n: u64 = number.parse().unwrap();
                    total += n;
                }
                if number.len() > 0 {
                    print!("({} {})", number, found_symbol);
                }
                number.clear();
                found_symbol = false;
            }
        }
        if found_symbol {
            let n: u64 = number.parse().unwrap();
            total += n;
        }
        if number.len() > 0 {
            print!("({} {})", number, found_symbol);
        }
        number.clear();
        found_symbol = false;
        println!();
    }
    println!("Part 1: {}", total);
}

fn part2(machine: &Vec<Vec<char>>, rows: usize, cols: usize) {
    let mut number = String::from("");
    let mut gear_numbers: HashMap<Gear, Vec<u64>> = HashMap::new();
    let mut gears: HashSet<Gear> = HashSet::new();

    for i in 0..rows {
        for j in 0..cols {
            let ch = machine[i][j];
            if ch.is_numeric() {
                number = format!("{}{}", number, ch);
                get_surrounding_gears(machine, i, j, rows, cols)
                    .iter()
                    .for_each(|gear| {
                        gears.insert(*gear);
                    });
            } else {
                if !number.is_empty() {
                    let n: u64 = number.parse().unwrap();
                    gears.iter().for_each(|gear| {
                        if let Some(numbers) = gear_numbers.get_mut(gear) {
                            numbers.push(n);
                        } else {
                            gear_numbers.insert(*gear, vec![n]);
                        }
                    });
                }
                number.clear();
                gears.clear();
            }
        }
        if !number.is_empty() {
            let n: u64 = number.parse().unwrap();
            gears.iter().for_each(|gear| {
                if let Some(numbers) = gear_numbers.get_mut(gear) {
                    numbers.push(n);
                } else {
                    gear_numbers.insert(*gear, vec![n]);
                }
            });
        }
        number.clear();
        gears.clear();
    }

    let total: u64 = gear_numbers
        .iter()
        .map(|(_gear, numbers)| {
            if numbers.len() == 2 {
                return numbers.get(0).unwrap() * numbers.get(1).unwrap();
            }
            0
        })
        .sum();

    println!("Part 2: {}", total);
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let reader_lines: Vec<String> = reader
        .lines()
        .map(|line_result| line_result.unwrap())
        .collect();

    let rows = reader_lines.len();
    let cols = reader_lines.get(0).unwrap().len();

    let mut machine: Vec<Vec<char>> = vec![vec![' '; cols]; rows];

    for (row, line) in reader_lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            machine[row][col] = ch;
        }
    }

    part1(&machine, rows, cols);
    part2(&machine, rows, cols);
}
