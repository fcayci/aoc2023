use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;

#[derive(Eq, Debug)]
struct Round {
    red: Option<usize>,
    green: Option<usize>,
    blue: Option<usize>,
}

impl FromStr for Round {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut round = Round {
            red: None,
            blue: None,
            green: None,
        };
        for cube in input.trim().split(",").into_iter() {
            let color = cube.trim().split(" ").nth(1).unwrap();
            let num: usize = cube.trim().split(" ").nth(0).unwrap().parse().unwrap();
            match color {
                "red" => round.red = Some(num),
                "blue" => round.blue = Some(num),
                "green" => round.green = Some(num),
                _ => return Err(()),
            }
        }
        Ok(round)
    }
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> Ordering {
        let red = self.red.cmp(&other.red);
        let blue = self.blue.cmp(&other.blue);
        let green = self.green.cmp(&other.green);
        if red == Ordering::Equal && green == Ordering::Equal && blue == Ordering::Equal {
            return Ordering::Equal;
        } else if red == Ordering::Greater
            || green == Ordering::Greater
            || blue == Ordering::Greater
        {
            return Ordering::Greater;
        }
        return Ordering::Less;
    }
}

impl PartialEq for Round {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

fn conundrum1(games: Vec<&str>, bucket: Round) -> usize {
    games
        .into_iter()
        .map(|game| {
            // get the gameid
            let gameid = game.split(":").nth(0).unwrap();
            let gameid: usize = gameid.split(" ").nth(1).unwrap().parse().unwrap();

            // get the rounds
            let rounds = game.split(":").nth(1).unwrap();
            let mut goodgame = rounds
                .split(";")
                .map(|g| Round::from_str(g).unwrap())
                .filter(|r| r > &bucket)
                .peekable();

            // if there is anything left in filter, game is not good, so ignore
            // otherwise get the gameid
            if !goodgame.peek().is_some() {
                gameid
            } else {
                0
            }
        })
        .sum()
}


fn conundrum2(games: Vec<&str>) -> usize {
    0
}

fn main() {
    let cubes = Round {
        red: Some(12),
        blue: Some(14),
        green: Some(13),
    };
    let f = fs::read_to_string("02.txt").unwrap();
    let lines: Vec<&str> = f.trim().split('\n').collect();
    let part1 = conundrum1(lines.clone(), cubes);
    let part2 = conundrum2(lines);

    println!("part1: {}\npart2: {}", part1, part2);
}
