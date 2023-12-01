use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("input/input01.txt").unwrap();

    let input = parse(&input_string);

    let numbers: Vec<Vec<u32>> = input.iter()
        .map(|line| get_numbers(line.to_string()))
        .collect();

    let numbers2: Vec<Vec<u32>> = input.iter()
        .map(|line| get_numbers2(line.to_string()))
        .collect();
                            


    // Your solution here...
    let sol1: u32 = get_sum(numbers);
    let sol2: u32 = get_sum(numbers2);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse(input: &str) -> Vec<String> {
    input.lines()
         .map(|line| line.to_string())
         .collect()
}

fn get_numbers(input: String) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    for c in input.chars() {
        if c.is_ascii_digit() {
            numbers.push(c.to_digit(10).unwrap());
        }
    }

    numbers
}

fn get_numbers2(input: String) -> Vec<u32> {
    let chars: Vec<char> = input.chars().collect();

    let mut numbers: Vec<u32> = Vec::new();

    let mut i = 0;

    while i < chars.len() {
        // If its a digit
        if chars[i].is_ascii_digit() {
            numbers.push(chars[i].to_digit(10).unwrap());
        }

        // If its one
        else if i + 2 < chars.len() && chars[i..i+3] == ['o', 'n', 'e'] {
            numbers.push(1);
        }

        // If its two
        else if i + 2 < chars.len() && chars[i..i+3] == ['t', 'w', 'o'] {
            numbers.push(2);
        }

        // If its three
        else if i + 4 < chars.len() && chars[i..i+5] == ['t', 'h', 'r', 'e', 'e'] {
            numbers.push(3);
        }

        // If its four
        else if i + 3 < chars.len() && chars[i..i+4] == ['f', 'o', 'u', 'r'] {
            numbers.push(4);
        }

        // If its five
        else if i + 3 < chars.len() && chars[i..i+4] == ['f', 'i', 'v', 'e'] {
            numbers.push(5);
        }

        // If its six
        else if i + 2 < chars.len() && chars[i..i+3] == ['s', 'i', 'x'] {
            numbers.push(6);
        }

        // If its seven
        else if i + 4 < chars.len() && chars[i..i+5] == ['s', 'e', 'v', 'e', 'n'] {
            numbers.push(7);
        }

        // If its eight
        else if i + 4 < chars.len() && chars[i..i+5] == ['e', 'i', 'g', 'h', 't'] {
            numbers.push(8);
        }

        // If its nine
        else if i + 3 < chars.len() && chars[i..i+4] == ['n', 'i', 'n', 'e'] {
            numbers.push(9);
        }

        i += 1;
    }

    numbers
}

fn get_sum(numbers: Vec<Vec<u32>>) -> u32 {
    numbers.iter()
        .map(|num| num[0]*10 + num[num.len()-1])
        .sum()
}