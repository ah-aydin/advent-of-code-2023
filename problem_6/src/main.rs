use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

fn part1(lines: Vec<String>) {
    let times: Vec<u64> = lines
        .get(0)
        .unwrap()
        .split(" ")
        .filter(|item| !item.is_empty())
        .map(|item| item.parse::<u64>().unwrap())
        .collect();

    let distances: Vec<u64> = lines
        .get(1)
        .unwrap()
        .split(" ")
        .filter(|item| !item.is_empty())
        .map(|item| item.parse::<u64>().unwrap())
        .collect();

    println!("Times:     {:?}\nDistances: {:?}", times, distances);

    let mut result: u64 = 1;
    for (time, distance) in zip(times, distances) {
        for i in 0..time {
            if i * (time - i) > distance {
                result *= time - 2 * i + 1;
                break;
            }
        }
    }

    println!("Part 1: {}", result);
}

fn part2(lines: Vec<String>) {
    let mut time = String::from("");
    lines
        .get(0)
        .unwrap()
        .split(" ")
        .filter(|item| !item.is_empty())
        .for_each(|item| time.push_str(item));

    let mut distance = String::from("");
    lines
        .get(1)
        .unwrap()
        .split(" ")
        .filter(|item| !item.is_empty())
        .for_each(|item| distance.push_str(item));

    let time: u64 = time.parse().unwrap();
    let distance: u64 = distance.parse().unwrap();

    println!("Time:     {:?}\nDistance: {:?}", time, distance);

    let mut result: u64 = 1;
    for i in 0..time {
        if i * (time - i) > distance {
            result *= time - 2 * i + 1;
            break;
        }
    }

    println!("Part 2: {}", result);
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(":")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .trim()
                .to_owned()
        })
        .collect();

    part1(lines.clone());
    part2(lines.clone());
}
