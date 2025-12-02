use std::env;
use std::fs;

fn puzzle1(input: &String) {}

fn puzzle2(input: &String) {}

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
