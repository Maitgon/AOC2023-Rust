use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////

type Input = Vec<Vec<char>>;

pub fn solve() -> SolutionPair {
    let input_string: String = read_to_string("input/input11.txt").unwrap();

    let input: Input = input_string.lines().map(|line| line.chars().collect()).collect();
    // Your solution here...
    let (sol1, sol2) = part(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part(input: &Input) -> (u64, u64) {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut empty_rows: HashSet<usize> = HashSet::new();

    for (i, line) in input.iter().enumerate() {
        let mut is_empty = true;
        for (j, c) in line.iter().enumerate() {
            if *c == '#' {
                galaxies.push((i, j));
                is_empty = false;
            }
        }

        if is_empty {
            empty_rows.insert(i);
        }
    }

    let mut empty_columns: HashSet<usize> = HashSet::new();

    for j in 0..input[0].len() {
        let mut is_empty = true;
        for i in 0..input.len() {
            if input[i][j] == '#' {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            empty_columns.insert(j);
        }
    }

    let mut new_galaxies: HashSet<(usize, usize)> = HashSet::new();
    let mut new_galaxies2: HashSet<(usize, usize)> = HashSet::new();

    for (i, j) in galaxies {
        let mut acc_i = 0;
        for x in 0..i {
            if empty_rows.contains(&x) {
                acc_i += 1;
            }
        }

        let mut acc_j = 0;

        for y in 0..j {
            if empty_columns.contains(&y) {
                acc_j += 1;
            }
        }

        new_galaxies.insert((i + acc_i, j + acc_j));
        new_galaxies2.insert((i + 999999*acc_i, j + 999999*acc_j));
    }

    // Calculate the manhattan distance between every pair in new_galaxies
    let mut total_distance = 0;

    for (i, gal1) in new_galaxies.iter().enumerate() {
        for gal2 in new_galaxies.iter().skip(i + 1) {
            total_distance += manhattan_distance_2d(*gal1, *gal2);
        }
    }

    let mut total_distance2 = 0;

    for (i, gal1) in new_galaxies2.iter().enumerate() {
        for gal2 in new_galaxies2.iter().skip(i + 1) {
            total_distance2 += manhattan_distance_2d(*gal1, *gal2);
        }
    }



    (total_distance as u64, total_distance2 as u64)
}

fn manhattan_distance_2d(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()) as usize
}
