//Serialize : 저장하기 / Deserialize : 불러오기
use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rank {
    Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card{rank, suit}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deck{
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();

        let suits = [Suit::Club,Suit::Diamond,Suit::Heart,Suit::Spade];
        let ranks = [Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five, 
        Rank::Six, Rank::Seven,Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King];
        
        for &suit in &suits {
            for &rank in &ranks {
                cards.push(Card::new(rank, suit));
            }
        }

        Self { cards }
    
    }

    pub fn shuffle(&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card>{
        self.cards.pop()
    }

    pub fn len(&self) -> usize{
        self.cards.len()
    }
    

}