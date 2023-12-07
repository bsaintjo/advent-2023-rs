use std::io::{self, BufRead};

fn convert_spelled_to_digits(s: &str) -> u32 {
    match s {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("{s}"),
    }
}

fn extract_digits(s: &str) -> u32 {
    let mut x = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ]
    .iter()
    .flat_map(|&n| s.match_indices(n))
    .collect::<Vec<_>>();
    x.sort_by_key(|x| x.0);

    let fst = convert_spelled_to_digits(x[0].1);
    let snd = convert_spelled_to_digits(x.last().unwrap_or(&x[0]).1);
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
        assert_eq!(extract_digits("two1nine"), 29);
        assert_eq!(extract_digits("eightwothree"), 83);
        assert_eq!(extract_digits("abcone2threexyz"), 13);
        assert_eq!(extract_digits("xtwone3four"), 24);
        assert_eq!(extract_digits("4nineeightseven2"), 42);
        assert_eq!(extract_digits("zoneight234"), 14);
        assert_eq!(extract_digits("7pqrstsixteen"), 76);
    }
}
