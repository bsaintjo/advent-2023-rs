use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct Game {
    idx: usize,
    bags: Vec<Bag>,
}

impl Game {
    fn new(idx: usize, bags: Vec<Bag>) -> Self {
        Self { idx, bags }
    }

    fn parse_game(s: &str) -> Self {
        let mut splitted = s.split([':', ';']);
        let idx = splitted
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let bags = splitted
            .map(|bag| {
                let cubes: Vec<_> = bag
                    .split(',')
                    .map(|g| {
                        let cube = Cube::parse_cube(g.split_ascii_whitespace());
                        cube
                    })
                    .collect();

                Bag::from_cubes(&cubes)
            })
            .collect::<Vec<_>>();
        Game { idx, bags }
    }

    fn all_possible(&self) -> bool {
        self.bags.iter().all(|b| b.possible())
    }

    fn min_necessary_power(&self) -> usize {
        let mut red = 1;
        let mut green = 1;
        let mut blue = 1;
        for bag in self.bags.iter() {
            if bag.red > red {
                red = bag.red;
            }
            if bag.green > green {
                green = bag.green;
            }

            if bag.blue > blue {
                blue = bag.blue;
            }
        }
        red * green * blue
    }
}

#[derive(Debug, PartialEq)]
struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

impl Bag {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }

    fn from_cubes(cs: &[Cube]) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for c in cs.iter() {
            match c {
                Cube::Red(x) => red = *x,
                Cube::Green(x) => green = *x,
                Cube::Blue(x) => blue = *x,
            }
        }
        Self::new(red, green, blue)
    }
    fn possible(&self) -> bool {
        (self.red <= 12) && (self.green <= 13) && (self.blue <= 14)
    }
}

#[derive(PartialEq, Debug)]
enum Cube {
    Red(usize),
    Blue(usize),
    Green(usize),
}

impl Cube {
    fn parse_cube<'a, I: Iterator<Item = &'a str>>(mut num_color: I) -> Self {
        let num = num_color.next().unwrap().parse::<usize>().unwrap();
        let color = num_color.next().unwrap();
        match color {
            "blue" => Self::Blue(num),
            "red" => Self::Red(num),
            "green" => Self::Green(num),
            _ => panic!("Unknown color: {color}"),
        }
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let mut sum = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        let game = Game::parse_game(&line);
        sum += game.min_necessary_power();
    }
    println!("{sum}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cube() {
        let xs = ["3", "blue"].into_iter();
        assert_eq!(Cube::parse_cube(xs), Cube::Blue(3));

        let xs = ["4", "red"].into_iter();
        assert_eq!(Cube::parse_cube(xs), Cube::Red(4));

        let xs = ["12", "green"].into_iter();
        assert_eq!(Cube::parse_cube(xs), Cube::Green(12));
    }

    #[test]
    fn test_parse_game() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::parse_game(s);
        assert_eq!(game, Game::new(1, vec![
            Bag::new(4, 0, 3),
            Bag::new(1, 2, 6),
            Bag::new(0, 2, 0),
        ]));
        assert!(game.all_possible());
        assert_eq!(game.min_necessary_power(), 48);

        let s = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = Game::parse_game(s);
        assert_eq!(game, Game::new(3, vec![
            Bag::new(20, 8, 6),
            Bag::new(4, 13, 5),
            Bag::new(1, 5, 0),
        ]));
        assert!(!game.all_possible());
        assert_eq!(game.min_necessary_power(), 1560);
    }
}
