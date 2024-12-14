use aoc2024::read_input_to_file;
use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

#[derive(Debug, PartialEq)]
struct Antenna {
    frequency: char,
    coords: Coords,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

struct Groups(HashMap<char, Vec<Antenna>>);

impl Groups {
    fn new(antennas: Vec<Antenna>) -> Groups {
        let mut m: HashMap<char, Vec<Antenna>> = HashMap::new();
        antennas
            .into_iter()
            .for_each(|a| m.entry(a.frequency).or_default().push(a));
        Groups(m)
    }

    fn check(self, map_coords: Vec<Coords>) -> HashSet<Coords> {
        let mut anti = HashSet::new();

        for (freq, antenna) in self.0.iter() {
            // If there are less than 2 antennas with this frequency, skip
            if antenna.len() < 2 {
                continue;
            }

            for a in antenna {
                for b in antenna {
                    // Skip if a and b are the same point
                    if a == b {
                        continue;
                    }

                    for c in map_coords.iter() {
                        // Check if c is collinear with a and b
                        if are_collinear(&a.coords, &b.coords, c) {
                            anti.insert(Coords { x: c.x, y: c.y });
                        }
                    }
                }
            }

            // Add all antennas with this frequency as antinodes
            for a in antenna {
                anti.insert(a.coords.clone());
            }
        }

        anti
    }
}

#[derive(Clone)]
struct Map(Vec<Vec<char>>);

impl Map {
    fn coords(self) -> Vec<Coords> {
        let mut c = vec![];

        for (x, row) in self.0.iter().enumerate() {
            for (y, &_cell) in row.iter().enumerate() {
                c.push(Coords { x, y })
            }
        }
        c
    }

    fn print(&self) {
        for x in self.0.iter() {
            // println!("{:?}", x.join("--").collect())
            println!("{:?}", x.iter().collect::<String>());
        }
    }

    fn add_antinodes(&mut self, v: &HashSet<Coords>) {
        for c in v {
            if self.0[c.x][c.y] == '.' {
                self.0[c.x][c.y] = '#';
            }
        }
    }
}

fn distance(a: &Coords, b: &Coords) -> i32 {
    ((b.x as i32 - a.x as i32).pow(2) + (b.y as i32 - a.y as i32).pow(2)).abs()
}

fn are_collinear(a: &Coords, b: &Coords, c: &Coords) -> bool {
    // First, check if points are collinear
    (b.x as i32 - a.x as i32) * (c.y as i32 - a.y as i32)
        == (c.x as i32 - a.x as i32) * (b.y as i32 - a.y as i32)

    // if !collinear {
    //     return false;
    // }
    // true

    // // Calculate distances
    // let dist_ab = ((b.x as i32 - a.x as i32).pow(2) + (b.y as i32 - a.y as i32).pow(2)).abs();
    // let dist_ac = ((c.x as i32 - a.x as i32).pow(2) + (c.y as i32 - a.y as i32).pow(2)).abs();
    //
    // // Check if distances are equal
    // dist_ab == dist_ac
}

fn load(input_f: &str) -> (Map, Vec<Antenna>) {
    let mut file = read_input_to_file(input_f);
    let mut antennas = vec![];
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {
            let map: Vec<Vec<char>> = s
                .trim() // Remove leading/trailing whitespace
                .split('\n')
                .enumerate()
                .map(|(x, l)| {
                    l.chars()
                        .enumerate()
                        .map(|(y, c)| {
                            if c != '.' {
                                antennas.push(Antenna {
                                    frequency: c,
                                    coords: Coords { x, y },
                                })
                            };
                            c
                        })
                        .collect()
                })
                .collect();
            // println!("{:?}", antennas);
            (Map(map), antennas)
        }
        Err(e) => panic!("{:?}", e),
    }
}

fn main() {
    // let (mut map, antennas) = load("inputs/example_input08.txt");
    let (mut map, antennas) = load("inputs/input08.txt");
    //

    let groups = Groups::new(antennas);

    let antinodes = groups.check(map.clone().coords());

    map.add_antinodes(&antinodes);
    map.print();

    println!("{:?}", antinodes.len())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_exemple_data() {
        let (map, antennas) = load("inputs/example_input08.txt");
        // let (map, antennas) = load("inputs/input08.txt");

        let groups = Groups::new(antennas);

        let antinodes = groups.check(map.coords());
        // assert_eq!(antinodes.len(), 14);
        assert_eq!(antinodes.len(), 34);
    }
}
