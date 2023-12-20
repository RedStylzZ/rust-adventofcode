use std::{cmp::Ordering, collections::HashMap, fs};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    J,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    T,
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

fn get_rank_part_two(hand: &Hand) -> Rank {
    use itertools::Itertools;
    // let mut card_count: HashMap<&Card, u8> = HashMap::new();

    let mut card_count: HashMap<&Card, usize> = hand.cards.iter().counts();

    // hand.cards.iter().for_each(|x| {
    //     if !card_count.contains_key(x) {
    //         card_count.insert(x, 1);
    //     } else {
    //         card_count.entry(x).and_modify(|u| *u += 1);
    //     }
    // });

    let joker_count = card_count.get(&Card::J).unwrap_or(&0).clone();
    if joker_count > 0 {
        let mut highest_card: Option<&Card> = None;
        let mut highest_count: &usize = &0;
        for (k, v) in &card_count {
            if **k != Card::J && v > highest_count {
                highest_count = v;
                highest_card = Some(k);
            }
        }

        if highest_card != None {
            card_count
                .entry(highest_card.unwrap())
                .and_modify(|x| *x += joker_count);

            card_count.remove(&Card::J);
        }
    }

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
        0 => (),
        _ => panic!("multiple pairs"),
    }

    return Rank::HighCard;
}

fn part_two(lines: &Vec<&str>) -> i64 {
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let mut hand = parse_hand(line);
        hand.rank = get_rank_part_two(&hand);
        hands.push(hand);
    }

    hands.sort();

    hands
        .iter()
        .enumerate()
        .fold(0, |sum, (rank, h)| sum + (rank + 1) as i64 * h.bid)
}

fn main() {
    let file_path = "input/day07.txt";

    println!("---------- Day07 ----------");
    println!("Reading {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Read file");

    let lines = contents.lines().collect();

    let sum_two = part_two(&lines);

    println!("PartTwo:\t{sum_two}");

    assert_eq!(251135960, sum_two);
}
