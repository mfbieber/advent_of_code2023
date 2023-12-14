use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use futures::SinkExt;
use crate::day7::Type::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair, Unknown};
use crate::read_lines;

#[derive(Hash, Clone, Debug, Eq, Ord, PartialOrd)]
struct Card {
    label: String,
    strength: i32
}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.label == other.label
        && self.strength == other.strength;
    }
}

impl Card {

    fn calculate_strength(&mut self) {
        if self.label.starts_with("A") {
            self.strength = 15;
        } else if self.label.starts_with("K") {
            self.strength = 14;
        } else if self.label.starts_with("Q") {
            self.strength = 13
        } else if self.label.starts_with("J") {
            self.strength = 12;
        } else if self.label.starts_with("K") {
            self.strength = 11;
        } else if self.label.starts_with("T") {
            self.strength = 10;
        } else {
            self.strength = self.label.parse().unwrap();
        }
    }

    fn get_strength(&self) -> i32 {
        if self.label.starts_with("A") {
            return 14;
        } else if self.label.starts_with("K") {
            return 13;
        } else if self.label.starts_with("Q") {
            return 12
        } else if self.label.starts_with("J") {
            return 11;
        } else if self.label.starts_with("T") {
            return 10;
        } else {
            return self.label.parse().unwrap();
        }
    }
}

#[derive(PartialEq, Debug, Eq)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    Unknown
}

#[derive(Debug, Eq)]
struct Hand {
    cards: HashMap<Card, i32>,
    hand_vec: Vec<Card>,
    hand_type: Type,
    bid: i64
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.cards.len() != other.cards.len() {
            return false;
        }
        let mut self_keys: Vec<&Card> = self.cards.keys().collect();
        let mut other_keys: Vec<&Card> = other.cards.keys().collect();
        if self_keys.sort() != other_keys.sort() {
            return false;
        }
        let compare_maps: Option<bool> = self_keys.iter().map(|self_key| {
            return self.cards.get(self_key) == other.cards.get(self_key);
        }).filter(|bool| *bool).last();
        return compare_maps.is_some()
            && compare_maps.unwrap()
            && self.hand_type == other.hand_type;
    }
}

impl Hand {

    fn new(hand_string: String, bid: i64) -> Self {
        Hand::build_hand_from_string(hand_string, bid)
    }

    fn build_hand_from_string(hand_string: String, bid: i64) -> Hand {
        let mut cards: HashMap<Card, i32> = HashMap::new();
        let hand_vec: Vec<Card> = hand_string.chars()
            .map(|card_string| {
                let mut card = Card {
                    label: String::from(card_string),
                    strength: 0,
                };
                card.calculate_strength();
                return card;
            }).collect();

        let _ = hand_vec.iter().for_each(|card| {
            let count_option: Option<&i32> = cards.get(&card);
            if count_option.is_some() {
                cards.insert(card.clone(), count_option.unwrap() + 1);
            } else {
                cards.insert(card.clone(), 1);
            }
        });
        let hand_type: Type = Hand::determine_type_for_cards(&cards);
        return Hand { cards, hand_vec, hand_type, bid }
    }

    fn determine_type_for_cards(cards: &HashMap<Card, i32>) -> Type {
        let keys: Vec<Card> = cards.keys().cloned().collect::<Vec<Card>>();
        if keys.len() == 1 && cards.get(&keys[0]).unwrap() == &5 {
            return FiveOfAKind;
        } else if keys.len() == 2
            && (cards.get(&keys[0]).unwrap() == &4
            || cards.get(&keys[1]).unwrap() == &4) {
            return FourOfAKind;
        } else if keys.len() == 2
            && (cards.get(&keys[0]).unwrap() == &3
            || cards.get(&keys[1]).unwrap() == &3) {
            return  FullHouse;
        } else if keys.len() == 3
            && (cards.get(&keys[0]).unwrap() == &3
            || cards.get(&keys[1]).unwrap() == &3
            || cards.get(&keys[2]).unwrap() == &3) {
            return ThreeOfAKind;
        } else if keys.len() == 3
            && (cards.get(&keys[0]).unwrap() == &2
            || cards.get(&keys[1]).unwrap() == &2
            || cards.get(&keys[2]).unwrap() == &2) {
            return TwoPair;
        } else if keys.len() == 4
            && (cards.get(&keys[0]).unwrap() == &2
            || cards.get(&keys[1]).unwrap() == &2
            || cards.get(&keys[2]).unwrap() == &2
            || cards.get(&keys[3]).unwrap() == &2) {
            return OnePair;
        } else if keys.len() == 5 {
            return HighCard;
        }
        return Unknown;
    }

    fn compare_hands(hand_vec_a: &Vec<Card>, hand_vec_b: &Vec<Card>) -> Ordering {
        return if hand_vec_a[0].strength == hand_vec_b[0].strength {
            Hand::compare_hands(&hand_vec_a[1..].to_owned(), &hand_vec_b[1..].to_owned())
        } else if hand_vec_a[0].strength > hand_vec_b[0].strength {
            return Ordering::Greater
        } else if hand_vec_a[0].strength < hand_vec_b[0].strength {
            return Ordering::Less
        } else {
            return Ordering::Equal
        }
    }

    fn is_higher_ranked(&self, other: &Hand) -> Ordering {
        if self.hand_type == other.hand_type {
            return Hand::compare_hands(&self.hand_vec, &other.hand_vec);
        }
        return match self.hand_type {
            FiveOfAKind => { Ordering::Greater }
            FourOfAKind => {
                if !(other.hand_type == FiveOfAKind) {
                    return Ordering::Greater
                }
                return Ordering::Less
            }
            FullHouse => {
                if !(other.hand_type == FourOfAKind
                    || other.hand_type == FiveOfAKind) {
                    return Ordering::Greater
                }
                return Ordering::Less
            }
            ThreeOfAKind => {
                if !(other.hand_type == FourOfAKind
                    || other.hand_type == FiveOfAKind
                    || other.hand_type == FullHouse) {
                    return Ordering::Greater
                }
                return Ordering::Less
            }
            TwoPair => {
                if !(other.hand_type == FourOfAKind
                    || other.hand_type == FiveOfAKind
                    || other.hand_type == FullHouse
                    || other.hand_type == ThreeOfAKind) {
                    return Ordering::Greater
                }
                return Ordering::Less
            }
            OnePair => {
                if !(other.hand_type == FourOfAKind
                    || other.hand_type == FiveOfAKind
                    || other.hand_type == FullHouse
                    || other.hand_type == ThreeOfAKind
                    || other.hand_type == TwoPair) {
                    return Ordering::Greater
                }
                return Ordering::Less
            }
            HighCard => {
                if !(other.hand_type == FourOfAKind
                    || other.hand_type == FiveOfAKind
                    || other.hand_type == FullHouse
                    || other.hand_type == ThreeOfAKind
                    || other.hand_type == TwoPair
                    || other.hand_type == OnePair) {
                    return Ordering::Greater
                }
                return Ordering::Less
            }
            Unknown => { Ordering::Greater }
        };
    }
}

fn rank_hands(mut hands: &mut Vec<Hand>) {
    hands.sort_by(|a,b| a.is_higher_ranked(b));
}

struct Game {
    hand: Hand,
    bid: i64
}

fn day7_part2(path: &PathBuf) -> u32 {
    let mut sum: u32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {

        }
    }
    println!("{}", sum);
    return sum;
}

fn day7_part1(path: &PathBuf) -> i64 {
    let mut hands: Vec<Hand> = vec![];
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let split: Vec<&str> = line
                .as_ref()
                .unwrap()
                .split(" ")
                .collect::<Vec<&str>>();
            let hand_string: String = split.get(0).unwrap().clone().parse().unwrap();
            let bid: i64 = split.get(1).unwrap().clone().parse().unwrap();
            let hand: Hand = Hand::new(hand_string, bid);
            hands.push(hand);
        }
    }
    rank_hands(&mut hands);
    let mut winnings: i64 = 0;
    let mut rank: i64 = 1;
    hands.iter().for_each(|hand| {
        winnings = winnings + rank * hand.bid;
        rank += 1;
    });
    println!("{}", winnings);
    return winnings;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use crate::day7::{Card, day7_part1, Hand, rank_hands, Type};
    use crate::day7::Type::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

    #[test]
    fn test_ranks_test_input_hands_correctly() {
        let mut hands: Vec<Hand> = vec![
            Hand::new(String::from("32T3K"), 765),
            Hand::new(String::from("T55J5"), 684),
            Hand::new(String::from("KK677"), 28),
            Hand::new(String::from("KTJJT"), 220),
            Hand::new(String::from("QQQJA"), 483)
        ];
        rank_hands(&mut hands);
        let expected_ranked_hands: Vec<Hand> = vec![
            Hand::new(String::from("32T3K"), 765),
            Hand::new(String::from("KTJJT"), 220),
            Hand::new(String::from("KK677"), 28),
            Hand::new(String::from("T55J5"), 684),
            Hand::new(String::from("QQQJA"), 483),
        ];
        assert_eq!(&hands,&expected_ranked_hands)
    }

    #[test]
    fn test_ranks_hands_of_same_type_correctly() {
        let mut hands: Vec<Hand> = vec![
            Hand::new(String::from("AAAAA"), 111),
            Hand::new(String::from("KKKKK"), 222)
        ];
        rank_hands(&mut hands);
        let expected_ranked_hands: Vec<Hand> = vec![
            Hand::new(String::from("KKKKK"), 222),
            Hand::new(String::from("AAAAA"), 111)
        ];
        assert_eq!(&hands,&expected_ranked_hands)
    }

    #[test]
    fn test_ranks_hands_of_same_type_correctly_already_sorted() {
        let mut hands: Vec<Hand> = vec![
            Hand::new(String::from("AAAAA"), 111),
            Hand::new(String::from("KKKKK"), 222)
        ];
        rank_hands(&mut hands);
        let expected_ranked_hands: Vec<Hand> = vec![
            Hand::new(String::from("KKKKK"), 222),
            Hand::new(String::from("AAAAA"), 111)
        ];
        assert_eq!(&hands,&expected_ranked_hands)
    }

    #[test]
    fn test_ranks_hands_of_same_type_is_sort_sensitive() {
        let mut hands: Vec<Hand> = vec![
            Hand::new(String::from("AAAAA"), 111),
            Hand::new(String::from("KKKKK"), 222)
        ];
        rank_hands(&mut hands);
        let expected_ranked_hands: Vec<Hand> = vec![
            Hand::new(String::from("AAAAA"), 111),
            Hand::new(String::from("KKKKK"), 222)
        ];
        assert_ne!(&hands,&expected_ranked_hands)
    }

    #[test]
    fn test_build_hand_from_string_five_of_a_kind() {
        let mut cards: HashMap<Card, i32> = HashMap::new();
        cards.insert(Card { label: String::from("3"), strength: 3, }, 5);
        let expected_hand: Hand = Hand {
            cards,
            hand_vec: vec![],
            hand_type: Type::FiveOfAKind,
            bid: 111,
        };
        let hand: Hand = Hand::new(String::from("33333"), 111);
        assert_eq!(hand, expected_hand);
    }

    #[test]
    fn test_build_hand_from_string_two_pair() {
        let mut cards: HashMap<Card, i32> = HashMap::new();
        cards.insert(Card { label: String::from("3"), strength: 3, }, 2);
        cards.insert(Card { label: String::from("2"), strength: 2, }, 2);
        cards.insert(Card { label: String::from("T"), strength: 1, }, 1);
        let expected_hand: Hand = Hand {
            cards,
            hand_vec: vec![],
            hand_type: TwoPair,
            bid: 111
        };
        let hand: Hand = Hand::new(String::from("12323"), 111);
        assert_eq!(hand, expected_hand);
    }

    #[test]
    fn test_determines_type_for_five_of_a_kind_correctly() {
        let mut cards: HashMap<Card, i32> = HashMap::new();
        cards.insert(Card { label: String::from("3"), strength: 0, }, 5);
        let hand_type: Type = Hand::determine_type_for_cards(&cards);
        assert_eq!(hand_type, FiveOfAKind);
    }

    #[test]
    fn test_determines_type_for_four_of_a_kind_correctly() {
        let mut cards: HashMap<Card, i32> = HashMap::new();
        cards.insert(Card { label: String::from("3"), strength: 0, }, 4);
        cards.insert(Card { label: String::from("2"), strength: 0, }, 1);
        let hand_type: Type = Hand::determine_type_for_cards(&cards);
        assert_eq!(hand_type, FourOfAKind);
    }

    #[test]
    fn test_determines_type_for_full_house_correctly() {
        let mut cards: HashMap<Card, i32> = HashMap::new();
        cards.insert(Card { label: String::from("3"), strength: 0, }, 3);
        cards.insert(Card { label: String::from("2"), strength: 0, }, 2);
        let hand_type: Type = Hand::determine_type_for_cards(&cards);
        assert_eq!(hand_type, FullHouse);
    }

    #[test]
    fn test_determines_type_for_three_of_a_kind_correctly() {
        let mut cards: HashMap<Card, i32> = HashMap::new();
        cards.insert(Card { label: String::from("3"), strength: 0, }, 3);
        cards.insert(Card { label: String::from("2"), strength: 0, }, 1);
        cards.insert(Card { label: String::from("T"), strength: 0, }, 1);
        let hand_type: Type = Hand::determine_type_for_cards(&cards);
        assert_eq!(hand_type, ThreeOfAKind);
    }

    #[test]
    fn test_determines_type_for_two_pair_correctly() {
        let mut cards: HashMap<Card, i32> = HashMap::new();
        cards.insert(Card { label: String::from("3"), strength: 0, }, 2);
        cards.insert(Card { label: String::from("2"), strength: 0, }, 2);
        cards.insert(Card { label: String::from("T"), strength: 0, }, 1);
        let hand_type: Type = Hand::determine_type_for_cards(&cards);
        assert_eq!(hand_type, TwoPair);
    }

    #[test]
    fn test_determines_type_for_one_pair_correctly() {
        let mut cards: HashMap<Card, i32> = HashMap::new();
        cards.insert(Card { label: String::from("3"), strength: 0, }, 2);
        cards.insert(Card { label: String::from("2"), strength: 0, }, 1);
        cards.insert(Card { label: String::from("T"), strength: 0, }, 1);
        cards.insert(Card { label: String::from("K"), strength: 0, }, 1);
        let hand_type: Type = Hand::determine_type_for_cards(&cards);
        assert_eq!(hand_type, OnePair);
    }

    #[test]
    fn test_determines_type_for_high_card_correctly() {
        let mut cards: HashMap<Card, i32> = HashMap::new();
        cards.insert(Card { label: String::from("3"), strength: 0, }, 1);
        cards.insert(Card { label: String::from("2"), strength: 0, }, 1);
        cards.insert(Card { label: String::from("T"), strength: 0, }, 1);
        cards.insert(Card { label: String::from("K"), strength: 0, }, 1);
        cards.insert(Card { label: String::from("8"), strength: 0, }, 1);
        let hand_type: Type = Hand::determine_type_for_cards(&cards);
        assert_eq!(hand_type, HighCard);
    }

    #[test]
    fn test_get_strength_correctly() {
        let card1: Card = Card {
            label: String::from("A"),
            strength: 0,
        };
        assert_eq!(card1.get_strength(), 14);
        let card1: Card = Card {
            label: String::from("8"),
            strength: 0,
        };
        assert_eq!(card1.get_strength(), 8);
    }

    #[test]
    fn test_part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day7/test/input.txt");
        assert_eq!(day7_part1(&d), 6440);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day7/input.txt");
        assert_eq!(day7_part1(&d), 250370104);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day7/test/input1.txt");
        //assert_eq!(day7_part2(&d), 281);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day7/input.txt");
        //assert_eq!(day7_part2(&d), 281);
    }

}