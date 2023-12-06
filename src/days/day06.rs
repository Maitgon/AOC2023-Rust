use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_string: String = read_to_string("input/input06.txt").unwrap();

    let (time, distances) = parse(&input_string);
    // Your solution here...
    let sol1: u64 = part1(&time, &distances);
    let (t, d) = parse2(input_string);
    let sol2: u64 = get_ways(t, d);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse(input_string: &str) -> (Vec<u64>, Vec<u64>) {
    let (first, second) = input_string.split_once('\n').unwrap();

    let time = first[5..].split_ascii_whitespace().map(|num| num.parse().unwrap()).collect();
    let distances = second[9..].split_ascii_whitespace().map(|num| num.parse().unwrap()).collect();

    (time, distances)
}

fn part1(time: &[u64], distances: &[u64]) -> u64 {
    let mut sol = 1;
    for (t, d) in time.iter().zip(distances.iter()) {
        let ways = get_ways(*t, *d);

        sol *= ways;
    }

    sol
}

fn parse2(input_string: String) -> (u64, u64) {
    let (first, second) = input_string.split_once('\n').unwrap();

    // Now we get all the numbers eliminating whitespace and letters from the string
    let time: u64 = first.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();
    let distance: u64 = second.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();

    (time, distance)
}

fn get_ways(t: u64, d: u64) -> u64 {
    let i = (t as f64 - ((t*t - 4*d) as f64).sqrt()) / 2.0;

    t-1 - 2*i.floor() as u64
}