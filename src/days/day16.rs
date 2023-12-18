use crate::{Solution, SolutionPair};
use core::panic;
use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::BTreeSet;

///////////////////////////////////////////////////////////////////////////////

type Input = Vec<Vec<char>>;

type Point = (usize, usize);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct State {
    point: Point,
    direction: Direction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("input/input16.txt").expect("No input found for day 16!").trim_end().to_string();

    let input: Input = input_string.split('\n')
        .map(|line| line.chars().collect())
        .collect();

    // Your solution here...
    let sol1: u64 = part1(&input, State { point: (0, 0), direction: Direction::Right });
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &Input, initial_state: State) -> u64 {
    let mut visited: HashSet<State> = HashSet::new();
    let mut illuminated: HashSet<Point> = HashSet::new();

    let mut lights: BTreeSet<State> = BTreeSet::new();
    lights.insert(initial_state);

    while !lights.is_empty() {
        let mut light = lights.pop_first().unwrap();

        while light.point.0 < input.len() && light.point.1 < input[0].len() {
            // We put the state and the lights in the visited set

            //println!("Light: {:?}", light);

            if visited.insert(light) {
                illuminated.insert(light.point);
            } else {
                break;
            }

            if input[light.point.0][light.point.1] == '/' {
                match light.direction {
                    Direction::Up => light.direction = Direction::Right,
                    Direction::Down => light.direction = Direction::Left,
                    Direction::Left => light.direction = Direction::Down,
                    Direction::Right => light.direction = Direction::Up,
                }
            } else if input[light.point.0][light.point.1] == '\\' {
                match light.direction {
                    Direction::Up => light.direction = Direction::Left,
                    Direction::Down => light.direction = Direction::Right,
                    Direction::Left => light.direction = Direction::Up,
                    Direction::Right => light.direction = Direction::Down,
                }
            } else if input[light.point.0][light.point.1] == '|' {
                match light.direction {
                    Direction::Up => (),
                    Direction::Down => (),
                    Direction::Left => {
                        light.direction = Direction::Up;
                        lights.insert(light);
                        light.direction = Direction::Down;
                    }
                    Direction::Right => {
                        light.direction = Direction::Down;
                        lights.insert(light);
                        light.direction = Direction::Up;
                    }
                }
            } else if input[light.point.0][light.point.1] == '-' {
                match light.direction {
                    Direction::Up => {
                        light.direction = Direction::Left;
                        lights.insert(light);
                        light.direction = Direction::Right;
                    }
                    Direction::Down => {
                        light.direction = Direction::Right;
                        lights.insert(light);
                        light.direction = Direction::Left;
                    }
                    Direction::Left => (),
                    Direction::Right => (),
                }
            }

            match light.direction {
                Direction::Up => {
                    if light.point.0 == 0 {
                        break;
                    }
                    light.point.0 -= 1;
                }
                Direction::Down => light.point.0 += 1,
                Direction::Left => {
                    if light.point.1 == 0 {
                        break;
                    }
                    light.point.1 -= 1;
                }
                Direction::Right => light.point.1 += 1,
            }
        }
    }

    illuminated.len() as u64
}

fn part2(input: &Input) -> u64 {
    let mut max = 0;

    for k in 0..input.len() {
        max = max.max(part1(input, State { point: (k, 0), direction: Direction::Right }));
    }

    for k in 0..input[0].len() {
        max = max.max(part1(input, State { point: (0, k), direction: Direction::Down }));
    }

    for k in 0..input.len() {
        max = max.max(part1(input, State { point: (k, input[0].len() - 1), direction: Direction::Left }));
    }

    for k in 0..input[0].len() {
        max = max.max(part1(input, State { point: (input.len() - 1, k), direction: Direction::Up }));
    }

    max
}