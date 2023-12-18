use crate::{Solution, SolutionPair};
use rayon::prelude::*;
use std::fs::read_to_string;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

type State = HashMap<Spring, u64>;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Spring {
    springs: Vec<char>,
    ways: Vec<u64>,
    last: char,
}

pub fn solve() -> SolutionPair {
    let input_string: String = read_to_string("input/input12.txt").unwrap();

    let input = parse(&input_string);

    // Your solution here...
    let sol1: u64 = part1(&input);
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse(input_string: &str) -> Vec<Spring> {
    let mut springs: Vec<Spring> = Vec::new();

    for line in input_string.lines() {
        let aux: Vec<&str> = line.split(' ').collect();

        let mut spring = Spring {
            springs: Vec::new(),
            ways: Vec::new(),
            last: ' ',
        };

        spring.springs = aux[0].chars().collect();

        for num in aux[1].split(',') {
            spring.ways.push(num.parse::<u64>().unwrap());
        }

        springs.push(spring);
    }

    springs
}

fn ways(spring: Vec<char>, ways: Vec<u64>) -> u64 {
    let mut states: State = HashMap::new();
    states.insert(Spring {
        springs: spring.clone(),
        ways: ways.clone(),
        last: ' ',
    }, 1);

    let mut total_ways = 0;

    // Queda poner que ocurre cuando llegan a longitud 0
    for _ in 0..spring.len()+1 {
        let mut new_states = HashMap::new();
        for (state, vals) in states {

            if state.springs.is_empty() && (state.ways.is_empty() || state.ways.len() == 1 && state.ways[0] == 0) {
                total_ways += vals;
                continue;
            } else if state.springs.is_empty() && !state.ways.is_empty() {
                continue;
            }

            // Check if the state is still viable
            if !is_viable(&state.springs.clone(), &state.ways.clone()) {
                continue;
            }

            // This one is so every new_state has the same length and its easier to get repeated ones
            let mut changing_states = Vec::new();

            match state.springs[0] {
                '.' => {
                    // Not valid states:
                    // 1. If the last char is # but they ask for more #
                    let mut new_state = Spring {
                        springs: state.springs,
                        ways: state.ways,
                        last: '.',
                    };
                    if state.last == '#' && new_state.ways[0] != 0 {
                        continue;
                    }

                    // Valid states:
                    // 1. If the last char is # and they dont want more 0
                    if state.last == '#' && new_state.ways[0] == 0 {
                        if new_state.ways.len() == 1 {
                            new_state.ways = Vec::new();
                            //println!("HERE");
                        } else {
                            new_state.ways = new_state.ways[1..].to_vec();
                        }
                    }
                    
                    // 2. If the last char is . do nothing

                    if new_state.springs.len() == 1 {
                        //println!("HERE");
                        new_state.springs = Vec::new();
                    } else {
                        new_state.springs = new_state.springs[1..].to_vec();
                    }

                    // Insert the new state and sum the values if it is already present
                    let count = new_states.entry(new_state).or_insert(0);
                    *count += vals;
                },
                '#' => {
                    // Not valid states:
                    // 1. If ways is empty

                    if state.ways.is_empty() {
                        continue;
                    }

                    // 2. If ways[0] is 0

                    if state.ways[0] == 0 {
                        continue;
                    }

                    // Valid states:
                    // 1. If ways[0] is more than 0

                    if state.ways[0] > 0 {
                        let mut new_state = Spring {
                            springs: state.springs,
                            ways: state.ways,
                            last: '#',
                        };

                        new_state.ways[0] -= 1;

                        if new_state.springs.len() == 1 {
                            new_state.springs = Vec::new();
                        } else {
                            new_state.springs = new_state.springs[1..].to_vec();
                        }

                        // Insert the new state and sum the values if it is already present
                        let count = new_states.entry(new_state).or_insert(0);
                        *count += vals;
                    }
                },
                '?' => {
                    // Adds to changing states the same states but substituting the first char for . and #
                    let mut new_spring_1 = state.springs.clone();
                    new_spring_1[0] = '.';
                    changing_states.push(Spring {
                        springs: new_spring_1,
                        ways: state.ways.clone(),
                        last: state.last,
                    });

                    let mut new_spring_2 = state.springs.clone();
                    new_spring_2[0] = '#';
                    changing_states.push(Spring {
                        springs: new_spring_2,
                        ways: state.ways.clone(),
                        last: state.last,
                    });
                },
                _ => panic!("Invalid char"),
            }

            // Add the changing states to the new states
            for changing_state in changing_states {
                match changing_state.springs[0] {
                    '.' => {
                        // Not valid states:
                        // 1. If the last char is # but they ask for more #
                        let mut new_state = Spring {
                            springs: changing_state.springs,
                            ways: changing_state.ways,
                            last: '.',
                        };
                        if changing_state.last == '#' && new_state.ways[0] != 0 {
                            continue;
                        }

                        // Valid states:
                        // 1. If the last char is # and they dont want more 0
                        if changing_state.last == '#' && new_state.ways[0] == 0 {
                            if new_state.ways.len() == 1 {
                                new_state.ways = Vec::new();
                            } else {
                                new_state.ways = new_state.ways[1..].to_vec();
                            }
                        }
                        
                        // 2. If the last char is . do nothing

                        if new_state.springs.len() == 1 {
                            new_state.springs = Vec::new();
                        } else {
                            new_state.springs = new_state.springs[1..].to_vec();
                        }

                        // Insert the new state and sum the values if it is already present
                        let count = new_states.entry(new_state).or_insert(0);
                        *count += vals;
                    },
                    '#' => {
                        // Not valid states:
                        // 1. If ways is empty

                        if changing_state.ways.is_empty() {
                            continue;
                        }

                        // 2. If ways[0] is 0

                        if changing_state.ways[0] == 0 {
                            continue;
                        }

                        // Valid states:
                        // 1. If ways[0] is more than 0

                        if changing_state.ways[0] > 0 {
                            let mut new_state = Spring {
                                springs: changing_state.springs,
                                ways: changing_state.ways,
                                last: '#',
                            };

                            new_state.ways[0] -= 1;

                            if new_state.springs.len() == 1 {
                                new_state.springs = Vec::new();
                            } else {
                                new_state.springs = new_state.springs[1..].to_vec();
                            }

                            // Insert the new state and sum the values if it is already present
                            let count = new_states.entry(new_state).or_insert(0);
                            *count += vals;
                        }
                    },
                    _ => panic!("Invalid char"),
                }
            }
        }
        states = new_states;
    }

    total_ways
}

fn part1(springs: &Vec<Spring>) -> u64 {
    springs.par_iter().map(|spring| ways(spring.springs.clone(), spring.ways.clone())).sum()
}

fn part2(springs: &Vec<Spring>) -> u64 {
    springs.par_iter().map(|spring| ways(get_new_spring(&spring.springs), spring.ways.clone().repeat(5))).sum()
}

fn get_new_spring(spring: &Vec<char>) -> Vec<char> {
    let mut new_spring: Vec<char> = spring.clone();
        // repeat the string 5 times

    for _ in 0..4 {
        new_spring.push('?');

        for c in spring {
            new_spring.push(*c);
        }
    }
    
    new_spring
}

fn is_viable(spring: &[char], ways: &Vec<u64>) -> bool {
    // We don't filter base for '.'
    if ways.is_empty() || ways[0] == 0 {
        return true;
    }
    // First, we see if there are enough # and ? to satisfy the ways
    let enough = ways.iter().sum::<u64>() <= spring.iter().filter(|c| **c == '#' || **c == '?').count() as u64;

    enough
}

/*
fn part1(springs: &Vec<Spring>) -> u64 {
    let mut acc = 0;

    for spring in springs {
        let all_variations = get_all_variations(spring.springs.clone());

        for variation in all_variations {
            if is_valid_variation(variation.clone(), spring.ways.clone()) {
                acc += 1;
                println!("{:?}", variation);
            }
        }
    }

    acc
}

fn get_all_variations(spring: Vec<char>) -> Vec<Vec<char>> {
    let mut all_variations: Vec<Vec<char>> = vec![spring.clone()];

    //substitute every ? with either . or # and get all the variations
    for i in 0..spring.len() {
        let mut new_variations = Vec::new();
        if spring[i] != '?' {
            continue;
        }

        for variation in all_variations {
            let mut new_variation = variation.clone();
            new_variation[i] = '.';
            new_variations.push(new_variation);

            let mut new_variation = variation.clone();
            new_variation[i] = '#';
            new_variations.push(new_variation);
        }

        all_variations = new_variations;

    }

    all_variations
}

fn is_valid_variation(spring: Vec<char>, ways: Vec<u64>) -> bool {
    let mut acc = 0;
    let mut actual_way = 0;
    let mut is_valid = true;

    // ..#...#...### 1 1 3
    for c in spring {
        if c == '.' && acc > 0 {
            if acc == ways[actual_way] {
                actual_way += 1;
                acc = 0;
            } else {
                is_valid = false;
                break;
            }
        } else if c == '#' {
            acc += 1;
            if actual_way == ways.len() {
                is_valid = false;
                break;
            }
        }
        //println!("{} {} {}", c, acc, actual_way);
    }

    if actual_way == ways.len()-1 && acc != ways[actual_way] || actual_way < ways.len()-1 {
        //println!("HEY");
        is_valid = false;
    }

    is_valid
}

*/