use std::fs;

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i32
}

fn build_hand(cards: &String, bid: i32) -> Hand {
    Hand {
        cards: cards.to_string(),
        bid
    }
}

fn main() {
    let file_path: String = String::from("src/bin/day7.txt");
    let file: String = fs::read_to_string(file_path).unwrap();
    let result: Vec<(String, i32)> = file
        .lines()
        .map(|line| {
        let mut parts = line.split_whitespace();
        let first = parts.next().unwrap_or_default().to_string();
        let second = parts.next().unwrap_or_default().parse().unwrap_or_default();
        (first, second)
    })
    .collect();

    println!("{:?}", result);

    let mut hands: Vec<Hand> = Vec::new();

    for (x, y) in result {
        let temp = build_hand(&x, y);
        println!("{:?}", temp.cards);
        println!("{:?}", temp.bid);
        hands.push(temp);
    }

    println!("{:?}", hands);
}
