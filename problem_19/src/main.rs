use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum Property {
    Extreme,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Property {
    fn new(s: &str) -> Property {
        match s {
            "x" => Property::Extreme,
            "m" => Property::Musical,
            "a" => Property::Aerodynamic,
            "s" => Property::Shiny,
            _ => unreachable!("What the heck is this '{}' property?", s),
        }
    }

    fn index(&self) -> usize {
        match self {
            Property::Extreme => 0,
            Property::Musical => 1,
            Property::Aerodynamic => 2,
            Property::Shiny => 3,
        }
    }
}

#[derive(Debug)]
enum ConditionType {
    Less,
    Greater,
}

#[derive(Debug)]
struct Condition {
    property: Property,
    condition_type: ConditionType,
    check_var: i64,
    destination_state: String,
}

impl Condition {
    fn new(
        property: Property,
        condition_type: ConditionType,
        check_var: i64,
        destination_state: &str,
    ) -> Condition {
        Condition {
            property,
            condition_type,
            check_var,
            destination_state: String::from(destination_state),
        }
    }

    fn check(&self, gear: &Gear) -> bool {
        match self.condition_type {
            ConditionType::Less => gear.get_property(self.property) < self.check_var,
            ConditionType::Greater => gear.get_property(self.property) > self.check_var,
        }
    }
}

#[derive(Debug)]
struct Gear {
    extreme: i64,
    musical: i64,
    aerodynamic: i64,
    shiny: i64,
}

impl Gear {
    fn new(extreme: i64, musical: i64, aerodynamic: i64, shiny: i64) -> Gear {
        Gear {
            extreme,
            musical,
            aerodynamic,
            shiny,
        }
    }

    fn get_property(&self, property: Property) -> i64 {
        match property {
            Property::Extreme => self.extreme,
            Property::Musical => self.musical,
            Property::Aerodynamic => self.aerodynamic,
            Property::Shiny => self.shiny,
        }
    }

    fn get_sum(&self) -> i64 {
        self.extreme + self.musical + self.aerodynamic + self.shiny
    }
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Range {
        if start > end {
            return Range { start: 0, end: 0 };
        }
        Range { start, end }
    }

    fn get_ranges_combinations(ranges: &Vec<Range>) -> i64 {
        ranges.iter().map(|range| range.len()).product()
    }

    fn len(&self) -> i64 {
        self.end - self.start + 1
    }

    fn as_tuple(&self) -> (i64, i64) {
        (self.start, self.end)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines: Vec<String> = BufReader::new(File::open(args.get(1).unwrap()).unwrap())
        .lines()
        .map(|line_result| line_result.unwrap())
        .collect();

    let mut workflows: HashMap<String, (Vec<Condition>, String)> = HashMap::new();
    let mut gears: Vec<Gear> = vec![];

    let mut switch = false;
    lines.iter().for_each(|line| {
        if line.is_empty() {
            switch = true;
            return;
        }
        if switch {
            let mut chars = line.chars();
            chars.next();
            chars.next_back();
            let split: Vec<&str> = chars.as_str().split(",").collect();
            // Plz ignore this ugly code
            gears.push(Gear::new(
                split
                    .get(0)
                    .unwrap()
                    .split("=")
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap(),
                split
                    .get(1)
                    .unwrap()
                    .split("=")
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap(),
                split
                    .get(2)
                    .unwrap()
                    .split("=")
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap(),
                split
                    .get(3)
                    .unwrap()
                    .split("=")
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap(),
            ));
            return;
        }

        let split: Vec<&str> = line.split("{").collect();
        let workflow_name = String::from(*split.get(0).unwrap());
        let rules = *split.get(1).unwrap();
        let rules: Vec<&str> = rules[0..rules.len() - 1].split(",").collect();

        let mut conditions: Vec<Condition> = Vec::new();

        for i in 0..rules.len() - 1 {
            let rule = *rules.get(i).unwrap();
            let split: Vec<&str> = rule.split(":").collect();
            let challenge = *split.get(0).unwrap();
            let success_destination = *split.get(1).unwrap();

            if challenge.contains(">") {
                let split: Vec<&str> = challenge.split(">").collect();
                let property = Property::new(*split.get(0).unwrap());
                let condition = Condition::new(
                    property,
                    ConditionType::Greater,
                    split.get(1).unwrap().parse().unwrap(),
                    success_destination,
                );
                conditions.push(condition);
            } else if challenge.contains("<") {
                let split: Vec<&str> = challenge.split("<").collect();
                let property = Property::new(*split.get(0).unwrap());
                let condition = Condition::new(
                    property,
                    ConditionType::Less,
                    split.get(1).unwrap().parse().unwrap(),
                    success_destination,
                );
                conditions.push(condition);
            } else {
                unreachable!("what the heck is this '{}' challenge?", challenge);
            }
        }

        workflows.insert(
            workflow_name,
            (conditions, rules.last().unwrap().to_string()),
        );
    });

    // Part 1
    let mut result_part1: i64 = 0;
    gears.iter().for_each(|gear| {
        let mut current_workflow = String::from("in");
        loop {
            let workflow = workflows.get(&current_workflow).unwrap();
            let mut found = false;
            let mut exit = false;
            for condition in workflow.0.iter() {
                if condition.check(gear) {
                    match condition.destination_state.as_str() {
                        "A" => {
                            result_part1 += gear.get_sum();
                            exit = true;
                            break;
                        }
                        "R" => {
                            exit = true;
                            break;
                        }
                        _ => {
                            current_workflow = condition.destination_state.to_string();
                            found = true;
                            break;
                        }
                    };
                }
            }
            if exit {
                break;
            }
            if !found {
                match workflow.1.as_str() {
                    "A" => {
                        result_part1 += gear.get_sum();
                        break;
                    }
                    "R" => break,
                    _ => current_workflow = workflow.1.to_string(),
                };
            }
        }
    });
    println!("Result part1: {}", result_part1);

    // Part 2
    let mut result_part2: i64 = 0;

    let mut stack: Vec<(Vec<Range>, String)> =
        vec![(vec![Range::new(1, 4000); 4], "in".to_string())];

    while let Some((mut ranges, workflow_name)) = stack.pop() {
        println!("===========================");
        println!("Processing {:?}, entering at '{}'", ranges, workflow_name);
        if workflow_name == "A" {
            println!("Accepted");
            result_part2 += Range::get_ranges_combinations(&ranges);
            continue;
        } else if workflow_name == "R" {
            println!("Rejected");
            continue;
        }

        let workflow = workflows.get(&workflow_name).unwrap();
        for condition in workflow.0.iter() {
            let mut continuing_ranges = ranges.clone();
            let index = condition.property.index();
            let (start, end) = continuing_ranges.get(index).unwrap().as_tuple();
            match condition.condition_type {
                ConditionType::Less => {
                    *continuing_ranges.get_mut(index).unwrap() =
                        Range::new(start, end.min(condition.check_var - 1));
                    *ranges.get_mut(index).unwrap() =
                        Range::new(start.max(condition.check_var), end);
                }
                ConditionType::Greater => {
                    *continuing_ranges.get_mut(index).unwrap() =
                        Range::new(start.max(condition.check_var + 1), end);
                    *ranges.get_mut(index).unwrap() =
                        Range::new(start, end.min(condition.check_var));
                }
            }
            println!("-----------");
            println!(
                "{:?}\nContinue range: {:?} to '{}' \nRemaining: {:?}",
                condition,
                continuing_ranges,
                condition.destination_state.to_string(),
                ranges
            );
            stack.push((continuing_ranges, condition.destination_state.to_string()));
        }
        println!("Remaining to '{}'", workflow.1.to_string());
        stack.push((ranges, workflow.1.to_string()));
    }

    println!("Result part2: {}", result_part2);
}
