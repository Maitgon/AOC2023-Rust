use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;
use regex::Regex;
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

type Workflow = Vec<Filter>;

#[derive(Debug)]
enum Filter {
    Less(String, u64, String),
    Greater(String, u64, String),
    Always(String),
}

type Interval = (u64, u64);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct PartInterval {
    flow: String,
    x: Interval,
    m: Interval,
    a: Interval,
    s: Interval,
}

pub fn solve() -> SolutionPair {
    let input_string = read_to_string("input/input19.txt").expect("No input found for day 19!").trim_end().to_string();

    let (workflows, parts) = parse(&input_string);

    // Your solution here...
    let sol1: u64 = part1(&workflows, &parts);
    let sol2: u64 = part2(&workflows);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(workflows: &HashMap<String, Workflow>, parts: &Vec<Part>) -> u64 {
    let mut sol = 0;
    for part in parts {
        let mut work = "in";
        loop {
            for filter in workflows.get(work).unwrap().iter() {
                match filter {
                    Filter::Less(p, long, dest) => {
                        if p == "x" && part.x < *long || p == "m" && part.m < *long || p == "a" && part.a < *long || p == "s" && part.s < *long {
                        work = dest;
                        break;
                        }
                    },
                    Filter::Greater(p, long, dest) => {
                        if p == "x" && part.x > *long || p == "m" && part.m > *long || p == "a" && part.a > *long || p == "s" && part.s > *long {
                        work = dest;
                        break;
                        }
                    },
                    Filter::Always(dest) => {
                        work = dest;
                        break;
                    },
                }
            }
            match work {
                "A" => {
                    sol += part.x + part.m + part.a + part.s;
                    break;
                }
                "R" => break,
                _ => (),
            }
        }
        
    }
    sol
}

fn part2(workflows: &HashMap<String, Workflow>) -> u64 {
    let mut actual_intervals = HashSet::new();

    actual_intervals.insert(PartInterval {
        flow: "in".to_owned(),
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    });

    let mut sol = 0;

    while !actual_intervals.is_empty() {
        let mut new_intervals = HashSet::new();
        let mut new_intervals_pruned = HashSet::new();
        for interval in actual_intervals.iter() {
            let mut interval = interval.clone();
            for filter in workflows.get(&interval.flow).unwrap().iter() {
                match filter {
                    Filter::Less(p, long, dest) => {
                        if p == "x" {
                            if interval.x.1 < *long {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_intervals.insert(new_interval);
                                break;
                            } else if interval.x.0 < *long && *long <= interval.x.1 {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_interval.x.1 = *long - 1;
                                new_intervals.insert(new_interval);
                                interval.x.0 = *long;
                            }
                        } else if p == "m" {
                            if interval.m.1 < *long {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_intervals.insert(new_interval);
                                break;
                            } else if interval.m.0 < *long && *long <= interval.m.1 {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_interval.m.1 = *long - 1;
                                new_intervals.insert(new_interval);
                                interval.m.0 = *long;
                            }
                        } else if p == "a" {
                            if interval.a.1 < *long {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_intervals.insert(new_interval);
                                break;
                            } else if interval.a.0 < *long && *long <= interval.a.1 {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_interval.a.1 = *long - 1;
                                new_intervals.insert(new_interval);
                                interval.a.0 = *long;
                            }
                        } else if p == "s" {
                            if interval.s.1 < *long {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_intervals.insert(new_interval);
                                break;
                            } else if interval.s.0 < *long && *long <= interval.s.1 {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_interval.s.1 = *long - 1;
                                new_intervals.insert(new_interval);
                                interval.s.0 = *long;
                            }
                        }
                    },
                    Filter::Greater(p, long, dest) => {
                        if p == "x" {
                            if interval.x.0 > *long {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_intervals.insert(new_interval);
                                break;
                            } else if interval.x.1 > *long && *long >= interval.x.0 {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_interval.x.0 = *long + 1;
                                new_intervals.insert(new_interval);
                                interval.x.1 = *long;
                            }
                        } else if p == "m" {
                            if interval.m.0 > *long {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_intervals.insert(new_interval);
                                break;
                            } else if interval.m.1 > *long && *long >= interval.m.0 {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_interval.m.0 = *long + 1;
                                new_intervals.insert(new_interval);
                                interval.m.1 = *long;
                            }
                        } else if p == "a" {
                            if interval.a.0 > *long {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_intervals.insert(new_interval);
                                break;
                            } else if interval.a.1 > *long && *long >= interval.a.0 {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_interval.a.0 = *long + 1;
                                new_intervals.insert(new_interval);
                                interval.a.1 = *long;
                            }
                        } else if p == "s" {
                            if interval.s.0 > *long {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_intervals.insert(new_interval);
                                break;
                            } else if interval.s.1 > *long && *long >= interval.s.0 {
                                let mut new_interval = interval.clone();
                                new_interval.flow = dest.to_owned();
                                new_interval.s.0 = *long + 1;
                                new_intervals.insert(new_interval);
                                interval.s.1 = *long;
                            }
                        }
                    },
                    Filter::Always(dest) => {
                        let mut new_interval = interval.clone();
                        new_interval.flow = dest.to_owned();
                        new_intervals.insert(new_interval);
                    },
                }
            }
        }

        for interval in new_intervals {
            match interval.flow.as_str() {
                "A" => sol += (interval.x.1 - interval.x.0 + 1) * (interval.m.1 - interval.m.0 + 1) * (interval.a.1 - interval.a.0 + 1) * (interval.s.1 - interval.s.0 + 1),
                "R" => (),
                _ => _ = new_intervals_pruned.insert(interval),
            }
        }
        
        actual_intervals = new_intervals_pruned;
    }

    sol
}

fn parse(input_string: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let aux = input_string.split_once("\n\n").unwrap();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let reg = Regex::new(r"(?<part>\w)(?<sign><|>)(?<long>\d+):(?<dest>\w+)").unwrap();
    for workflow_string in aux.0.lines() {
        //println!("{}", workflow_string);
        let aux_inner = workflow_string[0..workflow_string.len()-1].split_once('{').unwrap();

        let name = aux_inner.0.to_owned();

        let aux_inner_2: Vec<&str> = aux_inner.1.split(',').collect();
        let dist = aux_inner_2[aux_inner_2.len() - 1].to_owned();
        let mut filters: Vec<Filter> = Vec::new();
        for filt in aux_inner_2[..aux_inner_2.len() - 1].iter() {
            let caps = reg.captures(filt).unwrap();
            let part = caps.name("part").unwrap().as_str();
            let sign = caps.name("sign").unwrap().as_str();
            let long = caps.name("long").unwrap().as_str().parse::<u64>().unwrap();
            let dest = caps.name("dest").unwrap().as_str();

            let filter = match sign {
                "<" => Filter::Less(part.to_owned(), long, dest.to_owned()),
                ">" => Filter::Greater(part.to_owned(), long, dest.to_owned()),
                _ => panic!("Invalid sign"),
            };
            
            filters.push(filter);
        }
        
        filters.push(Filter::Always(dist));

        workflows.insert(name, filters);
    }

    let reg2 = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();

    let mut parts: Vec<Part> = Vec::new();

    for part_string in aux.1.lines() {
        let caps = reg2.captures(part_string).unwrap();
        let x = caps.name("x").unwrap().as_str().parse::<u64>().unwrap();
        let m = caps.name("m").unwrap().as_str().parse::<u64>().unwrap();
        let a = caps.name("a").unwrap().as_str().parse::<u64>().unwrap();
        let s = caps.name("s").unwrap().as_str().parse::<u64>().unwrap();

        parts.push(Part { x, m, a, s });
    }


    (workflows, parts)
}

