use std::fs;

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn distance(first: &Galaxy, second: &Galaxy, vspace: &Vec<usize>, hspace: &Vec<usize>) -> usize {
    let (xmax, xmin) = if first.x > second.x {
        (first.x, second.x)
    } else {
        (second.x, first.x)
    };

    let (ymax, ymin) = if first.y > second.y {
        (first.y, second.y)
    } else {
        (second.y, first.y)
    };

    let mut diffx = xmax - xmin;
    let mut diffy = ymax - ymin;

    // expand vspace side
    for (i, v) in vspace.iter().enumerate() {
        if *v != 0 {
            if i > xmin && i < xmax {
                diffx += vspace[i];
            }
        }
    }

    // expand hspace side
    for (i, v) in hspace.iter().enumerate() {
        if *v != 0 {
            if i > ymin && i < ymax {
                diffy += hspace[i];
            }
        }
    }

    diffx + diffy
}

fn expansion(input: &str, expsize: usize) -> usize {
    let cosmos: Vec<Vec<_>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    // expansion space vectors
    // 1 means no galaxies, 0 means there is a galaxy on that line
    let mut hspace: Vec<usize> = vec![expsize - 1; cosmos.len()];
    let mut vspace: Vec<usize> = vec![expsize - 1; cosmos[0].len()];

    // fill in galaxies and expansion space
    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (y, line) in cosmos.iter().enumerate() {
        for (x, v) in line.iter().enumerate() {
            if *v == '#' {
                vspace[x] = 0;
                hspace[y] = 0;
                galaxies.push(Galaxy { x, y });
            }
        }
    }

    // go through galaxies and calculate the distance dynamicaally based on expansion spaces
    let mut sum = 0;
    for (i, _) in galaxies.iter().enumerate() {
        sum += galaxies
            .iter()
            .skip(i + 1)
            .map(|g| distance(&galaxies[i], g, &vspace, &hspace))
            .sum::<usize>();
    }

    sum
}

fn main() {
    let f = fs::read_to_string("11.txt").unwrap();

    let part1 = expansion(&f, 2);
    let part2 = expansion(&f, 1000000);

    println!("part1: {}\npart2: {}", part1, part2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_expansion() {
        let test = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        use super::expansion;

        assert_eq!(expansion(test, 2), 374);
    }

    #[test]
    fn test_expansion2() {
        let test = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        use super::expansion;

        assert_eq!(expansion(test, 10), 1030);
    }

    #[test]
    fn test_expansion3() {
        let test = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        use super::expansion;

        assert_eq!(expansion(test, 100), 8410);
    }
}
