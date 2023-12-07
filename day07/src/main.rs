use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(Eq, PartialEq, PartialOrd, Debug)]
enum Cards {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, Eq)]
struct Hand {
    val: String,
    bid: usize,
    hand: Cards,
}

// We need to custom compare Hands
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.val == other.val
    }
}

// If hand is stronger, take that, if equal check the card order. Cards are substituted to other
// letters to make this built-in comparison.
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand > other.hand {
            return Ordering::Greater;
        } else if self.hand == other.hand {
            if self.val > other.val {
                return Ordering::Greater;
            } else if self.val < other.val {
                return Ordering::Less;
            } else {
                return Ordering::Equal;
            }
        }
        return Ordering::Less;
    }
}

#[derive(Debug)]
struct Game {
    cards: Vec<Hand>,
}

fn get_cards(input: &str) -> Cards {
    assert_eq!(input.len(), 5);

    // build a hashmap of character frequencies
    let mut cards: HashMap<char, u32> = HashMap::new();

    for c in input.chars() {
        *cards.entry(c).or_insert(0) += 1;
    }

    // get the frequency of each card, sort it, then convert to string for matching below
    let mut val = cards.iter().map(|(_, k)| k.clone()).collect::<Vec<u32>>();
    val.sort();
    let val = val.iter().fold("".to_string(), |a, e| a + &e.to_string());

    // return the hand for only possible values
    match &val[..] {
        "11111" => Cards::HighCard,
        "1112" => Cards::OnePair,
        "122" => Cards::TwoPair,
        "113" => Cards::ThreeKind,
        "23" => Cards::FullHouse,
        "14" => Cards::FourKind,
        "5" => Cards::FiveKind,
        // Should not happen
        _ => panic!("Nope"),
    }
}

// part2 parsing requires joker addition
fn get_cards_joker(input: &str) -> Cards {
    assert_eq!(input.len(), 5);

    // parse normally if there is no J
    if !input.contains("J") {
        return get_cards(input);
    }

    // get rid of J and parse rest normally
    let input = input.replace("J", "");

    // build a hashmap of character frequencies
    let mut cards: HashMap<char, u32> = HashMap::new();

    for c in input.chars() {
        *cards.entry(c).or_insert(0) += 1;
    }

    // get the frequency of each card, sort it, then convert to string for matching below
    let mut val = cards.iter().map(|(_, k)| k.clone()).collect::<Vec<u32>>();
    val.sort();
    let val = val.iter().fold("".to_string(), |a, e| a + &e.to_string());

    // this map is different, we have more cases depending on how many J's are in the hand
    match &val[..] {
        // 1 J
        "1111" => Cards::OnePair,
        "112" => Cards::ThreeKind,
        "22" => Cards::FullHouse,
        "13" => Cards::FourKind,
        "4" => Cards::FiveKind,
        // 2 J
        "111" => Cards::ThreeKind,
        "12" => Cards::FourKind,
        "3" => Cards::FiveKind,
        // 3 J
        "11" => Cards::FourKind,
        "2" => Cards::FiveKind,
        // 4 J
        "1" => Cards::FiveKind,
        // 5 J
        "" => Cards::FiveKind,
        // Should not happen
        _ => panic!("Nope"),
    }
}

impl Game {
    fn parse1(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().split('\n').collect();
        Self {
            cards: lines
                .into_iter()
                .map(|s| {
                    let split = s.split_once(' ').unwrap();
                    let val = split.0;

                    // Create a Hand, but replace the cards with better letters
                    // for easy sorting
                    Hand {
                        val: val
                            .replace("A", "E")
                            .replace("K", "D")
                            .replace("Q", "C")
                            .replace("J", "B")
                            .replace("T", "A")
                            .to_string(),
                        bid: split.1.parse::<usize>().unwrap(),
                        hand: get_cards(&val),
                    }
                })
                .collect::<Vec<Hand>>(),
        }
    }

    fn parse2(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().split('\n').collect();
        Self {
            cards: lines
                .into_iter()
                .map(|s| {
                    let split = s.split_once(' ').unwrap();
                    let val = split.0;

                    // Create a Hand, but replace the cards with better letters
                    // for easy sorting.
                    // Replace J with 1 to make it the weakest card
                    Hand {
                        val: val
                            .replace("A", "E")
                            .replace("K", "D")
                            .replace("Q", "C")
                            .replace("J", "1")
                            .replace("T", "A")
                            .to_string(),
                        bid: split.1.parse::<usize>().unwrap(),
                        hand: get_cards_joker(&val),
                    }
                })
                .collect::<Vec<Hand>>(),
        }
    }

    // go over the cards, accumulate bid * rank
    fn get_winning(&mut self) -> usize {
        self.cards.sort();
        self.cards
            .iter()
            .enumerate()
            .fold(0, |a, (i, e)| a + e.bid * (i + 1))
    }
}

fn camel_cards1(input: &str) -> u64 {
    let mut game = Game::parse1(input);
    game.get_winning().try_into().unwrap()
}

fn camel_cards2(input: &str) -> u64 {
    let mut game = Game::parse2(input);
    game.get_winning().try_into().unwrap()
}

fn main() {
    let f = fs::read_to_string("07.txt").unwrap();

    let part1 = camel_cards1(&f);
    let part2 = camel_cards2(&f);

    println!("part1: {}\npart2: {}", part1, part2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let test: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        use super::camel_cards1;

        assert_eq!(camel_cards1(test), 6440);
    }

    #[test]
    fn test_part2() {
        let test: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        use super::camel_cards2;

        assert_eq!(camel_cards2(test), 5905);
    }
}
