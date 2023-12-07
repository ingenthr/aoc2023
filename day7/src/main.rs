use std::io;
use std::fs;
use std::cmp::Ordering;

fn main() -> Result<(), io::Error> {
    let hands_in = fs::read_to_string("input.txt")?;
    let mut hands = Vec::new();

    for hand_in in hands_in.lines() {
        let hand_cards = hand_in.split_whitespace().nth(0).expect("Expected a string followed by a number");
        let hand_bid = hand_in.split_whitespace().nth(1).expect("Expected a string followed by a number").parse::<u32>().expect("Expected an unsigned 32 bit integer as a string.");
        
        hands.push(HandBid::new(hand_cards.to_string(), hand_bid));
    }

    println!("File had {} hands.", hands.len());

    Ok(())

}

#[derive(Eq)]
pub struct HandBid {
    hand: String, // TODO: convert to &str perhaps, but need to understand owned/borrowing implications
    bid: u32,
}

impl HandBid {
    pub fn new(hand: String, bid: u32) -> Self {
        Self {
            hand,
            bid
        }
    }
}

impl PartialOrd for HandBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandBid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bid.cmp(&other.bid) // TODO: totally wrong, but want to implement wrong comparison first
    }
}

impl PartialEq for HandBid {
    fn eq(&self, other: &Self) -> bool {
        self.bid == other.bid
    }
}

#[test]
fn test_new_handbid() {
    let test = HandBid::new("32T3K".to_string(), 765);
}

#[test]
fn compare_handbids() {
    let one = HandBid::new("32T3K".to_string(), 765);
    let two = HandBid::new("T55J5".to_string(), 684);

    assert!(two>one);

}