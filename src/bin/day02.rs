use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write, BufRead};
use std::path::Path;

fn main() {
    let score: i32 = part1();
    let sum_max_3: i32 = part2();

    // This is basically just from Rust by Example and writes the solutions to file
    let path = Path::new("./output/day02.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match write!(file, "Part 1: {}\nPart 2: {}", score, sum_max_3) {
        Err(why) => panic!("Couldn't write to {display}: {}", why),
        Ok(_) => println!("Successfully wrote solution to {}", display),
    }
}

/// Use hashmap to calculate score easily
fn part1() -> i32 {
    let move_score: HashMap<MovePair, i32> = HashMap::from([
        (MovePair {move1: "A".to_string(), move2: "X".to_string()}, 1 + 3),
        (MovePair {move1: "A".to_string(), move2: "Y".to_string()}, 2 + 6),
        (MovePair {move1: "A".to_string(), move2: "Z".to_string()}, 3 + 0),
        (MovePair {move1: "B".to_string(), move2: "X".to_string()}, 1 + 0),
        (MovePair {move1: "B".to_string(), move2: "Y".to_string()}, 2 + 3),
        (MovePair {move1: "B".to_string(), move2: "Z".to_string()}, 3 + 6),
        (MovePair {move1: "C".to_string(), move2: "X".to_string()}, 1 + 6),
        (MovePair {move1: "C".to_string(), move2: "Y".to_string()}, 2 + 0),
        (MovePair {move1: "C".to_string(), move2: "Z".to_string()}, 3 + 3),
    ]);

    let mut score: i32 = 0;

    if let Ok(lines) = read_lines("./input/rockpaperscissors.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip != "" {
                    // Split into two moves
                    let moves = ip.split_whitespace();
                    let moves_arr: Vec<&str> = moves.collect();

                    match move_score.get(&MovePair { move1: moves_arr[0].to_string(), move2: moves_arr[1].to_string() }) {
                        Some(p) => score += p,
                        None => panic!("Invalid MovePair: {} and {}", moves_arr[0], moves_arr[1]),
                    };
                }
            } else {
                panic!("Failed to read line");
            }
        }
    }

    return score;
}

/// Same as part 1 but now the hashmap key is different to indicate lose/draw/win
fn part2() -> i32 {
    let move_score: HashMap<Vec<&str>, i32> = HashMap::from([
        (vec!["A", "X"], 3 + 0),
        (vec!["A", "Y"], 1 + 3),
        (vec!["A", "Z"], 2 + 6),
        (vec!["B", "X"], 1 + 0),
        (vec!["B", "Y"], 2 + 3),
        (vec!["B", "Z"], 3 + 6),
        (vec!["C", "X"], 2 + 0),
        (vec!["C", "Y"], 3 + 3),
        (vec!["C", "Z"], 1 + 6),
    ]);

    let mut score = 0;

    if let Ok(lines) = read_lines("./input/rockpaperscissors.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip != "" {
                    // Split into two moves
                    let moves = ip.split_whitespace();
                    let moves_arr: Vec<&str> = moves.collect();

                    match move_score.get(&moves_arr) {
                        Some(p) => score += p,
                        None => panic!("Invalid MovePair: {} and {}", moves_arr[0], moves_arr[1]),
                    };
                }
            }
        }
    }

    return score;
}

/// From Rust By Example
/// Opens file, runs lines() for error checking
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Arguably easier as tuple but I wanna try using struct
#[derive(Hash, Eq, PartialEq, Debug)]
struct MovePair {
    move1: String,
    move2: String,
}