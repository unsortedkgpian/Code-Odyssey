use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {// Name of Struct always capitalized
    cards:Vec<String>,
}

impl Deck {
    //fn new() -> Deck{
      
    fn new() -> Self{
        let suits = ["Hearts", "Diamonds", "Spades", "Clubs"];
        let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

        let mut cards = Vec::new();

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        let deck  = Deck { cards};
        return deck;
    }

    fn shuffle(&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
        // -> don't understand 
        // Shuffle comes from rand::seq::SliceRandom; -> randomaize the things 
    }

    fn deal(&mut self, num_cards:usize) -> Vec<String> {
        self.cards.split_off(num_cards)
    }
}

// Derive is altributes tells the rust to impl some trait which is Debug

fn main() {

    // List of 'suits' - 'Spades', 'Clubs', 'Heart', 'Diamonds'
    // List of values - 'ace', 'two', 'three', 'four', 
    // Generating all type of list 
    //let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    //let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

    //let mut cards= vec![];

    //for suit in suits {
        //for value in values {
            //let card = format!("{} of {}", value, suit);
            //cards.push(card);
        //}
    //}

    //let deck:Deck = Deck { cards };
    //let _deck1 : Deck = Deck { cards : Vec::new() };

    // New Goal 
    let mut deck = Deck::new();

    //println!("Hello, world!");
    println!("Heres your deck {:?}", deck);
    println!("Heres your unshuffle deck {:#?}", deck);
    //deck.shuffle();
    //Todo -> Error handaling 
    deck.deal(2);
    println!("Heres your deck {:#?}", deck);
}
