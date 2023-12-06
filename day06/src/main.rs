fn wait_for_it1(input: &str) -> u64 {
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let times: Vec<_> = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let duration: Vec<_> = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut sum = 1;
    for (j, time) in times.iter().enumerate() {
        let dist = duration[j];
        let mut start = 0;
        // left
        for i in 1..*time {
            if i * (time - i) > dist {
                start = i;
                break;
            }
        }

        // right
        let mut end = 0;
        for i in (1..*time).rev() {
            if i * (time - i) > dist {
                end = i;
                break;
            }
        }

        sum *= end - start + 1;
    }

    sum
}

fn wait_for_it2(input: &str) -> u64 {
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let time: u64 = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .fold("".to_string(), |a, e| a + e)
        .parse::<u64>()
        .unwrap();

    let duration: u64 = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .fold("".to_string(), |a, e| a + e)
        .parse::<u64>()
        .unwrap();

    println!("\ntime: {:?}", time);
    println!("duration: {:?}", duration);
    println!("=========");

    // left
    let mut start = 0;
    for i in 1..time {
        if i * (time - i) > duration {
            start = i;
            break;
        }
    }

    // right
    let mut end = 0;
    for i in (1..time).rev() {
        if i * (time - i) > duration {
            end = i;
            break;
        }
    }

    end - start + 1
}

fn main() {
    let test: &str = "Time:      7  15   30\nDistance:  9  40  200";

    let part1 = wait_for_it1(test);
    let part2 = wait_for_it2(test);

    println!("part1: {}\npart2: {}", part1, part2);

    let input: &str = "Time:        62     64     91     90\nDistance:   553   1010   1473   1074";

    let part1 = wait_for_it1(input);
    let part2 = wait_for_it2(input);

    println!("part1: {}\npart2: {}", part1, part2);
}
