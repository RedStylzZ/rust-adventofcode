use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: i64,
    rank: Rank,
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.rank.cmp(&other.rank);
        if cmp != Ordering::Equal {
            return cmp;
        }

        for (i, _) in self.cards.iter().enumerate() {
            let card_cmp = self.cards[i].cmp(&other.cards[i]);

            if card_cmp == Ordering::Equal {
                continue;
            }

            return card_cmp;
        }
        panic!("Cards are equal")
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

fn get_rank(hand: &Hand) -> Rank {
    let mut card_count: HashMap<&Card, u8> = HashMap::new();

    hand.cards.iter().for_each(|x| {
        if !card_count.contains_key(x) {
            card_count.insert(x, 1);
        } else {
            card_count.entry(x).and_modify(|u| *u += 1);
        }
    });

    for (_, i) in &card_count {
        if *i == 5 {
            return Rank::FiveOfKind;
        }
        if *i == 4 {
            return Rank::FourOfKind;
        }
    }

    if card_count.iter().any(|(_, i)| *i == 3) {
        if card_count.iter().any(|(_, i)| *i == 2) {
            return Rank::FullHouse;
        }
        return Rank::ThreeOfKind;
    }

    let mut pair_count = 0;
    card_count.iter().for_each(|(_, i)| {
        if *i == 2 {
            pair_count += 1
        }
    });

    match pair_count {
        1 => return Rank::OnePair,
        2 => return Rank::TwoPair,
        _ => (),
    }

    return Rank::HighCard;
}

fn check_game(hands: &mut Vec<Hand>) {
    hands.iter_mut().for_each(|h| {
        let rank = get_rank(h);
        h.rank = rank;
    });

    hands.sort();
}

fn parse_hand(line: &&str) -> Hand {
    let parts: Vec<&str> = line.split(" ").collect();
    let bid: i64 = parts[parts.len() - 1].parse().unwrap();

    let cards: Vec<Card> = parts[0]
        .chars()
        .map(|c| match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::NINE,
            '8' => Card::EIGHT,
            '7' => Card::SEVEN,
            '6' => Card::SIX,
            '5' => Card::FIVE,
            '4' => Card::FOUR,
            '3' => Card::THREE,
            '2' => Card::TWO,
            _ => panic!("Invalid card"),
        })
        .collect();

    Hand {
        cards,
        bid,
        rank: Rank::HighCard,
    }
}

fn part_one(lines: &Vec<&str>) -> i64 {
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        hands.push(parse_hand(line));
    }

    check_game(&mut hands);

    let mut sum = 0;

    hands.iter().enumerate().for_each(|(rank, h)| {
        sum += (rank as i64 + 1) * h.bid;
    });

    return sum;
}

fn main() {
    let file_path = "input/day07.txt";

    println!("---------- Day07 ----------");
    println!("Reading {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Read file");

    let lines = contents.lines().collect();

    let sum_one = part_one(&lines);

    println!("PartOne:\t{sum_one}");
}
