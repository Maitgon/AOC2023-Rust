use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

type Input = Vec<Vec<char>>;

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("input/input14.txt").expect("No input found for day 14!");

    let input: Input = input_string.lines().map(|line| line.chars().collect()).collect();
    // Your solution here...
    let (sol1, sol2) = part1(input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(mut input: Input) -> (u64, u64) {

    let mut loads: HashMap<Input, u64> = HashMap::new();
    loads.insert(input.clone(), 0);
    
    move_up(&mut input);

    let count_load_1 = count_load(&input);

    // Complete the cyle
    rotate_right(&mut input);
    for _ in 0..3 {
        move_up(&mut input);
        rotate_right(&mut input);
    }

    let mut sol2 = 0;

    for cycles in 1.. {
        //Check if it is already in the map, if it is break of the loop, if it isnt add it to the map
        if loads.contains_key(&input) {
            let starting_cycle = loads.get(&input).unwrap();
            let period = cycles - starting_cycle;
            let index = ((1_000_000_000 - starting_cycle) % period) + starting_cycle;
            sol2 = count_load(loads.iter().find(|(_, &v)| v == index).unwrap().0);
            break;
        } else {
            loads.insert(input.clone(), cycles);
        }
        for _ in 0..4 {
            move_up(&mut input);
            rotate_right(&mut input);
        }
    }
    
    (count_load_1, sol2)
}

fn count_load(input: &Input) -> u64 {
    let mut count = 0;

    for (i, row) in input.iter().enumerate() {
        for elem in row {
            if *elem == 'O' {
                count += input.len() - i
            }
        }
    }

    count as u64
}

fn move_up(input: &mut Input) {
    for j in 0..input[0].len() {
        for i in 0..input.len() {
            if input[i][j] != 'O' {
                continue
            }

            let mut k = i;
            input[k][j] = '.';
            while k > 0 && input[k-1][j] == '.' {
                k -= 1;
            }

            input[k][j] = 'O';
        }
    }
}

//function to rotate 90 degrees to the right
fn rotate_right(input: &mut Input) {
    let mut new_input = vec![vec!['.'; input.len()]; input[0].len()];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            new_input[j][input.len()-1-i] = input[i][j];
        }
    }

    *input = new_input;
}
