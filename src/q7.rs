use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines}, cmp::Ordering,
};

#[derive(Debug)]
struct Game {
    hands: Vec<Hand>,
}

impl Game {
    fn new(hands: Vec<Hand>) -> Self {
        Game { hands }
    }

    fn order_games(&mut self) {
        self.hands.sort();
    }

    fn print_hands(&self) {
        for hand in self.hands.iter() {
            println!("{}", hand.cards.iter().map(|c| c.value).collect::<String>().as_str());
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    point: u32,
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type == other.hand_type {
            for i in 0..self.cards.len() {
                if self.cards[i].value() > other.cards[i].value() {
                    return Some(Ordering::Greater);
                } else if self.cards[i].value() < other.cards[i].value() {
                    return Some(Ordering::Less);
                }
            }

            return Some(Ordering::Equal);
        }

        self.hand_type.partial_cmp(&other.hand_type)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


impl Hand {
    fn new(point: u32) -> Hand {
        Hand {
            cards: Vec::new(),
            point,
            hand_type: HandType::HighCard,
        }
    }

    fn new_with_cards(cards: String) -> Hand {
        let mut cards_vec: Vec<Card> = Vec::new();
        for card in cards.chars() {
            let card = Card::new(card);
            cards_vec.push(card);
        }

        Hand {
            cards: cards_vec,
            point: 0,
            hand_type: HandType::HighCard,
        }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn set_pair_type(&mut self) {
        let mut set: HashMap<char, usize> = HashMap::new();

        for card in self.cards.iter() {
            set.entry(card.value).and_modify(|e| *e += 1).or_insert(1);
        }

        let values = set.values();
        let mut okee: Vec<&usize> = values.collect();
        okee.sort();

        self.hand_type = match okee[..] {
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 4] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => HandType::HighCard,
        };
    }
}

#[derive(Debug)]
#[derive(PartialEq, PartialOrd, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp(&other)
    }
}


#[derive(PartialEq, PartialOrd, Eq, Debug)]
struct Card {
    value: char,
}

impl Card {
    fn new(value: char) -> Card {
        Card { value }
    }

    fn value(&self) -> u32 {
        match self.value {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => self.value.to_digit(10).unwrap(),
        }
    }
}

pub fn part1(lines: Lines<BufReader<File>>) {
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let (cards, vals) = line.split_once(' ').unwrap();

        let point = vals.parse::<u32>().unwrap();

        let mut hand = Hand::new(point);
        for card in cards.chars() {
            let card = Card::new(card);
            hand.add_card(card);
        }

        hand.set_pair_type();
        hands.push(hand);
    }

    let mut game = Game::new(hands);
    game.order_games();

    let mut sum = 0;
    for (i, hand) in game.hands.iter().enumerate() {
        println!("{}: {}", i, hand.point);
        sum += hand.point * (i as u32 + 1);
    }

    println!("Sum: {:?}", sum);
}
