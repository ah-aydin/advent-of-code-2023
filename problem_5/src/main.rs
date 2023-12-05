mod range;

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
    thread::{self, JoinHandle},
};

use range::{Range, RangeList};

use crate::range::SeedRange;

fn populate_range_list(
    lines: &Vec<String>,
    line_number: usize,
    range_list: &mut RangeList,
) -> usize {
    let mut line_number = line_number;
    loop {
        if line_number >= lines.len() {
            break;
        }

        let line = lines.get(line_number).unwrap();
        if line.is_empty() {
            break;
        }

        let line_split: Vec<u64> = line.split(" ").map(|n| n.parse::<u64>().unwrap()).collect();
        let destination_start = line_split.get(0).unwrap();
        let source_start = line_split.get(1).unwrap();
        let range = line_split.get(2).unwrap();
        let source_end = source_start + range - 1;

        range_list.add_range(Range::new(*source_start, source_end, *destination_start));

        line_number += 1;
    }

    line_number
}

fn part1(lines: Vec<String>) {
    let seeds: HashSet<u64> = lines.get(0).unwrap()["seeds: ".len()..]
        .split(" ")
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();

    let mut line_number = 3; // Start from seed-to-soil map

    let mut seed_to_soil = RangeList::new();
    let mut soil_to_fertilizer = RangeList::new();
    let mut fertilizer_to_water = RangeList::new();
    let mut water_to_light = RangeList::new();
    let mut light_to_temperature = RangeList::new();
    let mut temperature_to_humidity = RangeList::new();
    let mut humidity_to_location = RangeList::new();

    line_number = populate_range_list(&lines, line_number, &mut seed_to_soil) + 2;
    line_number = populate_range_list(&lines, line_number, &mut soil_to_fertilizer) + 2;
    line_number = populate_range_list(&lines, line_number, &mut fertilizer_to_water) + 2;
    line_number = populate_range_list(&lines, line_number, &mut water_to_light) + 2;
    line_number = populate_range_list(&lines, line_number, &mut light_to_temperature) + 2;
    line_number = populate_range_list(&lines, line_number, &mut temperature_to_humidity) + 2;
    let _ = populate_range_list(&lines, line_number, &mut humidity_to_location) + 2;

    let min_distance = seeds
        .iter()
        .map(|seed| {
            let soil = seed_to_soil.get_destination(*seed);
            let fertilizer = soil_to_fertilizer.get_destination(soil);
            let water = fertilizer_to_water.get_destination(fertilizer);
            let light = water_to_light.get_destination(water);
            let temperature = light_to_temperature.get_destination(light);
            let humidity = temperature_to_humidity.get_destination(temperature);
            return humidity_to_location.get_destination(humidity);
        })
        .min()
        .unwrap();

    println!("Part 1: {}", min_distance);
}

/// The solution that you write after a long day at work and then failing
/// at basic math to do the problem efficiently
fn part2(lines: Vec<String>) {
    let seed_line_split: Vec<u64> = lines.get(0).unwrap()["seeds: ".len()..]
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut i: usize = 0;
    let mut seed_ranges: Vec<SeedRange> = Vec::new();
    loop {
        if i >= seed_line_split.len() {
            break;
        }

        let start = seed_line_split.get(i).unwrap();
        let range = seed_line_split.get(i + 1).unwrap();
        let end = start + range - 1;

        seed_ranges.push(SeedRange::new(*start, end));

        i += 2;
    }

    let mut line_number = 3; // Start from seed-to-soil map

    let mut seed_to_soil = RangeList::new();
    let mut soil_to_fertilizer = RangeList::new();
    let mut fertilizer_to_water = RangeList::new();
    let mut water_to_light = RangeList::new();
    let mut light_to_temperature = RangeList::new();
    let mut temperature_to_humidity = RangeList::new();
    let mut humidity_to_location = RangeList::new();

    line_number = populate_range_list(&lines, line_number, &mut seed_to_soil) + 2;
    line_number = populate_range_list(&lines, line_number, &mut soil_to_fertilizer) + 2;
    line_number = populate_range_list(&lines, line_number, &mut fertilizer_to_water) + 2;
    line_number = populate_range_list(&lines, line_number, &mut water_to_light) + 2;
    line_number = populate_range_list(&lines, line_number, &mut light_to_temperature) + 2;
    line_number = populate_range_list(&lines, line_number, &mut temperature_to_humidity) + 2;
    let _ = populate_range_list(&lines, line_number, &mut humidity_to_location) + 2;

    let seed_to_soil = Arc::new(seed_to_soil);
    let soil_to_fertilizer = Arc::new(soil_to_fertilizer);
    let fertilizer_to_water = Arc::new(fertilizer_to_water);
    let water_to_light = Arc::new(water_to_light);
    let light_to_temperature = Arc::new(light_to_temperature);
    let temperature_to_humidity = Arc::new(temperature_to_humidity);
    let humidity_to_location = Arc::new(humidity_to_location);

    let cpu_count = num_cpus::get();
    while seed_ranges.len() < cpu_count {
        seed_ranges.sort();

        let seed_range = seed_ranges.pop().unwrap();

        let half_point = seed_range.start + seed_range.get_lenght() / 2;
        seed_ranges.push(SeedRange::new(seed_range.start, half_point));
        seed_ranges.push(SeedRange::new(half_point, seed_range.end));
    }

    seed_ranges.iter().for_each(|range| println!("{:?}", range));
    let mut thread_handles: Vec<JoinHandle<u64>> = Vec::new();

    for i in 0..cpu_count {
        let seed_range = seed_ranges.get(i).unwrap().clone();
        let seed_to_soil = Arc::clone(&seed_to_soil);
        let soil_to_fertilizer = Arc::clone(&soil_to_fertilizer);
        let fertilizer_to_water = Arc::clone(&fertilizer_to_water);
        let water_to_light = Arc::clone(&water_to_light);
        let light_to_temperature = Arc::clone(&light_to_temperature);
        let temperature_to_humidity = Arc::clone(&temperature_to_humidity);
        let humidity_to_location = Arc::clone(&humidity_to_location);

        thread_handles.push(thread::spawn(move || {
            println!(
                "Thread {:?} started. Will process {} seeds",
                thread::current().id(),
                seed_range.get_lenght()
            );
            let mut min: Option<u64> = None;
            for seed in seed_range.start..(seed_range.end + 1) {
                if seed % 2000 == 0 {
                    println!(
                        "Tread {:?} {} seeds remaning",
                        thread::current().id(),
                        seed_range.end - seed
                    );
                }
                let soil = seed_to_soil.get_destination(seed);
                let fertilizer = soil_to_fertilizer.get_destination(soil);
                let water = fertilizer_to_water.get_destination(fertilizer);
                let light = water_to_light.get_destination(water);
                let temperature = light_to_temperature.get_destination(light);
                let humidity = temperature_to_humidity.get_destination(temperature);
                let distance = humidity_to_location.get_destination(humidity);
                if let Some(m) = min {
                    if m > distance {
                        min = Some(distance);
                    }
                } else {
                    min = Some(distance);
                }
            }
            println!("Min val {:?} {:?}", min, thread::current().id());
            min.unwrap()
        }));
    }

    let mut min: Option<u64> = None;
    for thread in thread_handles {
        let distance = thread.join().unwrap();
        if let Some(m) = min {
            if m > distance {
                min = Some(distance);
            }
        } else {
            min = Some(distance);
        }
    }

    println!("\n");

    println!("Part 2: {:?}", min);
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    part1(lines.clone());
    part2(lines.clone());
}
