use std::fs::File;
use std::io::{self, Write, BufRead};
use std::path::Path;

fn main() {
    let max: i32 = part1();
    let sum_max_3: i32 = part2();

    // This is basically just from Rust by Example and writes the solutions to file
    let path = Path::new("./output/day1.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // match file.write_all("{max}\n{sum_max_3}") {
    //     Err(why) => panic!("Couldn't write to {display}: {why}"),
    //     Ok(_) => println!("Successfully wrote solution to {display}"),
    // }

    match write!(file, "Part 1: {}\nPart 2: {}", max, sum_max_3) {
        Err(why) => panic!("Couldn't write to {display}: {}", why),
        Ok(_) => println!("Successfully wrote solution to {}", display),
    }
}

/// Simple max-checking function
fn part1() -> i32 {
    let mut max: i32 = 0;

    if let Ok(lines) = read_lines("./input/calories.txt") {
        let mut sum: i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let value: String = ip;
                // println!("Value: {}", value);
                // Empty line means all snacks counted, so check against max and set sum to 0
                if value.eq("") {
                    if sum > max { max = sum; }
                    sum = 0;
                // Add to max
                } else {
                    // println!("Sum: {}", sum);
                    sum += value.parse::<i32>().unwrap();
                }
            }
        }
    }

    return max;
}

/// Recursive check for top 3 maxes
fn part2() -> i32 {
    let mut maxes: [i32; 3] = [0; 3];  // descending order

    if let Ok(lines) = read_lines("./input/calories.txt") {
        let mut sum: i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let value: String = ip;

                if value.eq("") {
                    // New part recursively checks the maxes against the current sum and changes maxes accordingly
                    check_sum(sum, 3usize, &mut maxes);
                    sum = 0;
                } else {
                    sum += value.parse::<i32>().unwrap();
                }
            }
        }
    }

    return maxes.iter().sum();
}

/// Recursively check sums against maxes. Maxes are passed by reference.
/// num refers to the remaining maxes to check against.
fn check_sum(sum: i32, num: usize, maxes: &mut [i32; 3]) {
    if num == 0 {  // base case
        return;
    }
    
    if sum > maxes[3 - num] {
        let displaced = maxes[3 - num];
        maxes[3 - num] = sum;
        check_sum(displaced, num - 1, maxes);
    } else {
        check_sum(sum, num - 1, maxes);
    }
}

/// From Rust By Example
/// Opens file, runs lines() for error checking
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}