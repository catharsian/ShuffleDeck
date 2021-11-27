use rand::{
    self,
    Rng,
};

use std::path::Path;
use std::cmp::{ 
    self, 
    Ordering,
};
use opengl_graphics::{
    Texture,
    TextureSettings,
};
use graphics::{
    Image,
    rectangle::rectangle_by_corners,
};

pub mod animations;
pub use animations:: {
    Anims,
    Anim,
    DECK_SIZE,
    *
};

#[derive(Clone)]
#[derive(Copy)]


pub enum Suit<'a>{
    Diamonds(&'a Texture),
    Clubs(&'a Texture),
    Hearts(&'a Texture),
    Spades(&'a Texture),
    None,
}

#[derive(Clone)]
#[derive(Copy)]
pub enum Rank<'a>{
    Ace(&'a Texture),
    King(&'a Texture),
    Queen(&'a Texture),
    Joker(&'a Texture),
    N10(&'a Texture),
    N9(&'a Texture),
    N8(&'a Texture),
    N7(&'a Texture),
    N6(&'a Texture),
    None,
}




#[derive(Clone, Copy)]
pub struct Deck<'a> {
    rankings:[Rank<'a>; DECK_SIZE],
    suits: [Suit<'a>; DECK_SIZE],
    pub poses: Anims,
}



const IMAGE_WIDTH: f32 = 100.0;
const IMAGE_HEIGHT: f32= 170.0;

impl<'a> Deck<'a> {
    pub fn get_card<'b>(&'b self, n: usize) -> (Rank<'a>, Suit<'a>, Anim){
        (self.rankings[n], self.suits[n], self.poses.get_anim(n))
    }
    pub fn print_card(&'a mut self, n: usize){
        let card = self.get_card(n);

        match card.1 {
            Suit::Clubs(_) => print!("card is of club suit "),
            Suit::Diamonds(_) => print!("card is of diamonds suit "),
            Suit::Hearts(_) => print!("card is of hearts suit "),
            Suit::Spades(_) => print!("card is of spades suit "),
            Suit::None => println!("card is of none suit "),
        }
        match card.0 {
            Rank::Ace(_) => println!("and is Ace!!!"),
            Rank::King(_) => println!("and is a king."),
            Rank::Queen(_) => println!("and is a queen"),
            Rank::Joker(_) => println!("and is a joker."),
            Rank::N10(_) => println!("and is number 10"),
            Rank::N9(_) => println!("and is number 9"),
            Rank::N8(_) => println!("and is number 8"),
            Rank::N7(_) => println!("and is number 7"),
            Rank::N6(_) => println!("and is of the lowest rank."),
            Rank::None => println!("and is of none rank."),
        }
    }
    pub fn create_deck<'b>(textures: &'b [Texture; 13]) -> Deck<'b>{
        let mut new_deck = Deck{
            rankings: [Rank::None; 36],
            suits: [Suit::None; 36],
            poses: Anims::new(centre_x - 50.0, centre_y - 85.0),
        };
        let Ranks: [Rank; 9] = [
            Rank::Ace(&textures[0]), 
            Rank::King(&textures[1]),
            Rank::Queen(&textures[2]), 
            Rank::Joker(&textures[3]), 
            Rank::N10(&textures[4]), 
            Rank::N9(&textures[5]),
            Rank::N8(&textures[6]),
            Rank::N7(&textures[7]),
            Rank::N6(&textures[8])
        ];
        let Suits: [Suit; 4] = [
            Suit::Diamonds(&textures[9]),
            Suit::Clubs(&textures[10]),
            Suit::Hearts(&textures[11]),
            Suit::Spades(&textures[12])
        ];
        for (i, s) in Suits.iter().enumerate(){
            for (j, r) in Ranks.iter().enumerate(){
                new_deck.rankings[i * 9 + j] = *r;
                new_deck.suits[i * 9 + j] = *s;
            }
        }
        new_deck
    }
    pub fn swap(self, first: usize, second: usize) -> Self
    {

        let mut shit = self.clone();
        let first_tup = shit.get_card(first);
        let second_tup = shit.get_card(second);
        shit.rankings[first] = second_tup.0;
        shit.suits[first] = second_tup.1;
        shit.poses.apply_anim(first, second_tup.2);
        shit.rankings[second] = first_tup.0;
        shit.suits[second] = first_tup.1;
        shit.poses.apply_anim(second, first_tup.2);
        shit
    }
    pub fn shuffle(mut self) -> Self
    {
        let mut rng = rand::thread_rng();
        let g = rng.gen_range(100..=300);
        for i in 0..g
        {
            let first = rng.gen_range(0..36);
            let second = rng.gen_range(0..36);
            self = self.swap(first, second);
        }
        self
    }
    pub fn iter(&self) -> DeckIterator {
        DeckIterator {
            n: 0,
            owner: self,
        }
    }
}

pub struct DeckIterator<'a> {
    n: usize,
    owner: &'a Deck<'a>
}

impl<'a> Iterator for DeckIterator<'a> {
    type Item = (Rank<'a>, Suit<'a>, Anim);
    
    fn next(&mut self) -> Option<Self::Item>{
        if self.n < 36 {
            let ret = Some(self.owner.get_card(self.n));
            self.n+=1;
            ret
        }
        else {
            None
        }
    }
}