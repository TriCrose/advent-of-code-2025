use std::env;
use std::fs;

fn puzzle1(input: &String) {
    let mut current = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let dir: char = line[..1].parse().expect("Direction should be a char");
        let dist: u32 = line[1..].parse().expect("Distance should be a u32");

        let increment = if dir == 'L' { 100 - (dist % 100) } else {dist};
        current = (current + increment) % 100;
        if current == 0 {
            zero_count += 1;
        }
    }

    dbg!(current);
    dbg!(zero_count);
    println!("The password is '{zero_count}'!");
}

fn puzzle2(input: &String) {
    let mut current = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let dir: char = line[..1].parse().expect("Direction should be a char");
        let dist: u32 = line[1..].parse().expect("Distance should be a u32");

        // If we're going left then mirror the dial value before and after the calculation
        if dir == 'L' {
            current = (100 - current) % 100;
        }

        zero_count += dist / 100;
        current += dist % 100;
        if current >= 100 {
            zero_count += 1;
            current %= 100;
        }

        if dir == 'L' {
            current = (100 - current) % 100;
        }

        // println!("After {} we are at {} (zeroes: {})", line, current, zero_count);
    }

    dbg!(current);
    dbg!(zero_count);
    println!("The password is '{zero_count}'!");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let puzzle_num: u32 = args[1].parse().expect("First argument must the puzzle the solve (1 or 2)");
    let input_filename = &args[2];
    dbg!(input_filename);

    let contents = fs::read_to_string(input_filename).expect("Couldn't open and read {input_filename}");

    println!("Solving problem {puzzle_num}...");
    match puzzle_num {
        1 => puzzle1(&contents),
        2 => puzzle2(&contents),
        _ => println!("Puzzle must be either '1' or '2'")
    }
}
