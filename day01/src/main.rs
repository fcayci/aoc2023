use regex::Regex;
use std::fs;

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// str -> usize conversion
/// with custom NUMS array
fn convert(s: &str) -> usize {
    match s.parse::<usize>() {
        Ok(n) => n,
        _ => NUMS.iter().position(|&r| r == s).unwrap() + 1,
    }
}

/// Regex solution
fn trebuchet1(lines: Vec<&str>) -> usize {
    let re1 = Regex::new(r"(\d)").unwrap();
    // let re2 = Regex::new(r"(\d)(?!.*\d)").unwrap();
    //   does not support look-ahead: ?!
    let re2 = Regex::new(r"(\d)\D*$").unwrap();

    lines
        .into_iter()
        .map(|line| {
            let tens = convert(&re1.captures(line).unwrap()[1]);
            let ones = convert(&re2.captures(line).unwrap()[1]);
            tens * 10 + ones
        })
        .sum()
}

/// regex last digit is hard to get right without look-ahead
/// so alternative match_indices solution
fn trebuchet2(lines: Vec<&str>) -> usize {
    lines
        .into_iter()
        .map(|line| {
            let mut v: Vec<_> = line.match_indices(char::is_numeric).collect();

            for n in NUMS {
                v.extend(line.match_indices(n));
            }

            v.sort();

            let v: Vec<_> = v.into_iter().map(|(_, v)| convert(v)).collect();
            v.first().unwrap() * 10 + v.last().unwrap()
        })
        .sum()
}

fn main() {
    let f = fs::read_to_string("01.txt").unwrap();
    let lines: Vec<&str> = f.trim().split('\n').collect();
    let part1 = trebuchet1(lines.clone());
    let part2 = trebuchet2(lines);

    // alternatively for part2, we could replace the strings with numbers before doing anything
    // but there are strings like twone which ends up being tw1 instead of 2ne (depending on the
    // order of replacements) so this approach does not work.

    println!("part1: {}\npart2: {}", part1, part2);
}
