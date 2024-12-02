// https://adventofcode.com/2024/day/2
//
// Part 1
// - open file into a vector of Reports
// - check if levels are increasing/decreasing
// - check by how much they are increasing
// - judge if they are safe
// Part 2
// - apply problem dampener; do this by generating all possible levels
// - check if any of the possible levels are safe

use core::panic;
use std::{cmp::Ordering, collections::HashSet, fs::File, io::Read, path::Path};

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Direction {
    Increasing,
    Decreasing,
    Invalid,
}

impl Report {
    fn new(levels: Vec<i32>) -> Report {
        Report { levels }
    }

    fn has_valid_direction(&self) -> bool {
        let mut r: HashSet<Direction> = HashSet::new();

        self.levels.windows(2).for_each(|w| {
            r.insert(match w[0].cmp(&w[1]) {
                Ordering::Greater => Direction::Increasing,
                Ordering::Less => Direction::Decreasing,
                Ordering::Equal => Direction::Invalid,
            });
        });

        r.len() == 1
    }

    fn change_range(&self) -> i32 {
        let change: Vec<i32> = self
            .levels
            .windows(2)
            .map(|w| (w[0] - w[1]).abs())
            .collect();

        match change.iter().max() {
            Some(v) => *v,
            None => 0,
        }
    }

    fn is_safe(&self) -> bool {
        // apply problem dampener!
        //  -- generate all possible levels and check if any of them are safe
        let pos_reports = &self.generate_level_pos();

        let mut safe_possibilities: Vec<bool> = vec![];
        pos_reports.iter().for_each(|r| {
            if r.change_range() <= 3 && r.has_valid_direction() {
                safe_possibilities.push(true);
            } else {
                safe_possibilities.push(false);
            }
        });

        safe_possibilities.iter().any(|&x| x)
    }

    fn generate_level_pos(&self) -> Vec<Report> {
        let mut c = 0;
        let r = self
            .levels
            .iter()
            .map(|_| {
                let mut new_levels = self.levels.clone();
                new_levels.remove(c);
                c += 1;
                Report::new(new_levels)
            })
            .collect();

        r
    }
}

fn load_reports(input_f: &str) -> Vec<Report> {
    // load
    let input_f = Path::new(input_f);
    let mut file = match File::open(input_f) {
        Ok(file) => file,
        Err(e) => panic!("could not open file {}: {}!", input_f.display(), e),
    };

    let mut reports: Vec<Report> = vec![];

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {
            let lines: Vec<&str> = s.split("\n").collect();
            lines.iter().for_each(|l| {
                let levels: Vec<i32> = l.split_whitespace().map(|v| v.parse().unwrap()).collect();
                if !levels.is_empty() {
                    reports.push(Report::new(levels))
                }
            })
        }
        Err(e) => panic!("{:?}", e),
    }
    reports
}

fn main() {
    let reports = load_reports("inputs/day02.txt");

    let safe_reports = reports.iter().filter(|r| r.is_safe()).count();

    println!("{:?} reports are safe", safe_reports)
}
