use memoize::memoize;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[memoize]
fn find_arrangements(spring: String, mut group_sizes: Vec<usize>, tab: usize) -> u64 {
    if spring.is_empty() {
        return match group_sizes.is_empty() {
            true => 1,
            false => 0,
        };
    }
    if group_sizes.is_empty() {
        return match spring.contains('#') {
            true => 0,
            false => 1,
        };
    }

    let mut s = 0;
    let group_size = *group_sizes.first().unwrap();
    let first_c = spring.chars().nth(0).unwrap();

    s += match first_c {
        '.' => find_arrangements(spring[1..].to_string(), group_sizes.clone(), tab + 1),
        '?' => {
            let mut s = find_arrangements(spring[1..].to_string(), group_sizes.clone(), tab + 1);
            if group_size <= spring.len()
                && !spring[..group_size].contains('.')
                && spring.chars().nth(group_size).unwrap_or('.') != '#'
            {
                group_sizes.remove(0);
                let next_spring = match group_size + 1 < spring.len() {
                    true => &spring[(group_size + 1)..],
                    false => "",
                }
                .to_string();
                s += find_arrangements(next_spring, group_sizes.clone(), tab + 1);
            }
            s
        }
        '#' => {
            let mut s = 0;
            if group_size <= spring.len()
                && !spring[..group_size].contains('.')
                && spring.chars().nth(group_size).unwrap_or('.') != '#'
            {
                group_sizes.remove(0);
                let next_spring = match group_size + 1 < spring.len() {
                    true => &spring[(group_size + 1)..],
                    false => "",
                }
                .to_string();
                s += find_arrangements(next_spring, group_sizes.clone(), tab + 1);
            }
            s
        }
        _ => unreachable!("What the heck is this '{}' character", first_c),
    };

    s
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: problem_12 [puzzle_input_file]");
    }
    let file = File::open(args.get(1).unwrap()).unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line_result| line_result.unwrap())
        .collect();

    let mut part1: u64 = 0;
    lines.iter().for_each(|line| {
        let line_split: Vec<&str> = line.split(" ").collect();
        let spring: String = line_split.get(0).unwrap().to_string();
        let group_sizes: Vec<usize> = line_split
            .get(1)
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let s = find_arrangements(spring, group_sizes.clone(), 0);
        part1 += s;
    });
    println!("Part 1: {}", part1);

    let mut part2: u64 = 0;
    lines.iter().for_each(|line| {
        let line_split: Vec<&str> = line.split(" ").collect();
        let spring: String = line_split.get(0).unwrap().to_string();
        let spring = format!("{}?{}?{}?{}?{}", spring, spring, spring, spring, spring);

        let group_sizes: Vec<usize> = line_split
            .get(1)
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut new_group_sizes: Vec<usize> = Vec::with_capacity(group_sizes.len() * 5);
        for _ in 0..5 {
            group_sizes
                .iter()
                .for_each(|group_size| new_group_sizes.push(*group_size));
        }

        let s = find_arrangements(spring, new_group_sizes.clone(), 0);
        part2 += s;
    });
    println!("Part 2: {}", part2);
}
