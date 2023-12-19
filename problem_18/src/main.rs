use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(c: char) -> Direction {
        match c {
            'U' | '3' => Direction::Up,
            'D' | '1' => Direction::Down,
            'L' | '2' => Direction::Left,
            'R' | '0' => Direction::Right,

            _ => unreachable!("What the heck is this '{}' character?", c),
        }
    }

    fn get_change(&self, distance: i64) -> (i64, i64) {
        match self {
            Direction::Up => (0, -distance),
            Direction::Down => (0, distance),
            Direction::Left => (-distance, 0),
            Direction::Right => (distance, 0),
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    grid.iter().for_each(|line| {
        line.iter().for_each(|tile| match tile {
            true => print!("#"),
            false => print!("."),
        });
        println!();
    });
}

fn part1(lines: Vec<String>) {
    let mut points: Vec<Point> = vec![Point { x: 0, y: 0 }];
    for line in lines.iter() {
        let split: Vec<&str> = line.split(" ").collect();

        let direction = Direction::new(split.get(0).unwrap().chars().nth(0).unwrap());
        let distance: i64 = split.get(1).unwrap().parse().unwrap();

        let (x_change, y_change) = direction.get_change(distance);
        let last_point = points.last().unwrap();
        points.push(Point {
            x: last_point.x + x_change,
            y: last_point.y + y_change,
        });
    }
    points.pop();

    let mut x_count = 0;
    let mut y_count = 0;
    let mut x_min = 1 << 60;
    let mut y_min = 1 << 60;
    points.iter().for_each(|Point { x, y }| {
        if *x > x_count {
            x_count = *x;
        }
        if *x < x_min {
            x_min = *x;
        }
        if *y > y_count {
            y_count = *y;
        }
        if *y < y_min {
            y_min = *y;
        }
    });

    if x_count < 0 {
        x_count = x_count.abs();
        points
            .iter_mut()
            .for_each(|point| point.x = point.x + x_count * 2);
    }
    if y_count < 0 {
        y_count = y_count.abs();
        points
            .iter_mut()
            .for_each(|point| point.y = point.y + y_count * 2);
    }
    if x_min < 0 {
        x_count += x_min.abs();
        points
            .iter_mut()
            .for_each(|point| point.x = point.x + x_min.abs());
    }
    if y_min < 0 {
        y_count += y_min.abs();
        points
            .iter_mut()
            .for_each(|point| point.y = point.y + y_min.abs());
    }
    println!("{}, {}, {}, {}", x_count, y_count, x_min, y_min);

    let mut grid: Vec<Vec<bool>> = vec![vec![false; x_count as usize + 1]; y_count as usize + 1];

    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        let point1 = points.get(i).unwrap();
        let point2 = points.get(j).unwrap();
        if point1.x == point2.x {
            for y in point1.y.min(point2.y)..=point1.y.max(point2.y) {
                *grid
                    .get_mut(y as usize)
                    .unwrap()
                    .get_mut(point1.x as usize)
                    .unwrap() = true;
            }
        } else {
            for x in point1.x.min(point2.x)..=point1.x.max(point2.x) {
                *grid
                    .get_mut(point1.y as usize)
                    .unwrap()
                    .get_mut(x as usize)
                    .unwrap() = true;
            }
        }
    }

    print_grid(&grid);

    // Input a point that is inside the loop
    let mut input = String::new();
    println!("Gib point x inside");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let x_start: i64 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Gib point y inside");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let y_start: i64 = input.trim().parse().unwrap();

    println!("Starting to fill in from {} {}", x_start, y_start);

    let mut stack: Vec<Point> = vec![Point {
        x: x_start,
        y: y_start,
    }];

    while !stack.is_empty() {
        let Point { x, y } = stack.pop().unwrap();

        // Up
        if y - 1 >= 0 && !grid.get(y as usize - 1).unwrap().get(x as usize).unwrap() {
            *grid
                .get_mut(y as usize - 1)
                .unwrap()
                .get_mut(x as usize)
                .unwrap() = true;
            stack.push(Point { x, y: y - 1 });
        }
        // Down
        if y + 1 < y_count && !grid.get(y as usize + 1).unwrap().get(x as usize).unwrap() {
            *grid
                .get_mut(y as usize + 1)
                .unwrap()
                .get_mut(x as usize)
                .unwrap() = true;
            stack.push(Point { x, y: y + 1 });
        }
        // Left
        if x - 1 >= 0 && !grid.get(y as usize).unwrap().get(x as usize - 1).unwrap() {
            *grid
                .get_mut(y as usize)
                .unwrap()
                .get_mut(x as usize - 1)
                .unwrap() = true;
            stack.push(Point { x: x - 1, y });
        }
        // Right
        if x + 1 < x_count && !grid.get(y as usize).unwrap().get(x as usize + 1).unwrap() {
            *grid
                .get_mut(y as usize)
                .unwrap()
                .get_mut(x as usize + 1)
                .unwrap() = true;
            stack.push(Point { x: x + 1, y });
        }
    }

    print_grid(&grid);

    println!(
        "Result part1: {}",
        grid.iter()
            .map(|line| line
                .iter()
                .map(|tile| match tile {
                    true => 1,
                    false => 0,
                })
                .sum::<usize>())
            .sum::<usize>()
    );
}

fn part2(lines: Vec<String>) {
    let mut points: Vec<Point> = vec![Point { x: 0, y: 0 }];
    for line in lines.iter() {
        let split: Vec<&str> = line.split(" ").collect();

        let color = split.get(2).unwrap();
        // Remove #( and )
        let mut chars = color.chars();
        chars.next();
        chars.next();
        chars.next_back();
        let color = chars.as_str();

        // Extract distance
        let mut chars = color.chars();
        chars.next_back();
        let distance = i64::from_str_radix(chars.as_str(), 16).unwrap();
        let direction = Direction::new(color.chars().last().unwrap());

        let (x_change, y_change) = direction.get_change(distance);
        let last_point = points.last().unwrap();
        points.push(Point {
            x: last_point.x + x_change,
            y: last_point.y + y_change,
        });
    }

    let first_point = points.first().unwrap();
    let last_point = points.last().unwrap();

    println!(
        "Result part2: {}",
        (points
            .windows(2)
            // Stolen math from wikipedia
            .map(
                |points| points[0].x * points[1].y - points[0].y * points[1].x
                    + (points[0].x - points[1].x).abs()
                    + (points[0].y - points[1].y).abs()
            )
            .sum::<i64>()
            // Last point and first point don't get captured in the iteration togeather
            + (last_point.x * first_point.y - last_point.y * first_point.x).abs())
            / 2
            // Don't know why, but the result is off by one so slap it in here
            + 1
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines: Vec<String> = BufReader::new(File::open(args.get(1).unwrap()).unwrap())
        .lines()
        .map(|line_result| line_result.unwrap())
        .collect();

    part1(lines.clone());
    part2(lines);
}
