use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write, BufRead};
use std::path::Path;

fn main() {
    let p1: i32 = part1();
    let p2: i32 = part2();

    // This is basically just from Rust by Example and writes the solutions to file
    let path = Path::new("./output/day03.txt");
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

/// Split lines and go through half and call contains to check
fn part1() -> i32 {
    let mut priority_sum: i32 = 0;

    if let Ok(lines) = read_lines("./input/rucksack.txt") {
        for line in lines {
            if let Ok(ip) = line {
                // for every char in first half of string, check if second half contains it
                // println!("{ip}");
                let length: usize = ip.chars().count();
                let (s1, s2) = ip.split_at(length / 2);
                for c in s1.chars() {
                    if s2.contains(c) {
                        priority_sum += get_priority(c.to_string());
                        break;
                    }
                }
            }
        }
    }

    return priority_sum;
}

/// Easiest method but not efficient method is iterate through one string and check other two
/// and find the common char in all of them
/// I think this is O(n^2) if contains() is O(n)
fn part2() -> i32 {
    let mut priority_sum: i32 = 0;
    let mut str_array: [String; 2] = Default::default();

    if let Ok(lines) = read_lines("./input/rucksack.txt") {
        for (ix, line) in lines.enumerate() {
            if let Ok(ip) = line {
                if ix % 3 == 0 {
                    str_array[0] = ip;
                } else if ix % 3 == 1 {
                    str_array[1] = ip;
                } else {
                    for c in ip.chars() {
                        if str_array[0].contains(c) && str_array[1].contains(c) {
                            priority_sum += get_priority(c.to_string());
                            break;
                        }
                    }
                }
            }
        }
    }

    return priority_sum;
}

/// Very dumb not space-efficient solution to getting priorities
fn get_priority(c: String) -> i32 {
    let priorities: HashMap<String, i32> = HashMap::from([
        ('a'.to_string(), 1),
        ('b'.to_string(), 2),
        ('c'.to_string(), 3),
        ('d'.to_string(), 4),
        ('e'.to_string(), 5),
        ('f'.to_string(), 6),
        ('g'.to_string(), 7),
        ('h'.to_string(), 8),
        ('i'.to_string(), 9),
        ('j'.to_string(), 10),
        ('k'.to_string(), 11),
        ('l'.to_string(), 12),
        ('m'.to_string(), 13),
        ('n'.to_string(), 14),
        ('o'.to_string(), 15),
        ('p'.to_string(), 16),
        ('q'.to_string(), 17),
        ('r'.to_string(), 18),
        ('s'.to_string(), 19),
        ('t'.to_string(), 20),
        ('u'.to_string(), 21),
        ('v'.to_string(), 22),
        ('w'.to_string(), 23),
        ('x'.to_string(), 24),
        ('y'.to_string(), 25),
        ('z'.to_string(), 26),
        ('A'.to_string(), 27),
        ('B'.to_string(), 28),
        ('C'.to_string(), 29),
        ('D'.to_string(), 30),
        ('E'.to_string(), 31),
        ('F'.to_string(), 32),
        ('G'.to_string(), 33),
        ('H'.to_string(), 34),
        ('I'.to_string(), 35),
        ('J'.to_string(), 36),
        ('K'.to_string(), 37),
        ('L'.to_string(), 38),
        ('M'.to_string(), 39),
        ('N'.to_string(), 40),
        ('O'.to_string(), 41),
        ('P'.to_string(), 42),
        ('Q'.to_string(), 43),
        ('R'.to_string(), 44),
        ('S'.to_string(), 45),
        ('T'.to_string(), 46),
        ('U'.to_string(), 47),
        ('V'.to_string(), 48),
        ('W'.to_string(), 49),
        ('X'.to_string(), 50),
        ('Y'.to_string(), 51),
        ('Z'.to_string(), 52),
    ]);

    match priorities.get(&c) {
        Some(value) => return *value,
        None => panic!("Wrong key {}", c),
    }
}

/// From Rust By Example
/// Opens file, runs lines() for error checking
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
