// https://adventofcode.com/2024/day/1
//
// Part 1
// - load the input file in two vectors
// - sort
// - pair and loop to find the distance between the numbers
// - sum the distances
// Part 2
// - count how many times each element in A appears in B
// - calculate simmiliarity by multipying sum(A.element * count)

use core::panic;
use std::{fs::File, io::Read, path::Path};

fn main() {
    // load
    let input_f = Path::new("inputs/input01.txt");
    let mut file = match File::open(input_f) {
        Ok(file) => file,
        Err(e) => panic!("could not open file {}: {}!", input_f.display(), e),
    };

    let mut list_a: Vec<i32> = vec![];
    let mut list_b: Vec<i32> = vec![];

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {
            let lines: Vec<&str> = s.split("\n").collect();
            lines.iter().for_each(|l| {
                let _l: Vec<&str> = l.split_whitespace().collect();
                if _l.len() == 2 {
                    list_a.push(_l[0].parse().unwrap());
                    list_b.push(_l[1].parse().unwrap())
                }
            })
        }
        Err(e) => panic!("{:?}", e),
    }

    // sort the lists
    list_a.sort();
    list_b.sort();

    // pair and calculate distance
    let results: Vec<i32> = list_a
        .iter()
        .zip(list_b.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();

    // sum
    let sum: i32 = results.iter().sum();

    println!("Sum: {:?}", sum);

    // count how many times each element of A appears in B
    let sim: Vec<i32> = list_a
        .iter()
        .map(|x| {
            let count = list_b.iter().filter(|y| x == *y).count() as i32;
            x * count
        })
        .collect();

    let sim_sum: i32 = sim.iter().sum();

    println!("Sim sum: {:?}", sim_sum)
}
