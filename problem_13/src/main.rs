use std::{
    env,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn new(c: char) -> Tile {
        match c {
            '.' => Tile::Ash,
            '#' => Tile::Rock,
            _ => unreachable!("What the heck is this '{}' character?", c),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ash => write!(f, "."),
            Tile::Rock => write!(f, "#"),
        }
    }
}

fn find_vertical_mirror(pattern: &Vec<Vec<Tile>>) -> usize {
    let row_count = pattern.len();
    let col_count = pattern.get(0).unwrap().len();
    for col in 1..col_count {
        let mirror_size = col.min(col_count - col);
        let mut all_match = true;

        for (left_col, right_col) in zip(((col - mirror_size)..col).rev(), col..(col + mirror_size))
        {
            let mut row_match = true;
            for row in 0..row_count {
                if pattern.get(row).unwrap().get(left_col)
                    != pattern.get(row).unwrap().get(right_col)
                {
                    row_match = false;
                    break;
                }
            }
            if !row_match {
                all_match = false;
                break;
            }
        }

        if all_match {
            return col;
        }
    }
    0
}

fn find_horizontal_mirror(pattern: &Vec<Vec<Tile>>) -> usize {
    let row_count = pattern.len();
    let col_count = pattern.get(0).unwrap().len();
    for row in 1..row_count {
        let mirror_size = row.min(row_count - row);
        let mut all_match = true;

        for (up_row, down_row) in zip(((row - mirror_size)..row).rev(), row..(row + mirror_size)) {
            let mut col_match = true;
            for col in 0..col_count {
                if pattern.get(up_row).unwrap().get(col) != pattern.get(down_row).unwrap().get(col)
                {
                    col_match = false;
                    break;
                }
            }
            if !col_match {
                all_match = false;
                break;
            }
        }

        if all_match {
            return row;
        }
    }
    0
}

fn find_vertical_mirror_2(pattern: &Vec<Vec<Tile>>) -> usize {
    let row_count = pattern.len();
    let col_count = pattern.get(0).unwrap().len();
    for col in 1..col_count {
        let mirror_size = col.min(col_count - col);

        let mut mismatch_count = 0;
        for (left_col, right_col) in zip(((col - mirror_size)..col).rev(), col..(col + mirror_size))
        {
            for row in 0..row_count {
                if pattern.get(row).unwrap().get(left_col)
                    != pattern.get(row).unwrap().get(right_col)
                {
                    mismatch_count += 1;
                    if mismatch_count > 1 {
                        break;
                    }
                }
            }
            if mismatch_count > 1 {
                break;
            }
        }

        if mismatch_count == 1 {
            return col;
        }
    }
    0
}

fn find_horizontal_mirror_2(pattern: &Vec<Vec<Tile>>) -> usize {
    let row_count = pattern.len();
    let col_count = pattern.get(0).unwrap().len();
    for row in 1..row_count {
        let mirror_size = row.min(row_count - row);
        let mut mismatch_count = 0;

        for (up_row, down_row) in zip(((row - mirror_size)..row).rev(), row..(row + mirror_size)) {
            for col in 0..col_count {
                if pattern.get(up_row).unwrap().get(col) != pattern.get(down_row).unwrap().get(col)
                {
                    mismatch_count += 1;
                    if mismatch_count > 1 {
                        break;
                    }
                }
            }
            if mismatch_count > 1 {
                break;
            }
        }

        if mismatch_count == 1 {
            return row;
        }
    }
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: problem_13 [puzzle_input_file]");
    }
    let file = File::open(args.get(1).unwrap()).unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line_result| line_result.unwrap())
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut line_index = 0;
    let mut pattern: Vec<Vec<Tile>> = vec![];

    loop {
        let line = match lines.get(line_index) {
            Some(line) => line,
            None => break,
        };

        if line.len() == 0 || line_index + 1 == lines.len() {
            part1 += find_vertical_mirror(&pattern);
            part1 += find_horizontal_mirror(&pattern) * 100;
            part2 += find_vertical_mirror_2(&pattern);
            part2 += find_horizontal_mirror_2(&pattern) * 100;

            pattern.clear();
            line_index += 1;
        } else {
            pattern.push(line.chars().map(|c| Tile::new(c)).collect());
            line_index += 1;
        }
    }

    println!("Result part1: {part1}");
    println!("Result part2: {part2}");
}
