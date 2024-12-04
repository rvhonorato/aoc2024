// https://adventofcode.com/2024/day/4
//
// Part 1
// - read the file into an `Item` struct
// - convert the Vector of Item into a hashmap
// - look into the hashmap for the letter at the relative locations
// - check if they spell `XMAS`
// Part 2
// - similar to part 1 look for letters at relative locations (diagonals)
// - check if they spell the pattern (the position is important)

use std::io::Read;

#[derive(Debug, Clone)]
struct Item {
    loc: [i32; 2],
    letter: char,
}

fn read_file(input_f: &str) -> Vec<Item> {
    let input_f = std::path::Path::new(input_f);
    let mut file: std::fs::File = match std::fs::File::open(input_f) {
        Ok(file) => file,
        Err(e) => panic!("could not open file {}: {}!", input_f.display(), e),
    };

    let mut s = String::new();
    let mut items: Vec<Item> = vec![];
    match file.read_to_string(&mut s) {
        Ok(_) => {
            let lines: Vec<&str> = s.split("\n").collect();
            lines.iter().enumerate().for_each(|(i, row)| {
                row.chars()
                    .collect::<Vec<char>>()
                    .iter()
                    .enumerate()
                    .for_each(|(j, letter)| {
                        items.push(Item {
                            loc: [i as i32, j as i32],
                            letter: *letter,
                        })
                    });
            });
        }
        Err(e) => panic!("{:?}", e),
    };
    items
}

fn is_xmas(sequence: &[char]) -> bool {
    sequence.len() == 4
        && sequence[0] == 'X'
        && sequence[1] == 'M'
        && sequence[2] == 'A'
        && sequence[3] == 'S'
}
fn check_diagonal(items: [&Item; 3]) -> bool {
    let first = items[0].letter;
    let second = items[1].letter;
    let third = items[2].letter;
    (first == 'M' && second == 'A' && third == 'S')
        || (first == 'S' && second == 'A' && third == 'M')
}

fn main() {
    let inp = read_file("inputs/day04.txt");

    // Part 1 - look form `XMAS` in horizontal, vertical, diagonal (both ways)
    let directions = [
        // Horizontal: (row change, col change)
        (0, 1),  // right
        (0, -1), // left
        // Vertical
        (1, 0),  // down
        (-1, 0), // up
        // Diagonal top-left to bottom-right
        (1, 1),
        (-1, -1),
        // Diagonal top-right to bottom-left
        (1, -1),
        (-1, 1),
    ];

    let item_map: std::collections::HashMap<_, _> = inp
        .clone()
        .into_iter()
        .map(|item| (item.loc, item))
        .collect();

    let mut xmas_counter = 0;
    for item in &inp {
        for &(row_delta, col_delta) in &directions {
            let mut word = vec![item.letter];
            let mut loc = item.loc;
            for _ in 0..3 {
                loc = [loc[0] + row_delta, loc[1] + col_delta];
                if let Some(next) = item_map.get(&loc) {
                    word.push(next.letter);
                } else {
                    break;
                }
                if word.len() == 4 && is_xmas(&word) {
                    xmas_counter += 1;
                }
            }
        }
    }
    println!("xmas_counter: {:?}", xmas_counter);

    // Part 2 find `MAS` in diagonals
    let mut mas_counter = 0;
    for item in inp.iter() {
        if item.letter == 'A' {
            let locations = [
                [item.loc[0] - 1, item.loc[1] - 1], // top-left
                [item.loc[0] + 1, item.loc[1] + 1], // bottom-right
                [item.loc[0] - 1, item.loc[1] + 1], // top-right
                [item.loc[0] + 1, item.loc[1] - 1], // bottom-left
            ];

            // Get items from the map corresponding to the diagonal locations
            let top_left = item_map.get(&locations[0]);
            let bottom_right = item_map.get(&locations[1]);
            let top_right = item_map.get(&locations[2]);
            let bottom_left = item_map.get(&locations[3]);

            // Check if we have a valid X-MAS pattern
            if let (Some(tl), Some(br), Some(tr), Some(bl)) =
                (top_left, bottom_right, top_right, bottom_left)
            {
                // First diagonal: check top-left and bottom-right
                let diagonal1 = [tl, item, br];
                let diagonal2 = [tr, item, bl];

                if check_diagonal(diagonal1) && check_diagonal(diagonal2) {
                    mas_counter += 1;
                }
            }
        }
    }
    println!("mas_counter: {:?}", mas_counter);
}
