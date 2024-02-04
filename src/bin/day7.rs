use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
#[allow(dead_code)]
struct Hand {
    cards: String,
    bid: i32
}

#[derive(Debug)]
#[allow(dead_code)]
struct WeightedHand<'a> {
    hand: &'a Hand,
    htype: HType
}

fn build_hand(cards: &String, bid: i32) -> Hand {
    Hand {
        cards: cards.to_string(),
        bid
    }
}

fn build_weighted_hand(hand: &Hand, htype: HType) -> WeightedHand {
    WeightedHand {
        hand,
        htype
    }
}

#[derive(Debug)]
enum HType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}

fn findkind(hand: &String) -> HType {
    let mut map: HashMap<char, i32> = HashMap::new();
    for card in hand.chars() {
        map.entry(card).and_modify(|x| *x += 1).or_insert(1);
    }
    match map.len() {
        1 => HType::FiveKind,
        2 => {
            if map.values().any(|&count| count == 4) {
                HType::FourKind
            } else {
                HType::FullHouse
            }
        }
        3 => {
            if map.values().any(|&count| count == 3) {
                HType::ThreeKind
            } else {
                HType::TwoPair
            }
        }
        4 => HType::OnePair,
        5 => HType::HighCard,
        _ => unreachable!("Impossible Hand: {:?}", hand)
    }
}

fn main() {

    let file_path: String = String::from("src/bin/day7.txt");
    let file: String = fs::read_to_string(file_path).unwrap();
    let input: Vec<Hand> = file
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .chunks(2) 
        .map(|chunk| build_hand(&chunk[0].to_string(), chunk[1].parse().unwrap()))
        .collect();

    println!("{:?}", input);

    for hand in &input {
        //println!("{:?} is of type: {:?}", &hand.cards, findkind(&hand.cards));
        let x = build_weighted_hand(hand, findkind(&hand.cards));
        println!("{:?}", x);
    }

}
