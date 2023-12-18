use crate::{Solution, SolutionPair};
use regex::Regex;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct Dig {
    direction: Direction,
    long: usize,
    color: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    U,
    D,
    L,
    R,
}

type Input = Vec<Dig>;

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("input/input18.txt").expect("No input found for day 18!").trim_end().to_string();

    let input = parse(&input_string);
    // Your solution here...
    let sol1: u64 = get_area(&input);

    let input2: Vec<Dig> = input.iter()
        .map(|dig| {
            //parse the first 5 characters in hexadecimal to decimal in dig.color
            
            let long = i64::from_str_radix(&dig.color[0..5], 16).unwrap();
            let dir = match &dig.color[5..6] {
                "0" => Direction::R,
                "1" => Direction::D,
                "2" => Direction::L,
                "3" => Direction::U,
                _ => panic!("Invalid direction"),
            };

            Dig { direction: dir, long: long as usize, color: "".to_owned() }
        })
        .collect();

    let sol2: u64 = get_area(&input2);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse(input_string: &str) -> Input {
    let reg = Regex::new(r"^(?<direction>[UDLR]) (?<long>\d+) \(#(?<color>(?:[0-9]|[a-f]){6})\)$").unwrap();
    input_string.split('\n')
        .map(|line| {
            let caps = reg.captures(line).unwrap();
            let direction = match caps.name("direction").unwrap().as_str() {
                "U" => Direction::U,
                "D" => Direction::D,
                "L" => Direction::L,
                "R" => Direction::R,
                _ => panic!("Invalid direction"),
            };
            let long: usize = caps.name("long").unwrap().as_str().parse().unwrap();
            let color = caps.name("color").unwrap().as_str().to_string();
            
            Dig { direction, long, color }
        })
        .collect()
}

fn get_area(input: &Input) -> u64 {
    let mut points = Vec::new();
    let mut actual_point = (0, 0);
    points.push(actual_point);
    let mut b = 0;

    for dig in input {

        match dig.direction {
            Direction::U => {
                actual_point = (actual_point.0, actual_point.1 - dig.long as i64);
            },
            Direction::D => {
                actual_point = (actual_point.0, actual_point.1 + dig.long as i64);
            },
            Direction::L => {
                actual_point = (actual_point.0 - dig.long as i64, actual_point.1);
            },
            Direction::R => {
                actual_point = (actual_point.0 + dig.long as i64, actual_point.1);
            },
        }
        
        b += dig.long as u64;
        points.push(actual_point);
    }

    points.push((0, 0));

    // Calculate area using shoelace method

    let mut area: i64 = 0;

    for i in 0..points.len()-1 {
        area += points[i].0 * points[i+1].1 - points[i+1].0 * points[i].1;
    }

    // Using Pick's theorem we can calculate the number of interior squares
    // A = i + b/2 - 1 <===> i = A - b/2 + 1
    // Where A is the area, i is the number of interior squares and b is the number of boundary squares
    // Now, we just add b to get the total area: A + b/2 + 1

    area.unsigned_abs()/2 + b/2 + 1
}
