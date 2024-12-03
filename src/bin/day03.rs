// https://adventofcode.com/2024/day/3
//
// Part 1
// - compile regex
// - capture the matches in the input
// - do the multiplication operation
// - sum the results
// Part 2
// - compile new regex to identify the blocks
// - re-apply regex from part1
// - sum the results

use regex::Regex;

use core::panic;
use std::{fs::File, io::Read, path::Path};

fn read_file(input_f: &str) -> String {
    let input_f = Path::new(input_f);
    let mut file = match File::open(input_f) {
        Ok(file) => file,
        Err(e) => panic!("could not open file {}: {}!", input_f.display(), e),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {
            let lines: Vec<&str> = s.split("\n").collect();
            lines.join("")
        }
        Err(e) => panic!("{:?}", e),
    }
}

fn do_multiplications(m: &str) -> i32 {
    let r: Vec<i32> = m.split(',').map(|v| v.parse::<i32>().unwrap()).collect();
    r[0] * r[1]
}

fn main() {
    // Identify mul operations "NN,NN" https://regex101.com/r/YqXLQt/1
    let mul_re = Regex::new(r"ul\((\d+,\d+)\)").unwrap();

    // Identify commands blocks "do()xxxxdon't()" -https://regex101.com/r/x1X7y1/1
    let valid_block_re = Regex::new(r"(?m)(do\(\).*?(?:mul\(\d+,\d+\)).*?don't\(\))").unwrap();

    // Identify first command block "xxxxxdo()" - https://regex101.com/r/rxchKg/1
    let first_block_re = Regex::new(r"(?m)(^.*?(?:mul\(\d+,\d+\)).*?do\(\){1})").unwrap();

    // Load
    let input = read_file("inputs/input03.txt");

    // Part 1
    let mut result: Vec<i32> = vec![];
    for (_, [n]) in mul_re.captures_iter(&input).map(|c| c.extract()) {
        let r: Vec<i32> = n.split(',').map(|v| v.parse::<i32>().unwrap()).collect();
        result.push(r[0] * r[1])
    }

    let sum: i32 = result.iter().sum();
    println!("mul sum: {:?}", sum);

    // Part 2
    let mut result: Vec<i32> = vec![];

    // Capture valid blocks
    let mut valid_blocks: String = "".to_string();

    // First the first block
    for (_, [first_block]) in first_block_re.captures_iter(&input).map(|c| c.extract()) {
        valid_blocks += first_block;
    }

    // Get all the other blocks
    for (_, [block]) in valid_block_re.captures_iter(&input).map(|c| c.extract()) {
        valid_blocks += block;
    }

    // Do the multiplication for the valid blocks
    for (_, [j]) in mul_re.captures_iter(&valid_blocks).map(|c2| c2.extract()) {
        let mul = do_multiplications(j);
        result.push(mul)
    }

    let sum: i32 = result.iter().sum();
    println!("enabled mul sum: {:?}", sum);
}
