use std::fs;

struct Mapping {
    src: u64,
    dst: u64,
    range: u64,
}

struct Map {
    ss: Vec<Mapping>,
    sf: Vec<Mapping>,
    fw: Vec<Mapping>,
    wl: Vec<Mapping>,
    lt: Vec<Mapping>,
    th: Vec<Mapping>,
    hl: Vec<Mapping>,
}

impl Map {
    fn new() -> Self {
        Self {
            ss: Vec::new(),
            sf: Vec::new(),
            fw: Vec::new(),
            wl: Vec::new(),
            lt: Vec::new(),
            th: Vec::new(),
            hl: Vec::new(),
        }
    }

    fn add_map(&mut self, idx: usize, m: Mapping) {
        match idx {
            0 => self.ss.push(m),
            1 => self.sf.push(m),
            2 => self.fw.push(m),
            3 => self.wl.push(m),
            4 => self.lt.push(m),
            5 => self.th.push(m),
            6 => self.hl.push(m),
            _ => panic!("Nope"),
        };
    }

    fn get_location(&self, seed: u64) -> u64 {
        let mut val = seed;
        for map in &self.ss {
            if val >= map.src && val < map.src + map.range {
                val = val - map.src + map.dst;
                break;
            }
        }
        for map in &self.sf {
            if val >= map.src && val < map.src + map.range {
                val = val - map.src + map.dst;
                break;
            }
        }
        for map in &self.fw {
            if val >= map.src && val < map.src + map.range {
                val = val - map.src + map.dst;
                break;
            }
        }
        for map in &self.wl {
            if val >= map.src && val < map.src + map.range {
                val = val - map.src + map.dst;
                break;
            }
        }
        for map in &self.lt {
            if val >= map.src && val < map.src + map.range {
                val = val - map.src + map.dst;
                break;
            }
        }
        for map in &self.th {
            if val >= map.src && val < map.src + map.range {
                val = val - map.src + map.dst;
                break;
            }
        }
        for map in &self.hl {
            if val >= map.src && val < map.src + map.range {
                val = val - map.src + map.dst;
                break;
            }
        }

        val
    }
}

fn fertilizer1(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().split("\n\n").collect();

    let mut m = Map::new();

    let seeds: Vec<_> = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    // maps
    for i in 0..7 {
        let ssmap: Vec<_> = lines[i + 1]
            .split('\n')
            .into_iter()
            .skip(1)
            .map(|s| {
                s.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect();

        for map in &ssmap {
            m.add_map(
                i,
                Mapping {
                    src: map[1],
                    dst: map[0],
                    range: map[2],
                },
            );
        }
    }

    seeds
        .into_iter()
        .map(|s| m.get_location(s))
        .min()
        .unwrap()
        .try_into()
        .unwrap()
}

fn fertilizer2(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().split("\n\n").collect();

    let mut m = Map::new();

    let seeds: Vec<_> = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut v : Vec<(u64, u64)> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        v.push((seeds[i], seeds[i+1]));
    }

    // maps
    for i in 0..7 {
        let ssmap: Vec<_> = lines[i + 1]
            .split('\n')
            .into_iter()
            .skip(1)
            .map(|s| {
                s.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect();

        for map in &ssmap {
            m.add_map(
                i,
                Mapping {
                    src: map[1],
                    dst: map[0],
                    range: map[2],
                },
            );
        }
    }

    let mut min = 10000000000;
    for (s, r) in v {
        for i in s..s+r {
            let res = m.get_location(i);
            if res < min {
                min  = res;
            }
        }
        println!("+");
    }

    min.try_into().unwrap()
}

fn main() {
    let f = fs::read_to_string("05.txt").unwrap();
    let part1 = fertilizer1(&f);
    let part2 = fertilizer2(&f);

    println!("part1: {}\npart2: {}", part1, part2);
}
