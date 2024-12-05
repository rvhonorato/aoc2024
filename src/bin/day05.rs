// https://adventofcode.com/2024/day/5
//
// Part 1
// - read input into page ordering rules
// - read input into page updates
//
//
use aoc2024::read_input_to_file;
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

fn load_data(input_f: &str) -> (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>) {
    let mut file = read_input_to_file(input_f);

    let mut s = String::new();
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = vec![];
    match file.read_to_string(&mut s) {
        Ok(_) => {
            let lines: Vec<&str> = s.split("\n").collect();
            lines.iter().for_each(|l| {
                if l.contains('|') {
                    let _l: Vec<&str> = l.split('|').collect();
                    let k: usize = _l[0].parse().unwrap();
                    let v: usize = _l[1].parse().unwrap();
                    rules.entry(k).or_default().insert(v);
                } else if !l.is_empty() {
                    let u: Vec<usize> = l.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
                    updates.push(u);
                }
            });
        }
        Err(e) => panic!("{:?}", e),
    };
    (rules, updates)
}

fn is_manual_valid(manual: &Vec<usize>, rules: &HashMap<usize, HashSet<usize>>) -> bool {
    let mut valid_pages: Vec<bool> = vec![];
    manual.iter().enumerate().for_each(|(i, page)| {
        let behind = &manual[..i];
        let ahead = &manual[i + 1..];
        for page_ahead in ahead {
            // println!("checking page ahead {:?}", page_ahead);
            if let Some(p) = rules.get(page) {
                if p.contains(page_ahead) {
                    valid_pages.push(true);
                } else {
                    valid_pages.push(false);
                }
            }
        }

        // check if pages behind are allowed
        for page_behind in behind {
            if let Some(p) = rules.get(page) {
                if p.contains(page_behind) {
                    valid_pages.push(false);
                } else {
                    valid_pages.push(true);
                }
            }
        }
    });
    valid_pages.iter().all(|x| *x)
}

fn generate_swaps(vec: Vec<usize>) -> Vec<Vec<usize>> {
    let mut swapped_versions = Vec::new();

    for i in 0..vec.len() {
        for j in (i + 1)..vec.len() {
            let mut swapped = vec.to_vec();
            swapped.swap(i, j);
            swapped_versions.push(swapped);
        }
    }

    println!("{:?}", swapped_versions);

    swapped_versions
}

fn main() {
    let (rules, updates) = load_data("inputs/input05.txt");
    // let (rules, updates) = load_data("inputs/example_input05.txt");
    let mut result: Vec<usize> = vec![];
    let mut incorrect_manuals = vec![];
    updates.iter().for_each(|manual| {
        if is_manual_valid(manual, &rules) {
            // println!("✅ {:?}", manual);
            // get the middle number
            result.push(manual[manual.len() / 2])
        } else {
            incorrect_manuals.push(manual);
            // println!("❌ {:?}", manual)
        }
    });
    println!("result {:?}", result.iter().sum::<usize>());

    // Part 2
    //  brute force the incorrect_manuals until they are correct
    // let mut fixed_manuals: Vec<Vec<usize>> = vec![];
    // let mut result: Vec<usize> = vec![];
    // let total = incorrect_manuals.len();
    // let mut counter = 0;
    // incorrect_manuals.iter().for_each(|m| {
    //     println!("{:?}", m);
    //     println!("{:?}", total - counter);
    //     m.iter()
    //         .cloned()
    //         .permutations(m.len())
    //         .find(|p| is_manual_valid(p, &rules))
    //         .map(|p| {
    //             result.push(p[p.len() / 2]);
    //             fixed_manuals.push(p);
    //         });
    //     counter += 1;
    // });

    let mut total = 0;
    let (result, fixed_manuals): (Vec<usize>, Vec<Vec<usize>>) = incorrect_manuals
        .par_iter()
        .filter_map(|m| {
            m.iter()
                .cloned()
                .permutations(m.len())
                .find(|p| is_manual_valid(p, &rules))
                .map(|p| (p[p.len() / 2], p))
        })
        .unzip();
    println!("fixed result {:?}", result.iter().sum::<usize>());
}
