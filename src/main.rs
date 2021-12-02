use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day2_2() {
    let mut commands: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./data/2-1-input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(command) = line {
                commands.push(command);
            }
        }
    }
    let mut depth: i64 = 0;
    let mut horizontal: i64 = 0;
    let mut aim: i64 = 0;
    for command in commands {
        println!("Command: {}", command);
        let mut split = command.split_whitespace();
        let direction = split.next().unwrap();
        match direction {
            "forward" => {
                let forward = split.next().unwrap().parse::<i64>().unwrap();
                horizontal += forward;
                depth += forward * aim;
                println!("Moving forward {} with aim {} Depth: {} Horizontal: {}", forward, aim, depth, horizontal)
            }
            "up" => {
                let up = split.next().unwrap().parse::<i64>().unwrap();
                aim -= up;
                println!("Aiming up {}, aim now {}", up, aim);
            }
            "down" => {
                let down = split.next().unwrap().parse::<i64>().unwrap();
                aim += down;
                println!("Aiming down {}, aim now {}", down, aim);
            }
            _ => println!("Unmatched case {}", direction)
        }
    }
    println!("Depth {} Horizontal {} Mult {}", depth, horizontal, depth * horizontal);
}

fn day2_1() {
    let mut commands: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./data/2-1-input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(command) = line {
                commands.push(command);
            }
        }
    }
    let mut depth = 0;
    let mut horizontal = 0;
    for command in commands {
        let mut split = command.split_whitespace();
        let direction = split.next().unwrap();
        match direction {
            "forward" => horizontal += split.next().unwrap().parse::<i32>().unwrap(),
            "up" => depth -= split.next().unwrap().parse::<i32>().unwrap(),
            "down" => depth += split.next().unwrap().parse::<i32>().unwrap(),
            _ => println!("Unmatched case {}", direction)
        }
    }
    println!("Depth {} Horizontal {} Mult {}", depth, horizontal, depth * horizontal);
}

fn day1_2() {
    let mut depths: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./data/1-1-input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(depth) = line.unwrap().parse::<i32>() {
                depths.push(depth);
            }
        }
    }

    let sliding_window = depths.windows(3);
    let mut total = 0;
    let mut prev = 0;
    let mut cur = 0;
    for window in sliding_window {
        cur = window.iter().sum();
        if prev == 0 {
            prev = cur;
            continue;
        }
        if cur > prev {
            total += 1;
        }
        prev = cur;
    }
    println!("Depth increased {} times.", total);
}

fn day1_1() {
    // File hosts must exist in current path before this produces output
    let mut total = 0;
    let mut prev = 0;

    if let Ok(lines) = read_lines("./data/1-1-input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(depth) = line.unwrap().parse::<i32>() {
                if prev == 0 {
                    prev = depth;
                    continue;
                }
                if depth > prev {
                    total += 1;
                }
                prev = depth;
            }
        }
    }
    println!("Depth increased {} times.", total);
}


fn main() {
    day2_2();
}
