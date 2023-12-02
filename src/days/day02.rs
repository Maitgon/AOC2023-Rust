use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

struct Game {
    id: u64,
    game: Vec<Turn>,
}

type Turn = Vec<Ball>;

enum Ball {
    Blue(u64),
    Red(u64),
    Green(u64),
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input02.txt").unwrap();

    let game = parse(input);

    // Your solution here...
    let sol1: u64 = game.iter()
        .filter(|g| is_valid_game(g))
        .map(|g| g.id)
        .sum();
    let sol2: u64 = game.iter()
        .map(minimal_cubes_product)
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse(input: String) -> Vec<Game> {
    let mut final_input = Vec::new();

    for line in input.lines() {
        
        let aux: Vec<&str> = line.split(": ").collect();
        let id: u64 = aux[0][5..].parse().unwrap();

        let mut game = Game {
            id,
            game: Vec::new(),
        };

        for turn in aux[1].split("; ") {
            let mut t = Turn::new();

            for ball in turn.split(", ") {
                let aux2 = ball.split(' ').collect::<Vec<&str>>();
                let num: u64 = aux2[0].parse().unwrap();
                let b = match aux2[1] {
                    "blue" => Ball::Blue(num),
                    "red" => Ball::Red(num),
                    "green" => Ball::Green(num),
                    _ => panic!("Invalid ball color: {}", ball),
                };

                t.push(b);
            }

            game.game.push(t);
        }

        final_input.push(game);
    }

    final_input

}

fn is_valid_game(game: &Game) -> bool {
    let max_red = 12;
    let max_blue = 14;
    let max_green = 13;

    for turn in &game.game {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for ball in turn {
            match ball {
                Ball::Red(n) => red += n,
                Ball::Blue(n) => blue += n,
                Ball::Green(n) => green += n,
            }
        }

        if red > max_red || blue > max_blue || green > max_green {
            return false;
        }
    }

    true
}

fn minimal_cubes_product(game: &Game) -> u64 {
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;

    for turn in &game.game {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for ball in turn {
            match ball {
                Ball::Red(n) => red += n,
                Ball::Blue(n) => blue += n,
                Ball::Green(n) => green += n,
            }
        }

        if red > max_red {
            max_red = red;
        }
        if blue > max_blue {
            max_blue = blue;
        }
        if green > max_green {
            max_green = green;
        }
    }

    max_red * max_blue * max_green
}
