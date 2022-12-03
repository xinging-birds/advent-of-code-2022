use std::fs::File;
use std::io::{self, Write, BufRead};
use std::path::Path;

fn main() {
    let p1: i32 = part1();
    let p2: i32 = part2();

    // This is basically just from Rust by Example and writes the solutions to file
    let path = Path::new("./output/template.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match write!(file, "Part 1: {}\nPart 2: {}", p1, p2) {
        Err(why) => panic!("Couldn't write to {display}: {}", why),
        Ok(_) => println!("Successfully wrote solution to {}", display),
    }
}

/// Use hashmap to calculate score easily
fn part1() -> i32 {
    if let Ok(lines) = read_lines("./input/template.txt") {
        for line in lines {
            if let Ok(ip) = line {
                // Code goes here
                println!("{ip}");
            }
        }
    }

    return 0;
}

/// Same as part 1 but now the hashmap key is different to indicate lose/draw/win
fn part2() -> i32 {
    if let Ok(lines) = read_lines("./input/template.txt") {
        for line in lines {
            if let Ok(ip) = line {
               // Code goes here
               println!("{ip}");
            }
        }
    }

    return 0;
}

/// From Rust By Example
/// Opens file, runs lines() for error checking
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
