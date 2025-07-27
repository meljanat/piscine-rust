use rand::prelude::*;

pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Suit {
    pub fn random() -> Suit {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let value: u8 = rng.gen_range(0..4);
        Suit::translate(value)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            0 => Suit::Hearts,
            1 => Suit::Diamonds,
            2 => Suit::Clubs,
            3 => Suit::Spades,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let value: u8 = rng.gen_range(0..13);
        Rank::translate(value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            12 => Rank::Ace,
        }
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    return (card.suit == Suit::Spades && card.rank == Rank::Ace);
}