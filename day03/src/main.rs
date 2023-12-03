use std::fs;

fn check_around(m: &Vec<Vec<char>>, sj: usize, ej: usize, i: usize) -> bool {
    // expand up down boundaries
    let cs = i.saturating_sub(1);
    let ce = if i == m.len() - 1 { m.len() - 1 } else { i + 1 };

    // expand left right boundaries
    let sj = sj.saturating_sub(1);
    let ej = if ej == m[i].len() - 1 {
        m[i].len() - 1
    } else {
        ej + 1
    };

    let mut found = false;

    'a: for i in cs..=ce {
        for j in sj..=ej {
            let val = m[i][j];
            if val != '.' && !val.is_digit(10) && val.is_ascii() {
                found = true;
                break 'a;
            }
        }
    }

    found
}

fn gear_ratios1(gears: Vec<&str>) -> usize {
    // Make a two dimensional char array
    let g: Vec<Vec<char>> = gears
        .iter()
        .map(|g| {
            // add one more dot-line to the end to make sure we process any edge numbers
            let mut s = g.chars().collect::<Vec<char>>();
            s.push('.');
            s
        })
        .collect();

    // println!("{:?}", g);

    let mut sum: usize = 0;
    for (i, row) in g.iter().enumerate() {
        let mut sj: isize = -1;
        let mut ej: isize = -1;
        let mut num = 0;
        for (j, _) in row.iter().enumerate() {
            let curr = g[i][j];
            if curr != '.' && curr.is_digit(10) {
                // only update start index once
                if sj == -1 {
                    sj = j as isize;
                }
                // update end index always
                ej = j as isize;
                // update number
                num = num * 10 + curr.to_digit(10).unwrap();
            } else {
                // we got a number, let's check
                if num != 0 {
                    if check_around(&g, sj as usize, ej as usize, i) {
                        println!("{}", num);
                        sum += num as usize;
                    }
                }
                sj = -1;
                num = 0;
            }
        }
    }
    sum
}

#[derive(Debug)]
struct Num {
    num: usize,
    // pos: (usize, usize),
    starpos: (usize, usize),
}

fn check_star(m: &Vec<Vec<char>>, num: usize, sj_: usize, ej: usize, row: usize) -> Option<Num> {
    // expand up down boundaries
    let cs = row.saturating_sub(1);
    let ce = if row == m.len() - 1 {
        m.len() - 1
    } else {
        row + 1
    };

    // expand left right boundaries
    let sj = sj_.saturating_sub(1);
    let ej = if ej == m[row].len() - 1 {
        m[row].len() - 1
    } else {
        ej + 1
    };

    for i in cs..=ce {
        for j in sj..=ej {
            let val = m[i][j];
            if val == '*' {
                return Some(Num {
                    num,
                    // pos : (sj_, row),
                    starpos: (i, j),
                });
            }
        }
    }

    return None;
}

fn gear_ratios2(gears: Vec<&str>) -> usize {
    // Make a two dimensional char array
    let g: Vec<Vec<char>> = gears
        .iter()
        .map(|g| {
            // add one more dot-line to the end to make sure we process any edge numbers
            let mut s = g.chars().collect::<Vec<char>>();
            s.push('.');
            s
        })
        .collect();

    let mut nums: Vec<Num> = Vec::new();
    for (i, row) in g.iter().enumerate() {
        let mut sj: isize = -1;
        let mut ej: isize = -1;
        let mut num = 0;
        for (j, _) in row.iter().enumerate() {
            let curr = g[i][j];
            if curr != '.' && curr.is_digit(10) {
                // only update start index once
                if sj == -1 {
                    sj = j as isize;
                }
                // update end index always
                ej = j as isize;
                // update number
                num = num * 10 + curr.to_digit(10).unwrap();
            } else {
                // we got a number, let's check
                if num != 0 {
                    if let Some(x) = check_star(&g, num as usize, sj as usize, ej as usize, i) {
                        nums.push(x);
                    }
                }
                sj = -1;
                num = 0;
            }
        }
    }

    // println!("{:?}", nums);

    // iterate over the vector to get multiples
    let mut sum = 0;
    for num in &nums {
        let v: Vec<_> = nums.iter().filter(|n| n.starpos == num.starpos).collect();
        // get exactly two, multiply numbers and add to sum
        if v.len() == 2 {
            // println!("{:?}", v);
            sum += v[0].num * v[1].num;
        }
    }

    // since we will count for both numbers twice, divide sum by 2 to get the result
    sum / 2
}

fn main() {
    let f = fs::read_to_string("03.txt").unwrap();
    let lines: Vec<&str> = f.trim().split('\n').collect();
    let part1 = gear_ratios1(lines.clone());
    let part2 = gear_ratios2(lines);

    println!("part1: {}\npart2: {}", part1, part2);
}
