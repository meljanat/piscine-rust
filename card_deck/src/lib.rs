use rand::prelude::*;

#[derive(PartialEq, Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(PartialEq, Debug)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let value: u8 = rng.gen_range(0..=3);
        Suit::translate(value)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Club,
            3 => Suit::Spade,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let value: u8 = rng.gen_range(1..=13);
        Rank::translate(value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    return card.suit == Suit::Spade && card.rank == Rank::Ace;
}
