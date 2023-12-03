use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

type Input = Vec<Vec<char>>;

type Position = (usize, usize);

#[derive(Debug)]
struct Number {
    value: u64,
    position: Vec<Position>,
}

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("input/input03.txt").unwrap();

    let input = parse(input_string);

    // Your solution here...
    let (sol1, sol2) = part(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse(input_string: String) -> Input {
    input_string.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn part(input: &Input) -> (u64, u64) {
    let numbers = get_numbers(input);
    let mut gear: HashMap<Position, Vec<u64>> = HashMap::new();
    
    let mut sol1 = 0;

    for num in numbers {
        let around: HashSet<Position> = get_around(&num.position, 0, input.len(), 0, input[0].len());

        let mut valid = false;

        for pos in around {
            if input[pos.0][pos.1] != '.' && !input[pos.0][pos.1].is_ascii_digit() {
                valid = true;
                
                if input[pos.0][pos.1] == '*' {
                    gear.entry(pos).or_default().push(num.value);
                    break
                }
            }
        }

        if valid {
            sol1 += num.value;
        }
    }

    let sol2: u64 = gear.iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum();

    (sol1, sol2)
}

fn get_numbers(input: &Input) -> Vec<Number> {
    let mut numbers = Vec::new();

    let mut pos_vec: Vec<Position> = Vec::new();
    let mut num: u64 = 0;

    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                let val = c.to_digit(10).unwrap() as u64;
                let pos = (i, j);

                num = num * 10 + val;

                pos_vec.push(pos);
                
                if j+1 == line.len() || j+1 < line.len() && !input[i][j + 1].is_ascii_digit() {
                    numbers.push(Number {
                        value: num,
                        position: pos_vec,
                    });
                    num = 0;
                    pos_vec = Vec::new();
                } 
            }
        }
        num = 0;
        pos_vec = Vec::new();
    }

    numbers
}

fn get_around(pos: &Vec<Position>, min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> HashSet<Position> {
    let mut around: HashSet<Position> = HashSet::new();

    for p in pos {
        let i = p.0 as i32;
        let j = p.1 as i32;

        for x in i-1..=i+1 {
            for y in j-1..=j+1 {
                if x < max_x as i32 && y < max_y as i32 && x >= min_x as i32 && y >= min_y as i32 && !pos.contains(&(x as usize, y as usize)) {
                    around.insert((x as usize, y as usize));
                }
            }
        }
    }

    around
}
