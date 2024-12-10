// https://adventofcode.com/2024/day/7
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
enum Operators {
    Add,
    Multiply,
    Concatenate,
}

#[derive(Debug)]
struct Equation {
    numbers: Vec<i64>,
    result: i64,
}

impl Equation {
    fn solve(&self) -> i64 {
        let n = self.numbers.len() - 1;
        get_combination(n)
            .iter()
            .find_map(|ops| {
                let mut r: i64 = self.numbers[0];
                for (i, op) in ops.iter().enumerate() {
                    match op {
                        Operators::Add => r += self.numbers[i + 1],
                        Operators::Multiply => r *= self.numbers[i + 1],
                        Operators::Concatenate => {
                            let concat_str = format!("{}{}", r, self.numbers[i + 1]);
                            r = concat_str.parse().unwrap_or(0);
                        }
                    }
                }
                if r == self.result {
                    Some(r)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }
}

fn get_combination(n: usize) -> Vec<Vec<Operators>> {
    std::iter::repeat([Operators::Add, Operators::Multiply, Operators::Concatenate])
        .take(n)
        .multi_cartesian_product()
        .collect()
}

fn load_data(input_path: &str) -> Vec<Equation> {
    let file = File::open(input_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map_while(|line| line.ok())
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let result = parts[0].parse().expect("Invalid result");
            let numbers: Vec<i64> = parts[1]
                .split_whitespace()
                .map(|n| n.parse().expect("Invalid number"))
                .collect();
            Equation { numbers, result }
        })
        .collect()
}

fn main() {
    // let input = load_data("inputs/example_input07.txt");
    let input = load_data("inputs/input07.txt");

    let result: i64 = input.iter().map(|eq| eq.solve()).sum();

    println!("sum total equations: {:?}", result);
}
