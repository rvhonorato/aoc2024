// https://adventofcode.com/2024/day/6
use aoc2024::read_input_to_file;
use std::{collections::HashSet, io::Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
#[derive(Clone)]
struct Guard {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Guard {
    fn new(x: i32, y: i32, direction: Direction) -> Self {
        Guard { x, y, direction }
    }

    fn walk(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn patrol(mut self, mut map: Map) -> usize {
        let mut visited = HashSet::new();

        visited.insert((self.x, self.y));

        while map.within_bounds(self.x, self.y) {
            // get the next step
            let (next_x, next_y) = match self.direction {
                Direction::Up => (self.x, self.y - 1),
                Direction::Right => (self.x + 1, self.y),
                Direction::Down => (self.x, self.y + 1),
                Direction::Left => (self.x - 1, self.y),
            };

            // if the guard fell of the map terminate the patrol
            if !map.within_bounds(next_x, next_y) {
                break;
            }

            if map.get_cell(next_x, next_y) == '#' {
                self.direction = self.direction.turn_right();
            } else {
                self.walk();
                map.mark_visited(self.x, self.y);
                visited.insert((self.x, self.y));
            };
            // map.print_map();
            // println!("========================");
            // thread::sleep(Duration::from_secs_f64(0.05));
        }
        visited.len()
    }

    fn is_stuck_in_loop(&mut self, map: Map) -> bool {
        let mut state_history = HashSet::new();

        while map.within_bounds(self.x, self.y) {
            // Create a unique state representation
            let current_state = (self.x, self.y, self.direction);

            // If we've seen this exact state before, we're in a loop
            if !state_history.insert(current_state) {
                return true;
            }

            let (next_x, next_y) = match self.direction {
                Direction::Up => (self.x, self.y - 1),
                Direction::Right => (self.x + 1, self.y),
                Direction::Down => (self.x, self.y + 1),
                Direction::Left => (self.x - 1, self.y),
            };

            // Check if out of bounds
            if !map.within_bounds(next_x, next_y) {
                return false;
            }

            if map.get_cell(next_x, next_y) == '#' {
                self.direction = self.direction.turn_right();
            } else {
                self.walk();
            }
        }
        false
    }
}

#[derive(Clone)]
struct Map(Vec<Vec<char>>);

impl Map {
    fn load(input_f: &str) -> Self {
        let mut file = read_input_to_file(input_f);
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(_) => {
                let map: Vec<Vec<char>> = s
                    .trim() // Remove leading/trailing whitespace
                    .split('\n')
                    .map(|l| l.chars().collect())
                    .collect();
                let m = Map(map);
                // m.print_map();
                m
            }
            Err(e) => panic!("{:?}", e),
        }
    }
    fn find_guard(&self) -> Guard {
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == '^' {
                    // println!("{:?}-{:?}", x, y);
                    return Guard::new(x as i32, y as i32, Direction::Up);
                }
            }
        }
        panic!("no guard on duty!")
    }

    fn mark_visited(&mut self, x: i32, y: i32) {
        self.0[y as usize][x as usize] = 'X'
    }

    // fn print_map(&self) {
    //     self.0.iter().for_each(|x| println!("{:?}", x));
    // }

    fn get_cell(&self, x: i32, y: i32) -> char {
        self.0[y as usize][x as usize]
    }
    fn within_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && y < self.0.len() as i32 && x < self.0[y as usize].len() as i32
    }
    fn time_loop(&self) -> usize {
        //
        // Initialize simulation!
        //
        // Create multiple copies of the Map
        //  Each copy will have an obstacle in a different position
        //  Place a guard in it and let it run the variant map
        //  If the get stuck, add one!
        //
        //  > NOTE: The guard will be stuck in the loop forever, but don't worry
        //  > it was just a clone, the real guard is safe. Clones don't feel anything...
        //  > hopefully
        //
        let mut loop_positions = 0;
        // Find the original guard
        let guard_start = self.find_guard();
        let mut total_maps = 0;
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                // Skip the guard's starting position
                if self.0[y][x] == '^' || self.0[y][x] == '#' {
                    continue;
                }
                // Create a copy of the map with a new obstacle
                let mut test_map = self.clone();
                test_map.0[y][x] = '#';

                // Create a clone of the guard to run this map
                let mut test_guard = guard_start.clone();

                // Observe if the clone is stuck
                if test_guard.is_stuck_in_loop(test_map) {
                    loop_positions += 1;
                }

                total_maps += 1;
            }
        }
        println!(
            "checked {:?} maps, {:?} guards are stuck",
            total_maps, loop_positions
        );
        loop_positions
    }
}

fn main() {
    // let map = Map::load("inputs/example_input06.txt");
    let map = Map::load("inputs/day06.txt");

    let guard = map.find_guard();

    let visited_locs = guard.patrol(map.clone());
    println!("visited: {:?}", visited_locs);

    let loop_positions = map.time_loop();
    println!("Positions that cause a loop: {}", loop_positions);
}
