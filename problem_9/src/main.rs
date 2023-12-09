use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn solve(records: &Vec<Vec<i64>>) {
    let mut result_part1: i64 = 0;
    let mut result_part2: i64 = 0;

    records.iter().for_each(|record| {
        let mut previous_diffs: Vec<i64> = record.to_vec();
        let mut diffs: Vec<i64> = Vec::with_capacity(record.len() - 1);

        let mut first_diffs: Vec<i64> = Vec::with_capacity(record.len());
        let mut last_diffs: Vec<i64> = Vec::with_capacity(record.len());

        first_diffs.push(*record.first().unwrap());
        last_diffs.push(*record.last().unwrap());

        loop {
            let mut contains_non_zero = false;
            for i in 0..(previous_diffs.len() - 1) {
                let diff = previous_diffs.get(i + 1).unwrap() - previous_diffs.get(i).unwrap();
                if diff != 0 {
                    contains_non_zero = true;
                }
                diffs.push(diff);
            }

            first_diffs.push(*diffs.first().unwrap());
            last_diffs.push(*diffs.last().unwrap());

            if !contains_non_zero {
                break;
            }
            previous_diffs = diffs.to_vec();
            diffs.clear();
        }

        result_part1 += last_diffs.iter().sum::<i64>();

        let mut record_part2_result = 0;
        first_diffs.iter().rev().for_each(|diff| {
            record_part2_result = diff - record_part2_result;
        });
        result_part2 += record_part2_result;
    });

    println!("Part 1: {result_part1}");
    println!("Part 2: {result_part2}");
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let records: Vec<Vec<i64>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    solve(&records);
}
