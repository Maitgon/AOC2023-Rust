use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

type Input = Vec<Vec<Vec<char>>>;

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("input/input13.txt").expect("No input found for day 13!");

    let input: Input = input_string.split("\n\n").
        map(|m| m.lines()
            .map(|l| l.chars().collect())
            .collect())
        .collect();

    // Your solution here...
    let sol1: u64 = input.iter().map(get_reflections).sum();
    let sol2: u64 = input.iter().map(get_reflections_2).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn get_reflections(m : &Vec<Vec<char>>) -> u64 {
    let mut vertical_refl = 0;

    for r in 0..m.len()-1 {
        let mut rev = true;
        for j in 0..m[r].len() {
            let mut k: usize = 0;
            while r + k + 1 < m.len() && r >= k {
                if m[r-k][j] != m[r+k+1][j] {
                    rev = false;
                    break;
                }
                k += 1;
            }

            if !rev {
                break;
            }
        }

        if rev {
            vertical_refl = r+1;
            break;
        }
    }

    vertical_refl *= 100;

    let mut horizontal_refl = 0;

    for c in 0..m[0].len()-1 {
        let mut rev = true;
        for i in 0..m.len() {
            let mut k: usize = 0;
            while c + k + 1 < m[i].len() && c >= k {
                if m[i][c-k] != m[i][c+k+1] {
                    rev = false;
                    break;
                }
                k += 1;
            }

            if !rev {
                break;
            }
        }

        if rev {
            horizontal_refl = c+1;
            break;
        }
    }

    (vertical_refl + horizontal_refl) as u64
}

fn get_reflections_2(m : &Vec<Vec<char>>) -> u64 {
    let mut vertical_refl = 0;

    for r in 0..m.len()-1 {
        let mut rev = 0;
        for j in 0..m[r].len() {
            let mut k: usize = 0;
            while r + k + 1 < m.len() && r >= k {
                if m[r-k][j] != m[r+k+1][j] {
                    rev += 1;
                    if rev > 1 {
                        break;
                    }
                }
                k += 1;
            }

            if rev > 1 {
                break;
            }
        }

        if rev == 1 {
            vertical_refl = r+1;
            break;
        }
    }

    vertical_refl *= 100;

    let mut horizontal_refl = 0;

    for c in 0..m[0].len()-1 {
        let mut rev = 0;
        for i in 0..m.len() {
            let mut k: usize = 0;
            while c + k + 1 < m[i].len() && c >= k {
                if m[i][c-k] != m[i][c+k+1] {
                    rev += 1;
                    if rev > 1 {
                        break;
                    }
                }
                k += 1;
            }

            if rev > 1 {
                break;
            }
        }

        if rev == 1 {
            horizontal_refl = c+1;
            break;
        }
    }

    (vertical_refl + horizontal_refl) as u64
}