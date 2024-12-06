// https://adventofcode.com/2024/day/5
use aoc2024::read_input_to_file;
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

fn is_manual_valid(manual: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> bool {
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

fn topological_sort(manual: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    let mut sorted = Vec::new();
    let mut remaining: HashSet<usize> = manual.iter().cloned().collect();

    while !remaining.is_empty() {
        // Find a page that can be placed next
        if let Some(next_page) = remaining.clone().iter().find(|&page| {
            // Check if this page can be placed without violating any rules
            !remaining.iter().any(|&other| {
                other != *page
                    && rules
                        .get(&other)
                        .map_or(false, |after| after.contains(page))
            })
        }) {
            sorted.push(*next_page);
            remaining.remove(next_page);
        } else {
            // If no page can be placed, break to prevent infinite loop
            break;
        }
    }

    // If we couldn't sort all pages, add remaining pages
    sorted.extend(remaining);

    sorted
}

fn fix_incorrect_manual(manual: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    // Try topological sort first
    let sorted = topological_sort(manual, rules);

    // If the sorted version is valid, return it
    if is_manual_valid(&sorted, rules) {
        return sorted;
    }

    // If not, we might need a more complex approach
    // For now, return the original manual (you might want to enhance this)
    manual.to_vec()
}

fn main() {
    let (rules, updates) = load_data("inputs/day05.txt");
    // let (rules, updates) = load_data("inputs/example_day05.txt");

    // Part 1
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
    let mut result: Vec<usize> = vec![];
    incorrect_manuals.iter().for_each(|manual| {
        if is_manual_valid(manual, &rules) {
            // Correctly ordered update, use existing middle page
            result.push(manual[manual.len() / 2])
        } else {
            // Incorrect update, find correct ordering
            let corrected_manual = fix_incorrect_manual(manual, &rules);
            result.push(corrected_manual[corrected_manual.len() / 2]);
        }
    });

    println!("correct result: {:?}", result.iter().sum::<usize>());
}
