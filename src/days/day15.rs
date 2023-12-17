use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::array;

///////////////////////////////////////////////////////////////////////////////

struct Lens {
    label: String,
    value: u64,
}

#[derive(Debug, Eq, PartialEq)]
enum Label {
    Dash(String),
    Equal(String, u64),
}

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("input/input15.txt").expect("No input found for day 15!").trim_end().to_string();
    // Your solution here...
    let sol1: u64 = input_string.split(',')
        .map(hash_algorithm)
        .sum();

    let input = parse(input_string);

    let sol2: u64 = part2(input); // Use function remove(ix) to remove the element at index ix and shift all elements after it to the left

    (Solution::from(sol1), Solution::from(sol2))
}

fn part2(input: Vec<Label>) -> u64 {
    let mut boxes: [Vec<Lens>; 256] = array::from_fn(|_| Vec::new());

    for label in input {
        match label {
            Label::Dash(val) => {
                let hash = hash_algorithm(&val) as usize;
                
                // check if boxes[hash] contains a lens with the same label

                if let Some(ix) = boxes[hash].iter().position(|lens| lens.label == val) {
                    boxes[hash].remove(ix);
                }
            },

            Label::Equal(label, value) => {
                let hash = hash_algorithm(&label) as usize;
                
                if let Some(ix) = boxes[hash].iter().position(|lens| lens.label == label) {
                    boxes[hash][ix].value = value;
                } else {
                    boxes[hash].push(Lens { label, value });
                }
            },
        }
    }

    // Reject functional modernity, embrace for loop tradition
    /*
    let mut sol = 0;
    for (i, boxi) in boxes.iter().enumerate() {
        let mut aux = 0;
        for (j, boxj) in boxi.iter().enumerate() {
            aux += (j + 1) as u64 * boxj.value;
        }
        sol += (i + 1) as u64 * aux;
    }
    */

    // Functional modernity AAAAAAAAAAAAAAAAAAAAAAAAAAAA
    let sol = boxes.iter()
        .map(|boxi| {
            boxi.iter()
                .enumerate()
                .fold(0, |acc, (j, boxj)| acc + (j + 1) as u64 * boxj.value)
        })
        .enumerate()
        .fold(0, |acc, (i, aux)| acc + (i + 1) as u64 * aux);

    sol
}

fn hash_algorithm(string: &str) -> u64 {
    let mut current_value = 0;

    for c in string.chars() {
        current_value += c as u64;
        current_value *= 17;
        current_value %= 256;
    }
    
    current_value
}

fn parse(input: String) -> Vec<Label> {
    input.split(',')
        .map(|s| {
            match s.split_once('=') {
                Some((label, value)) => Label::Equal(label.to_string(), value.parse::<u64>().unwrap()),
                None => Label::Dash(s.strip_suffix('-').unwrap().to_string()),
            }
        })
        .collect()
}
