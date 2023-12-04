use std::fs;

const NUMS: &[usize] = &[0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];

fn scratchcards1(card: Vec<&str>) -> usize {
    card.iter()
        .map(|c| {
            // strip out first part
            let cn = c.split(":").nth(1).unwrap().trim();

            // winning numbers
            let v1: Vec<usize> = cn
                .split("|")
                .nth(0)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect();

            // what we have
            let v2: Vec<usize> = cn
                .trim()
                .split("|")
                .nth(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect();

            // check numbers to see if we are winning
            v2.iter().map(|v| v1.contains(v) as usize).sum::<usize>()
        })
        .map(|v| NUMS[v])
        .sum()
}

fn scratchcards2(card: Vec<&str>) -> usize {
    let wins: Vec<usize> = card
        .iter()
        .map(|c| {
            // strip out first part
            let cn = c.split(":").nth(1).unwrap().trim();

            // winning numbers
            let v1: Vec<usize> = cn
                .split("|")
                .nth(0)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect();

            // what we have
            let v2: Vec<usize> = cn
                .trim()
                .split("|")
                .nth(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect();

            // get the number of winnings for each card
            v2.iter().map(|v| v1.contains(v) as usize).sum::<usize>()
        })
        .collect();

    let mut scr = vec![1; wins.len()];

    // println!("{:?}", wins);
    for (i, w) in wins.iter().enumerate() {
        for j in i + 1..i + 1 + w {
            scr[j] += scr[i];
        }
    }

    scr.iter().sum()
}

fn main() {
    let f = fs::read_to_string("04.txt").unwrap();
    let lines: Vec<&str> = f.trim().split('\n').collect();
    let part1 = scratchcards1(lines.clone());
    let part2 = scratchcards2(lines);

    println!("part1: {}\npart2: {}", part1, part2);
}
