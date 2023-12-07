use std::{cmp::Ordering,  iter::zip};

fn card_to_power(c: &char) -> usize {
    match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => unreachable!("Character {} hs no rule", c),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn get_type_of_hand(hand: &str) -> HandType {
        let mut found_cards: Vec<u8> = vec![0; 13];
        for c in hand.chars() {
            found_cards[card_to_power(&c)] += 1;
        }
        found_cards.sort();
        let nums_as_string: Vec<String> = found_cards
            .iter()
            .filter(|card_count| **card_count != 0)
            .map(|card_count| card_count.to_string())
            .collect();
        let mut type_string = String::from("");
        for num_as_string in nums_as_string {
            type_string.push_str(&num_as_string);
        }

        match type_string.as_str() {
            "11111" => Self::HighCard,
            "1112" => Self::OnePair,
            "122" => Self::TwoPair,
            "113" => Self::ThreeOfAKind,
            "23" => Self::FullHouse,
            "14" => Self::FourOfAKind,
            "5" => Self::FiveOfAKind,
            _ => unreachable!("Found nonexistent type {}", type_string),
        }
    }

    fn get_type_of_hand_with_joker(hand: &str) -> HandType {
        let mut found_cards: Vec<u8> = vec![0; 13];
        let mut j_count: u8 = 0;
        for c in hand.chars() {
            if c == 'J' {
                j_count += 1;
                continue;
            }
            found_cards[card_to_power(&c)] += 1;
        }

        let max_index = found_cards
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();
        found_cards[max_index] += j_count;

        found_cards.sort();
        let nums_as_string: Vec<String> = found_cards
            .iter()
            .filter(|card_count| **card_count != 0)
            .map(|card_count| card_count.to_string())
            .collect();
        let mut type_string = String::from("");
        for num_as_string in nums_as_string {
            type_string.push_str(&num_as_string);
        }

        match type_string.as_str() {
            "11111" => Self::HighCard,
            "1112" => Self::OnePair,
            "122" => Self::TwoPair,
            "113" => Self::ThreeOfAKind,
            "23" => Self::FullHouse,
            "14" => Self::FourOfAKind,
            "5" => Self::FiveOfAKind,
            _ => unreachable!("Found nonexistent type {}", type_string),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    hand: String,
    pub bid: u64,
    hand_type: HandType,
    with_joker: bool,
}

impl Hand {
    pub fn new(hand: String, bid: u64) -> Hand {
        let hand_type = HandType::get_type_of_hand(&hand);
        Hand {
            hand,
            bid,
            hand_type,
            with_joker: false,
        }
    }

    pub fn new_with_joker(hand: String, bid: u64) -> Hand {
        let hand_type = HandType::get_type_of_hand_with_joker(&hand);
        Hand {
            hand,
            bid,
            hand_type,
            with_joker: true,
        }
    }

    fn compare_hand_power(&self, other: &Hand) -> Ordering {
        for (card, other_card) in zip(self.hand.chars(), other.hand.chars()) {
            let cmp;
            if self.with_joker {
                if card == 'J' && other_card == 'J' {
                    continue;
                } else if card == 'J' {
                    return Ordering::Less;
                } else if other_card == 'J' {
                    return Ordering::Greater;
                }
                cmp = card_to_power(&card).cmp(&card_to_power(&other_card));
            } else {
                cmp = card_to_power(&card).cmp(&card_to_power(&other_card));
            }
            return match cmp {
                Ordering::Equal => continue,
                other => other,
            };
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(Ordering::Equal) => Some(self.compare_hand_power(other)),
            other => other,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.compare_hand_power(other),
            other => other,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hand_type_order() {
        assert!(HandType::HighCard < HandType::OnePair);
        assert!(HandType::HighCard < HandType::TwoPair);
        assert!(HandType::FullHouse < HandType::FourOfAKind);
        assert!(HandType::HighCard < HandType::FiveOfAKind);
        assert!(HandType::FourOfAKind < HandType::FiveOfAKind);
        assert!(HandType::TwoPair < HandType::ThreeOfAKind);
    }
}
