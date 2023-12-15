use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

#[derive(Clone, Debug)]
struct Lense {
    label: String,
    focal_length: u64,
}

fn calc_hash(s: &str) -> u64 {
    let mut hash = 0;
    s.chars()
        .for_each(|c| hash = (hash + (c as u8) as u64) * 17 % 256);
    hash
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: problem_15 [puzzle_input_file]");
        return;
    }
    let lines: Vec<String> = BufReader::new(File::open(args.get(1).unwrap()).unwrap())
        .lines()
        .map(|line_result| line_result.unwrap())
        .collect();

    let mut result_part1: u64 = 0;
    let mut hashes: HashMap<String, u64> = HashMap::new();
    lines.iter().for_each(|line| {
        line.split(",").for_each(|s| match hashes.get(s) {
            Some(hash) => result_part1 += *hash as u64,
            None => {
                let hash = calc_hash(s);
                println!("{s} = {hash}");
                result_part1 += hash;
                hashes.insert(s.to_string(), hash);
            }
        });
    });
    println!("Result part1: {result_part1}");

    hashes.clear();
    let mut boxes: Vec<Vec<Lense>> = vec![Vec::new(); 256];
    lines.iter().for_each(|line| {
        line.split(",").for_each(|s| {
            if s.contains("=") {
                let split: Vec<&str> = s.split("=").collect();
                let lense = Lense {
                    label: split.get(0).unwrap().to_string(),
                    focal_length: split.get(1).unwrap().parse::<u64>().unwrap(),
                };

                let hash = match hashes.get(&lense.label) {
                    Some(hash) => *hash,
                    None => {
                        let hash = calc_hash(&lense.label);
                        hashes.insert(lense.label.clone(), hash);
                        hash
                    }
                } as usize;

                if let Some(pos) = boxes
                    .get(hash)
                    .unwrap()
                    .iter()
                    .position(|in_lense| in_lense.label == lense.label)
                {
                    boxes
                        .get_mut(hash)
                        .unwrap()
                        .get_mut(pos)
                        .unwrap()
                        .focal_length = lense.focal_length;
                    println!("{s} swaped in {hash}");
                } else {
                    boxes.get_mut(hash).unwrap().push(lense);
                    println!("{s} goes in {hash}");
                }
            } else if s.contains("-") {
                let label = *s.split('-').collect::<Vec<&str>>().get(0).unwrap();
                let hash = match hashes.get(label) {
                    Some(hash) => *hash,
                    None => {
                        let hash = calc_hash(label);
                        hashes.insert(label.to_string(), hash);
                        hash
                    }
                } as usize;

                if let Some(pos) = boxes
                    .get(hash)
                    .unwrap()
                    .iter()
                    .position(|in_lense| in_lense.label == label)
                {
                    boxes.get_mut(hash).unwrap().remove(pos);
                    println!("{s} is removed from {hash}");
                }
            }
        });
    });

    let mut result_part2: usize = 0;
    boxes.iter().enumerate().for_each(|(i, bx)| {
        bx.iter().enumerate().for_each(|(j, lense)| {
            result_part2 += (i + 1) * (j + 1) * (lense.focal_length as usize);
        });
    });
    println!("Result part2: {result_part2}");
}
