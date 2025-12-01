use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let input_filename = &args[1];
    dbg!(input_filename);

    let contents = fs::read_to_string(input_filename).expect("Couldn't open and read {input_filename}");

    let mut current = 50;
    let mut zero_count = 0;

    for line in contents.lines() {
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
}
