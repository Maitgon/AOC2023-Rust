use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use regex::Regex;
use std::collections::HashMap;
use num::integer::lcm;

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq)]
struct Input {
    dir: Vec<char>,
    graph: Graph,
}

type Graph = HashMap<String, (String, String)>;

pub fn solve() -> SolutionPair {
    let input_string: String = read_to_string("input/input08.txt").unwrap();

    let input = parse(&input_string);

    // Your solution here...
    let sol1: u64 = part1(&input);
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse(input_string: &str) -> Input {
    let aux: Vec<&str> = input_string.split("\n\n").collect();

    let dir: Vec<char> = aux[0].chars().collect();

    let mut graph: Graph = HashMap::new();

    let re = Regex::new(r"\w{3}").unwrap();

    for line in aux[1].lines() {
        let place: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        
        graph.insert(place[0].to_string(), (place[1].to_string(), place[2].to_string()));
    }

    Input {dir, graph}
}

fn part1(input: &Input) -> u64 {
    let mut found = false;
    let mut steps = 0;
    let mut actual_node = "AAA";
    while !found {
        for step in input.dir.iter() {
            match step {
                'L' => actual_node = input.graph[actual_node].0.as_str(),
                'R' => actual_node = input.graph[actual_node].1.as_str(),
                _ => panic!("Invalid direction"),
            }
            steps += 1;
            
            if actual_node == "ZZZ" {
                found = true;
                break;
            }
        }
    }

    steps
}

fn part1_aux(input: &Input, node0: &str) -> u64 {
    let mut found = false;
    let mut steps = 0;
    let mut actual_node = node0;
    while !found {
        for step in input.dir.iter() {
            match step {
                'L' => actual_node = input.graph[actual_node].0.as_str(),
                'R' => actual_node = input.graph[actual_node].1.as_str(),
                _ => panic!("Invalid direction"),
            }
            steps += 1;
            
            if actual_node.ends_with('Z') {
                found = true;
                break;
            }
        }
    }

    steps
}

fn part2(input: &Input) -> u64 {
    let mut actual_nodes: Vec<String> = Vec::new();

    for node in input.graph.keys() {
        if node.ends_with('A') {
            actual_nodes.push(node.to_string());
        }
    }

    actual_nodes.iter()
        .map(|node| part1_aux(input, node.as_str()))
        .reduce(lcm)
        .unwrap()
}