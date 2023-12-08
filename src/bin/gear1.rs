use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash)]
struct Coordinate {
    row: isize,
    column: isize,
}

struct Part {
    n: usize,
    coordinate: Coordinate,
    len: isize,
}

impl Part {
    fn surrounding(&self) -> HashSet<Coordinate> {
        let mut surround = HashSet::new();

        for y  in self.coordinate.column - 1 ..= self.coordinate.column + 1 {
            for x in self.coordinate.row - 1 ..= self.coordinate.row + self.len {
                surround.insert(Coordinate { row: x, column: y });
            }
        }
        surround
    }
}

struct Schematic {
    parts: Vec<Part>,
    symbols: HashSet<Coordinate>,
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_surrounding() {
        let part = Part {
            n: 12,
            coordinate: Coordinate { row: 1, column: 1 },
            len: 2,
        };
    }
}