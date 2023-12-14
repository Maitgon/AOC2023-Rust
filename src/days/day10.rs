use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////

type Index = (usize, usize);

type Input = Vec<Vec<char>>;

pub fn solve() -> SolutionPair {
    let input_string: String = read_to_string("input/input10.txt").unwrap();

    let input: Input = input_string.lines().map(|line| line.chars().collect()).collect();
    // Your solution here...
    let (sol1, sol2) = part(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part(input: &Input) -> (u64, i64) {
    // Find the starting index
    let mut start_index: Index = (0, 0);
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                start_index = (i, j);
            }
        }
    }

    // Now we go through the loop and check how long it is
    let mut cur = start_index;

    if input[cur.0-1][cur.1] == '7' || input[cur.0-1][cur.1] == 'F' || input[cur.0-1][cur.1] == '|' {
        cur = (cur.0-1, cur.1);
    } else if input[cur.0][cur.1-1] == 'F' || input[cur.0][cur.1-1] == 'L' || input[cur.0][cur.1-1] == '-' {
        cur = (cur.0, cur.1-1);
    } else if input[cur.0+1][cur.1] == '|' || input[cur.0+1][cur.1] == 'J' || input[cur.0+1][cur.1] == 'L' {
        cur = (cur.0+1, cur.1);
    } else if input[cur.0][cur.1+1] == '7' || input[cur.0][cur.1+1] == '-' || input[cur.0][cur.1+1] == 'J' {
        cur = (cur.0, cur.1+1);
    }

    let mut pre = start_index;

    let mut points = HashSet::new();
    points.insert(start_index);

    let mut length = 0;

    while cur != start_index {
        points.insert(cur);

        // If we moved up
        if pre.0 == cur.0 + 1 && pre.1 == cur.1 {
            pre = cur;
            match input[cur.0][cur.1] {
                '7' => cur = (cur.0, cur.1-1),
                'F' => cur = (cur.0, cur.1+1),
                '|' => cur = (cur.0-1, cur.1),
                c => panic!("Invalid character {} at ({}, {})", c, cur.0, cur.1),
            }
        }

        // If we moved left
        else if pre.0 == cur.0 && pre.1 == cur.1 + 1 {
            pre = cur;
            match input[cur.0][cur.1] {
                'F' => cur = (cur.0+1, cur.1),
                'L' => cur = (cur.0-1, cur.1),
                '-' => cur = (cur.0, cur.1-1),
                c => panic!("Invalid character {} at ({}, {})", c, cur.0, cur.1),
            }
        }

        // If we moved down
        else if pre.0 == cur.0 - 1 && pre.1 == cur.1 {
            pre = cur;
            match input[cur.0][cur.1] {
                'J' => cur = (cur.0, cur.1-1),
                'L' => cur = (cur.0, cur.1+1),
                '|' => cur = (cur.0+1, cur.1),
                c => panic!("Invalid character {} at ({}, {})", c, cur.0, cur.1),
            }
        }

        // If we moved right
        else if pre.0 == cur.0 && pre.1 == cur.1 - 1 {
            pre = cur;
            match input[cur.0][cur.1] {
                '7' => cur = (cur.0+1, cur.1),
                '-' => cur = (cur.0, cur.1+1),
                'J' => cur = (cur.0-1, cur.1),
                c => panic!("Invalid character {} at ({}, {})", c, cur.0, cur.1),
            }
        }

        length += 1;
    }

    let mut area = 0;

    for i in 0..input.len() {
        let mut inversions = 0;
        for j in 1..input[i].len() {
            
            if points.contains(&(i, j-1)) && (input[i][j-1] == 'J' || input[i][j-1] == 'L' || input[i][j-1] == '|') {
                inversions += 1;
            }

            if inversions % 2 == 1 && !points.contains(&(i, j)) {
                area += 1;
            }
        }
    }

    // Now we calculate the ares inside the points using the shoelace method

    (length / 2 + 1, area)
}
