use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

type Input = Vec<Vec<i64>>;

pub fn solve() -> SolutionPair {
    let input_string: String = read_to_string("input/input09.txt").unwrap();

    let input = parse(&input_string);
    // Your solution here...
    let sol1: i64 = input.iter().map(|v| part1(v.clone())).sum();
    let sol2: i64 = input.iter().map(|v| part2(v.clone())).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse(input_string: &str) -> Input {
    let mut input: Input = Vec::new();

    for line in input_string.lines() {
        let mut aux: Vec<i64> = Vec::new();
        for num in line.split(' ') {
            aux.push(num.parse::<i64>().unwrap());
        }
        input.push(aux);
    }

    input
}

fn part1(mut input: Vec<i64>) -> i64 {
    let mut zeroes = false;
    let mut j = input.len() - 1;
    while !zeroes {
        zeroes = true;
        for i in 0..j {
            if input[i] != 0 {
                zeroes = false;
            }

            input[i] = input[i+1] - input[i];
        }
        j -= 1;
    }

    let mut sum = 0;

    for val in input.iter().skip(j) {
        sum += val;
    }

    sum
}

fn part2(mut input: Vec<i64>) -> i64 {
    let mut zeroes = false;
    let mut j = 1;

    while !zeroes {
        zeroes = true;
        for i in (j..input.len()).rev() {
            if input[i] != 0 {
                zeroes = false;
            }

            input[i] -= input[i-1];
        }
        j += 1;
    }

    let mut sum = 0;

    for val in (input.iter().take(j)).rev() {
        sum = val - sum;
    }

    sum
}
