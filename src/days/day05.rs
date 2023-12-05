use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

type Seeds = Vec<u64>;
type SeedsRange = Vec<(u64, u64)>;

#[derive(Debug)]
struct Conversion {
    destination: u64,
    source: u64,
    range: u64,
}

pub fn solve() -> SolutionPair {
    let input_string: String = read_to_string("input/input05.txt").unwrap();

    let (mut seeds, conversions_all) = parse(input_string);

    println!("{:?}", seeds);
    println!("{:?}", conversions_all);

    // Your solution here...
    let sol2: u64 = part2(&seeds, &conversions_all);
    let sol1: u64 = part1(&mut seeds, &conversions_all);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse(input_string: String) -> (Seeds, Vec<Vec<Conversion>>) {
    let aux = input_string.split("\n\n").collect::<Vec<&str>>();

    let seeds = aux[0].split(' ').map(|num| if num == "seeds:" { 0 } else { num.parse().unwrap() }).collect::<Seeds>()[1..].to_vec();

    let mut conversions_all: Vec<Vec<Conversion>> = Vec::new();

    for clust in &aux[1..] {
        let conversion_aux = clust.lines().collect::<Vec<&str>>()[1..].to_vec();

        let mut conversions: Vec<Conversion> = Vec::new();

        for conv in conversion_aux {
            let aux1: Vec<u64> = conv.split(' ').map(|num| num.parse().unwrap()).collect();
            conversions.push(Conversion {
                destination: aux1[0],
                source: aux1[1],
                range: aux1[2],
            });
        }

        conversions_all.push(conversions);
    }

    (seeds, conversions_all)
}

fn part1(seeds: &mut Seeds, conversions_all: &Vec<Vec<Conversion>>) -> u64 {
    for seed in &mut *seeds {
        for conversions in conversions_all {
            for conversion in conversions {
                if conversion.source <= *seed && *seed < conversion.source + conversion.range {
                    *seed = conversion.destination + (*seed - conversion.source);
                    break;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

fn part2(seeds: &Seeds, conversions_all: &Vec<Vec<Conversion>>) -> u64 {
    let mut seeds_range: SeedsRange = Vec::new();

    let mut i = 0;
    while i < seeds.len() {
        seeds_range.push((seeds[i], seeds[i] + seeds[i+1] - 1));
        i += 2;
    }

    let mut minimal: Vec<u64> = Vec::new();

    for seed_range in seeds_range {
        let mut conversion_out = vec![seed_range];
        for conversions in conversions_all {
            let mut new_conversion_out: Vec<(u64, u64)> = Vec::new();
            for seed_interval in conversion_out.clone() {        
                let mut still_converting: Vec<(u64, u64)> = vec![seed_interval];
                for conversion in conversions {
                    let mut new_conversion_list: Vec<(u64, u64)> = Vec::new();
                    for aux in still_converting {
                        
                        // This one if the conversion is inside the range
                        if conversion.source <= aux.0 && aux.1 < conversion.source + conversion.range {
                            new_conversion_out.push((conversion.destination + aux.0 - conversion.source, conversion.destination + aux.1 - conversion.source));
                        
                        // This one if the conversion is all outside the right or left
                        } else if aux.0 >= conversion.source + conversion.range || aux.1 < conversion.source {
                            new_conversion_list.push(aux);
                        
                        // This one if the conversion is outside the range in the right
                        } else if conversion.source <= aux.0 && conversion.source + conversion.range <= aux.1 {
                            new_conversion_out.push((conversion.destination + aux.0 - conversion.source, conversion.destination + conversion.range - 1));
                            new_conversion_list.push((conversion.source + conversion.range, aux.1));

                        // This one if the conversion is outside the range in the left
                        } else if aux.0 < conversion.source && aux.1 < conversion.source + conversion.range {
                            new_conversion_list.push((aux.0, conversion.source - 1));
                            new_conversion_out.push((conversion.destination, conversion.destination + aux.1 - conversion.source));

                        // This one if the conversion is outside the range in the left and right
                        } else if aux.0 < conversion.source && conversion.source + conversion.range <= aux.1 {
                            new_conversion_list.push((aux.0, conversion.source-1));
                            new_conversion_out.push((conversion.destination, conversion.destination + conversion.range - 1));
                            new_conversion_list.push((conversion.source + conversion.range, aux.1));

                        } else {
                            panic!("This should not happen")
                        }
                    }
                    
                    still_converting = new_conversion_list;
                }
                conversion_out = still_converting;
            }
            conversion_out.append(&mut new_conversion_out);
        }
        minimal.push(conversion_out.iter().map(|x| x.0).min().unwrap());
    }

    *minimal.iter().min().unwrap()
}
