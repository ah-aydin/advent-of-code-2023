use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn part1(line: &String) -> u64 {
    let numbers = line
        .split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split("|")
        .collect::<Vec<&str>>();

    let winning_numbers: HashSet<u64> = numbers
        .get(0)
        .unwrap()
        .split(" ")
        .map(|item| item.trim())
        .filter(|item| !item.is_empty())
        .map(|item| item.parse::<u64>().unwrap())
        .collect();

    let mut guessed_number_count: u32 = 0;

    numbers
        .get(1)
        .unwrap()
        .split(" ")
        .map(|item| item.trim())
        .filter(|item| !item.is_empty())
        .map(|item| item.parse::<u64>().unwrap())
        .for_each(|number| {
            if winning_numbers.contains(&number) {
                guessed_number_count += 1;
            }
        });

    if guessed_number_count == 0 {
        return 0;
    }

    (2 as u64).pow(guessed_number_count - 1)
}

fn part2(lines: Vec<String>) {
    let mut game_counts: Vec<u64> = vec![1; lines.len()];

    for (game_id, game) in lines.iter().enumerate() {
        let numbers = game
            .split(":")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .split("|")
            .collect::<Vec<&str>>();

        let winning_numbers: HashSet<u64> = numbers
            .get(0)
            .unwrap()
            .split(" ")
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
            .map(|item| item.parse::<u64>().unwrap())
            .collect();

        let mut guessed_number_count: usize = 0;

        numbers
            .get(1)
            .unwrap()
            .split(" ")
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
            .map(|item| item.parse::<u64>().unwrap())
            .for_each(|number| {
                if winning_numbers.contains(&number) {
                    guessed_number_count += 1;
                }
            });

        for i in (game_id + 1)..(game_id + 1 + guessed_number_count) {
            game_counts[i] += game_counts[game_id];
        }
    }

    println!("{:?}", game_counts);

    println!("Part 2: {}", game_counts.iter().sum::<u64>());
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut result_part1 = 0;
    let mut lines: Vec<String> = vec![];

    for line_result in reader.lines() {
        lines.push(line_result.as_ref().unwrap().clone());
        result_part1 += part1(&line_result.unwrap());
    }

    println!("Part 1: {}", result_part1);

    part2(lines);
}
