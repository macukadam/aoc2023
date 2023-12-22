use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
};

#[derive(Debug)]
struct Game<T: CardTrait> {
    hands: Vec<Hand<T>>,
}

impl<T: CardTrait + std::cmp::Eq> Game<T> {
    fn new(hands: Vec<Hand<T>>) -> Self {
        Game { hands }
    }

    fn order_games(&mut self) {
        self.hands.sort();
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Hand<T: CardTrait> {
    cards: Vec<T>,
    point: u32,
    hand_type: HandType,
}

impl<T: CardTrait + std::cmp::Eq> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: CardTrait + std::cmp::Eq> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for i in 0..self.cards.len() {
                match self.cards[i].value().cmp(&other.cards[i].value()) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => continue,
                }
            }

            return Ordering::Equal;
        }

        self.hand_type.partial_cmp(&other.hand_type).unwrap()
    }
}

impl<T: CardTrait> Hand<T> {
    fn new(point: u32) -> Hand<T> {
        Hand {
            cards: Vec::new(),
            point,
            hand_type: HandType::HighCard,
        }
    }

    fn add_card(&mut self, card: T) {
        self.cards.push(card);
    }

    fn set_hand_type(&mut self, strategy: &dyn HandTypeSettingStrategy<T>) {
        strategy.set_hand_type(self);
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
struct Card {
    value: char,
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
struct ModifiedCard {
    value: char,
}

pub trait CardTrait {
    fn new(value: char) -> Self;
    fn value(&self) -> u32;
    fn value_char(&self) -> char;
}

pub trait HandTypeSettingStrategy<T: CardTrait> {
    fn set_hand_type(&self, hand: &mut Hand<T>);
}

struct Q1Strategy;
struct Q2Strategy;

impl<T: CardTrait> HandTypeSettingStrategy<T> for Q1Strategy {
    fn set_hand_type(&self, hand: &mut Hand<T>) {
        let mut set: HashMap<char, usize> = HashMap::new();

        for card in hand.cards.iter() {
            set.entry(card.value_char())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        let values = set.values();
        let mut okee: Vec<&usize> = values.collect();
        okee.sort();

        hand.hand_type = match okee[..] {
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

impl<T: CardTrait> HandTypeSettingStrategy<T> for Q2Strategy {
    fn set_hand_type(&self, hand: &mut Hand<T>) {
        let mut set: HashMap<char, usize> = HashMap::new();

        for card in hand.cards.iter() {
            set.entry(card.value_char())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        let values = set.values();
        let mut okee: Vec<&usize> = values.collect();
        okee.sort();

        let num_of_j = set.get(&'J').unwrap_or(&0);

        hand.hand_type = match okee[..] {
            [1, 1, 1, 2] => {
                if *num_of_j == 2 || *num_of_j == 1 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::OnePair
                }
            }
            [1, 2, 2] => {
                if *num_of_j == 2 {
                    HandType::FourOfAKind
                } else if *num_of_j == 1 {
                    HandType::FullHouse
                } else {
                    HandType::TwoPair
                }
            }
            [1, 1, 3] => {
                if *num_of_j == 1 || *num_of_j == 3 {
                    HandType::FourOfAKind
                } else {
                    HandType::ThreeOfAKind
                }
            }
            [2, 3] => {
                if *num_of_j == 3 || *num_of_j == 2 {
                    HandType::FiveOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            [1, 4] => {
                if *num_of_j == 1 || *num_of_j == 4 {
                    HandType::FiveOfAKind
                } else {
                    HandType::FourOfAKind
                }
            }
            [5] => HandType::FiveOfAKind,
            _ => {
                if *num_of_j == 1 {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
        };
    }
}

impl CardTrait for ModifiedCard {
    fn new(value: char) -> Self {
        ModifiedCard { value }
    }

    fn value(&self) -> u32 {
        match self.value {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            'J' => 0,
            _ => self.value.to_digit(10).unwrap(),
        }
    }

    fn value_char(&self) -> char {
        self.value
    }
}

impl CardTrait for Card {
    fn new(value: char) -> Self {
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

    fn value_char(&self) -> char {
        self.value
    }
}

pub fn run<T: CardTrait + std::cmp::Eq>(lines: Lines<BufReader<File>>, strategy: &dyn HandTypeSettingStrategy<T>) {
    let mut hands: Vec<Hand<T>> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let (cards, vals) = line.split_once(' ').unwrap();

        let point = vals.parse::<u32>().unwrap();

        let mut hand = Hand::new(point);
        for card in cards.chars() {
            let card = T::new(card);
            hand.add_card(card);
        }

        hand.set_hand_type(strategy);
        hands.push(hand);
    }

    let mut game = Game::new(hands);
    game.order_games();

    let mut sum = 0;
    for (i, hand) in game.hands.iter().enumerate() {
        sum += hand.point * (i as u32 + 1);
    }

    println!("Sum: {:?}", sum);
}

pub fn part1(lines: Lines<BufReader<File>>) {
    run::<Card>(lines, &Q1Strategy);
}

pub fn part2(lines: Lines<BufReader<File>>) {
    run::<ModifiedCard>(lines, &Q2Strategy);
}
