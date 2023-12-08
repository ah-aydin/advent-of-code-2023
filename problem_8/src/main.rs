use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Junction {
    left: String,
    right: String,
}

impl Junction {
    fn new(left: String, right: String) -> Junction {
        Junction { left, right }
    }

    fn get_next_junction(&self, direction: &Direction) -> &String {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

fn find_smallest_common_multiple(numbers: &Vec<u64>) -> u64 {
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            return a;
        }
        gcd(b, a % b)
    }

    fn lcm(a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 {
            return 0;
        }
        (a * b) / gcd(a, b)
    }

    let mut result = numbers.get(0).cloned().unwrap();

    for &num in numbers.iter().skip(1) {
        result = lcm(result, num);
    }

    result
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let directions: Vec<Direction> = lines
        .get(0)
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!("Wtf is this direction?"),
        })
        .collect();

    let mut part2_starting_locations: Vec<String> = vec![];
    let mut junctions: HashMap<String, Junction> = HashMap::new();
    for i in 2..lines.len() {
        let line = lines.get(i).unwrap();
        let line = String::from(&line[0..(line.len() - 1)]);
        let split: Vec<&str> = line.split(" = (").collect();
        let location = split.get(0).unwrap();

        if location.chars().last().unwrap() == 'A' {
            part2_starting_locations.push(location.to_string());
        }

        let junction_names: Vec<String> = split
            .get(1)
            .unwrap()
            .split(", ")
            .map(|juntion_name| juntion_name.to_owned())
            .collect();

        junctions.insert(
            location.to_string(),
            Junction::new(
                junction_names.get(0).unwrap().to_string(),
                junction_names.get(1).unwrap().to_string(),
            ),
        );
    }

    let mut step: usize = 0;
    let mut step_count: u64 = 0;
    let mut current_location: &str = "AAA";
    loop {
        let junction = junctions.get(current_location).unwrap();
        current_location = junction.get_next_junction(directions.get(step).unwrap());

        step += 1;
        step_count += 1;
        step %= directions.len();

        println!("{} {}", step, current_location);
        if current_location == "ZZZ" {
            break;
        }
    }

    println!("Part 1: {}", step_count);

    let mut required_step_counts: Vec<u64> = Vec::with_capacity(part2_starting_locations.len());
    part2_starting_locations
        .iter()
        .for_each(|starting_location| {
            let mut step: usize = 0;
            let mut step_count: u64 = 0;
            let mut current_location: &str = starting_location;
            loop {
                let junction = junctions.get(current_location).unwrap();
                current_location = junction.get_next_junction(directions.get(step).unwrap());

                step += 1;
                step_count += 1;
                step %= directions.len();

                if current_location.chars().last().unwrap() == 'Z' {
                    break;
                }
            }

            required_step_counts.push(step_count)
        });

    println!("{:?}", required_step_counts);

    println!(
        "Part 2: {}",
        find_smallest_common_multiple(&required_step_counts)
    );
}
