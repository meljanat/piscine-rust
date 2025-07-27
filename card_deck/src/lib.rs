#[derive(PartialEq, Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
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
        use rand::prelude::*;
        let mut rng = rand::rng();
        let value: u8 = rng.random_range(0..4);
        Suit::translate(value)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            0 => Suit::Hearts,
            1 => Suit::Diamonds,
            2 => Suit::Clubs,
            3 => Suit::Spades,
            _ => unreachable!(),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        use rand::prelude::*;
        let mut rng = rand::rng();
        let value: u8 = rng.random_range(1..14);
        Rank::translate(value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    return card.suit == Suit::Spades && card.rank == Rank::Ace;
}
