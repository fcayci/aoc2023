use std::fs;

enum ExtDirection {
    RIGHT,
    LEFT,
}

// recursively call extrapolate to find the last value
fn extrapolate(values: &Vec<i64>, dir: ExtDirection) -> i64 {
    // stop condition
    if values.iter().all(|v| *v == 0) {
        return 0;
    }

    let mut v: Vec<i64> = Vec::new();
    for (i, _) in values.iter().enumerate() {
        if i == values.len() - 1 {
            break;
        }
        v.push(values[i + 1] - values[i]);
    }

    match dir {
        ExtDirection::RIGHT => values[values.len() - 1] + extrapolate(&v, ExtDirection::RIGHT),
        ExtDirection::LEFT => values[0] - extrapolate(&v, ExtDirection::LEFT),
    }
}

fn mirage1(input: &str) -> i64 {
    let mirage: Vec<_> = input
        .split('\n')
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    mirage
        .into_iter()
        .map(|v| extrapolate(&v, ExtDirection::RIGHT))
        .sum()
}

fn mirage2(input: &str) -> i64 {
    let mirage: Vec<_> = input
        .split('\n')
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    mirage
        .into_iter()
        .map(|v| extrapolate(&v, ExtDirection::LEFT))
        .sum()
}

fn main() {
    let f = fs::read_to_string("09.txt").unwrap();

    let part1 = mirage1(&f);
    let part2 = mirage2(&f);

    println!("part1: {}\npart2: {}", part1, part2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mirage1() {
        let test = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        use super::mirage1;

        assert_eq!(mirage1(test), 114);
    }

    #[test]
    fn test_mirage2() {
        let test = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        use super::mirage2;

        assert_eq!(mirage2(test), 2);
    }
}
