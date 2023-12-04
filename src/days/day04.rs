use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

struct Card {
    win: Vec<u64>,
    yours: Vec<u64>,
}

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("input/input04.txt").unwrap();

    let input = input_string.lines()
        .map(|line| parse_card(line.to_string()))
        .collect::<Vec<Card>>();

    // Your solution here...
    let (sol1, sol2) = solve_both(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_card(input_string: String) -> Card {
    let aux_id = input_string.split(": ").collect::<Vec<&str>>();

    let aux2 = aux_id[1].split(" | ").collect::<Vec<&str>>();
    let win = aux2[0].split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<u64>>();
    let yours = aux2[1].split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<u64>>();

    Card {
        win,
        yours,
    }
}

fn solve_both(input: &Vec<Card>) -> (u64, u64) {
    let mut sol1 = 0;
    let mut copies: Vec<u64> = vec![1; input.len()];

    for i in 0..input.len() {
        let mut matches = 0;
        for num in &input[i].yours {
            if input[i].win.contains(num) {
                matches += 1;
            }
        }

        // This is for the part 1 solution
        if matches == 0 {
            continue;
        } else {
            sol1 += 1 << (matches - 1);
        }

        // This is for the part 2 solution
        for j in i+1..=i+matches {
            copies[j] += copies[i];
        }
    }

    (sol1, copies.iter().sum())
}
