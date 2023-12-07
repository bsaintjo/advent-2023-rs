struct Game {
    idx: usize,
    bags: Vec<Bag>,
}

struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

impl Bag {
    fn possible(&self) -> bool {
        (self.red < 12) && (self.green < 13) && (self.blue < 14)
    }
}

fn main() {}
