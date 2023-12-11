use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct Galaxy {
    row: usize,
    col: usize,
}

impl Galaxy {
    fn get_distance_with_other(&self, other: &Galaxy) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(point1: usize, point2: usize) -> Range {
        match point1 < point2 {
            true => Range {
                start: point1,
                end: point2,
            },
            false => Range {
                start: point2,
                end: point1,
            },
        }
    }

    fn within_range(&self, point: usize) -> bool {
        self.start < point && point < self.end
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: problem_11 [puzzle_input]");
        return;
    }

    let file = File::open(args.get(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line_result| line_result.unwrap())
        .collect();

    // The data aint that big, so a vec sould have sufficed as well, but I don't have a nice
    // 'remove' method with a vec
    let mut expanded_rows: HashSet<usize> = (0..lines.len()).collect();
    let mut expanded_cols: HashSet<usize> = (0..lines.get(0).unwrap().len()).collect();
    let mut galaxies: Vec<Galaxy> = vec![];

    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| match c {
            '.' => {}
            '#' => {
                expanded_rows.remove(&row);
                expanded_cols.remove(&col);
                galaxies.push(Galaxy { row, col });
            }
            _ => unreachable!("What the heck is this '{}' character?", c),
        });
    });

    let mut distance_sums: usize = 0;
    let mut distance_sums_part2: usize = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let galaxy1 = galaxies.get(i).unwrap();
            let galaxy2 = galaxies.get(j).unwrap();

            let col_range = Range::new(galaxy1.col, galaxy2.col);
            let row_range = Range::new(galaxy1.row, galaxy2.row);

            let mut distance: usize = galaxy1.get_distance_with_other(galaxy2);
            let mut distance_part2 = distance;

            let expanded_col_count = expanded_cols
                .iter()
                .filter(|col| col_range.within_range(**col))
                .count();
            let expanded_row_count = expanded_rows
                .iter()
                .filter(|row| row_range.within_range(**row))
                .count();

            distance += expanded_col_count;
            distance += expanded_row_count;
            distance_part2 += expanded_col_count * 999999;
            distance_part2 += expanded_row_count * 999999;

            distance_sums += distance;
            distance_sums_part2 += distance_part2;
        }
    }

    println!("Part 1: {}", distance_sums);
    println!("Part 2: {}", distance_sums_part2);
}
