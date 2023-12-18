use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use pathfinding::directed::astar::astar;

///////////////////////////////////////////////////////////////////////////////

type Input = Vec<Vec<u64>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    point: (usize, usize),
    direction: Direction,
    forwards: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("input/input17.txt").expect("No input found for day 17!").trim_end().to_string();

    let input: Input = input_string.split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();

    // Your solution here...
    let sol1: u64 = part1(&input);
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &Input) -> u64 {
    astar(
        &State { point: (0, 0), direction: Direction::Right, forwards: 1 },
        |state| {
            let mut successors = Vec::new();

            if state.forwards < 3 {
                // Go forwards
                let next_point = next_point(state.point, state.direction, input);
                if let Some(next_point) = next_point {
                    successors.push((State { 
                        point: next_point,
                        direction: state.direction,
                        forwards: state.forwards + 1
                    }, input[next_point.0][next_point.1]));
                }
            }

            let (left_dir, right_dir) = match state.direction {
                Direction::Up => (Direction::Left, Direction::Right),
                Direction::Down => (Direction::Right, Direction::Left),
                Direction::Left => (Direction::Down, Direction::Up),
                Direction::Right => (Direction::Up, Direction::Down),
            };

            // Turn Left
            let next_left_point = next_point(state.point, left_dir, input);
            if let Some(next_left_point) = next_left_point {
                successors.push((State { 
                    point: next_left_point,
                    direction: left_dir,
                    forwards: 1
                }, input[next_left_point.0][next_left_point.1]));
            }

            // Turn Right
            let next_right_point = next_point(state.point, right_dir, input);
            if let Some(next_right_point) = next_right_point {
                successors.push((State { 
                    point: next_right_point,
                    direction: right_dir,
                    forwards: 1
                }, input[next_right_point.0][next_right_point.1]));
            }

            successors
        },
        |state| {
            // The manhattan distance to the goal is the heuristic
            (input.len()-1 - state.point.0 + input[0].len()-1 - state.point.1) as u64
        },
        |state| {
            state.point.0 == input.len() - 1 && state.point.1 == input[0].len() - 1
        }).unwrap().1
}

fn part2(input: &Input) -> u64 {
    astar(
        &State { point: (0, 0), direction: Direction::Right, forwards: 1 },
        |state| {
            let mut successors = Vec::new();

            if state.forwards < 10 {
                // Go forwards
                let next_point = next_point(state.point, state.direction, input);
                if let Some(next_point) = next_point {
                    successors.push((State { 
                        point: next_point,
                        direction: state.direction,
                        forwards: state.forwards + 1
                    }, input[next_point.0][next_point.1]));
                }
            }

            let (left_dir, right_dir) = match state.direction {
                Direction::Up => (Direction::Left, Direction::Right),
                Direction::Down => (Direction::Right, Direction::Left),
                Direction::Left => (Direction::Down, Direction::Up),
                Direction::Right => (Direction::Up, Direction::Down),
            };

            // Turn Left
            if state.forwards >= 4 {
                let next_left_point = next_point(state.point, left_dir, input);
                if let Some(next_left_point) = next_left_point {
                    successors.push((State { 
                        point: next_left_point,
                        direction: left_dir,
                        forwards: 1
                    }, input[next_left_point.0][next_left_point.1]));
                }
            }

            // Turn Right
            if state.forwards >= 4 {
                let next_right_point = next_point(state.point, right_dir, input);
                if let Some(next_right_point) = next_right_point {
                    successors.push((State { 
                        point: next_right_point,
                        direction: right_dir,
                        forwards: 1
                    }, input[next_right_point.0][next_right_point.1]));
                }
            }

            successors
        },
        |state| {
            // The manhattan distance to the goal is the heuristic
            (input.len()-1 - state.point.0 + input[0].len()-1 - state.point.1) as u64
        },
        |state| {
            state.point.0 == input.len() - 1 && state.point.1 == input[0].len() - 1 && state.forwards >= 4
        }).unwrap().1
}

fn next_point(point: (usize, usize), direction: Direction, input: &Input) -> Option<(usize, usize)> {
    let point = (point.0 as isize, point.1 as isize);
    let point = match direction {
        Direction::Up => (point.0 - 1, point.1),
        Direction::Down => (point.0 + 1, point.1),
        Direction::Left => (point.0, point.1 - 1),
        Direction::Right => (point.0, point.1 + 1),
    };

    if point.0 < 0 || point.1 < 0 || point.0 >= input.len() as isize || point.1 >= input[0].len() as isize {
        None
    } else {
        Some((point.0 as usize, point.1 as usize))
    }

    
}
