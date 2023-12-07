use std::io::{self, BufRead};

fn extract_digits(s: &str) -> u32 {
    let d = s.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>();
    let fst = d[0].to_digit(10).unwrap();
    let snd = d.last().unwrap_or(&d[0]).to_digit(10).unwrap();
    (fst * 10) + snd
}

fn main() {
    let stdin = io::stdin().lock();
    let sum: u32 = stdin.lines().map(|l| extract_digits(&l.unwrap())).sum();
    println!("{sum}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_digits() {
        assert_eq!(extract_digits("1abc2"), 12);
        assert_eq!(extract_digits("pqr3stu8vwx"), 38);
        assert_eq!(extract_digits("a1b2c3d4e5f"), 15);
        assert_eq!(extract_digits("treb7uchet"), 77);
    }
}
