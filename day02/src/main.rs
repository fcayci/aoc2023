use std::fs;

struct Cubes {
    red: usize,
    blue: usize,
    green: usize,
}

fn conundrum1(games: Vec<&str>, bucket: &Cubes) -> usize {
    let mut sum: usize = 0;

    for game in &games {
        // get the gameid
        let gameid = game.split(":").nth(0).unwrap();
        let gameid: usize = gameid.split(" ").nth(1).unwrap().parse().unwrap();
        println!("Game ID: {}", gameid);
        // get the games
        let allgames = game.split(":").nth(1).unwrap();
        let mut res = true;
        'm: for games in allgames.split(";").into_iter() {
            for game in games.split(",").into_iter() {
                let color = game.trim().split(" ").nth(1).unwrap();
                let num: usize = game.trim().split(" ").nth(0).unwrap().parse().unwrap();
                if color == "red" && bucket.red < num {
                    res = false;
                    break 'm;
                }
                if color == "blue" && bucket.blue < num {
                    res = false;
                    break 'm;
                }
                if color == "green" && bucket.green < num {
                    res = false;
                    break 'm;
                }
            }
        }
        if res {
            sum += gameid;
        }
    }

    sum
}

fn conundrum2(games: Vec<&str>) -> usize {
    let mut sum: usize = 0;

    for game in &games {
        // get the gameid
        let gameid = game.split(":").nth(0).unwrap();
        let gameid: usize = gameid.split(" ").nth(1).unwrap().parse().unwrap();
        println!("Game ID: {}", gameid);
        // get the games
        let allgames = game.split(":").nth(1).unwrap();
        for games in allgames.split(";").into_iter() {
            for game in games.split(",").into_iter() {
                let _color = game.trim().split(" ").nth(1).unwrap();
                let _num: usize = game.trim().split(" ").nth(0).unwrap().parse().unwrap();
            }
        }
        sum += gameid;
    }

    sum
}

fn main() {
    let cubes = Cubes {
        red: 12,
        blue: 14,
        green: 13,
    };
    let f = fs::read_to_string("02.txt").unwrap();
    let lines: Vec<&str> = f.trim().split('\n').collect();
    let part1 = conundrum1(lines.clone(), &cubes);
    let part2 = conundrum2(lines);

    println!("part1: {}\npart2: {}", part1, part2);
}
