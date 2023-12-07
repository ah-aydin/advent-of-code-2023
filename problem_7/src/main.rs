mod hand;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use hand::Hand;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut hands: Vec<Hand> = vec![];
    let mut hands_with_joker: Vec<Hand> = vec![];
    for line_result in reader.lines() {
        let line_split: Vec<String> = line_result
            .unwrap()
            .split(" ")
            .map(|s| s.to_owned())
            .collect();
        let hand = line_split.get(0).unwrap();
        let bid: u64 = line_split.get(1).unwrap().parse().unwrap();
        hands.push(Hand::new(hand.clone(), bid));
        hands_with_joker.push(Hand::new_with_joker(hand.clone(), bid));
    }

    hands.sort();
    hands_with_joker.sort();

    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    for (i, hand) in hands.iter().enumerate() {
        part1 += ((i + 1) as u64) * hand.bid;
    }
    for (i, hand) in hands_with_joker.iter().enumerate() {
        part2 += ((i + 1) as u64) * hand.bid;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
