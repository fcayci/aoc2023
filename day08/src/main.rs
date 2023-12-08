use std::{collections::HashMap, fs};

fn wasteland1(input: &str) -> u64 {
    let split = input.split_once("\n\n").unwrap();
    // instructions
    let inst = split.0;

    // hashmap of nodes
    let nodes: HashMap<String, (String, String)> = split
        .1
        .trim()
        .split('\n')
        .map(|s| {
            let split = s.split_once(" = ").unwrap();
            let me = split.0.trim().to_string();
            let trim: &[_] = &['(', ')'];
            let split = split.1.trim_matches(trim).split_once(',').unwrap();
            let left = split.0.trim().to_string();
            let right = split.1.trim().to_string();
            (me, (left, right))
        })
        .collect();

    let mut index = String::from("AAA");
    let target = "ZZZ";
    let mut steps = 0;

    // traverse the map
    loop {
        for dir in inst.chars() {
            index = match dir {
                'L' => nodes.get(&index).unwrap().0.clone(),
                'R' => nodes.get(&index).unwrap().1.clone(),
                _ => panic!("Nope"),
            };
            steps += 1;
        }
        // stop condition
        if index.eq(&target) {
            break;
        }
    }

    steps
}

fn wasteland2(input: &str) -> u128 {
    let split = input.split_once("\n\n").unwrap();
    // instructions
    let inst = split.0;

    // hashmap of nodes
    let nodes: HashMap<String, (String, String)> = split
        .1
        .trim()
        .split('\n')
        .map(|s| {
            let split = s.split_once(" = ").unwrap();
            let me = split.0.trim().to_string();
            let trim: &[_] = &['(', ')'];
            let split = split.1.trim_matches(trim).split_once(',').unwrap();
            let left = split.0.trim().to_string();
            let right = split.1.trim().to_string();
            (me, (left, right))
        })
        .collect();

    let indexes: Vec<_> = nodes
        .iter()
        .filter(|s| s.0.ends_with('A'))
        .map(|s| s.0)
        .collect();

    // traverse the map for each part to find the minimum number of steps
    // it takes to reach to Z then find least common multiplier
    indexes
        .into_iter()
        .map(|ss| {
            let mut steps = 0;
            let mut s = ss.clone();
            loop {
                for dir in inst.chars() {
                    s = match dir {
                        'L' => nodes.get(&s).unwrap().0.to_owned(),
                        'R' => nodes.get(&s).unwrap().1.to_owned(),
                        _ => panic!("Nope"),
                    };
                    steps += 1;
                }
                // stop condition
                if s.ends_with('Z') {
                    break steps;
                }
            }
        })
        .reduce(|r, a| lcm(r, a))
        .unwrap()
        .try_into()
        .unwrap()
}

// find least common multiplier
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

// find greatest common divisor
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn main() {
    let f = fs::read_to_string("08.txt").unwrap();

    let part1 = wasteland1(&f);
    let part2 = wasteland2(&f);

    println!("part1: {}\npart2: {}", part1, part2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1a() {
        let test = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        use super::wasteland1;

        assert_eq!(wasteland1(test), 2);
    }

    #[test]
    fn test_part1b() {
        let test = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        use super::wasteland1;

        assert_eq!(wasteland1(test), 6);
    }

    #[test]
    fn test_part2() {
        let test = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        use super::wasteland2;

        assert_eq!(wasteland2(test), 6);
    }
}
