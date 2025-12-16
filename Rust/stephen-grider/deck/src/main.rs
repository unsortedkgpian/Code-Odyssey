#[derive(Debug)]
struct Deck {// Name of Struct always capitalized
    cards:Vec<String>,
}

// Derive is altributes tells the rust to impl some trait which is Debug

fn main() {

    // List of 'suits' - 'Spades', 'Clubs', 'Heart', 'Diamonds'
    // List of values - 'ace', 'two', 'three', 'four', 
    // Generating all type of list 
    let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

    let mut cards= vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    let deck:Deck = Deck { cards };
    let _deck1 : Deck = Deck { cards : Vec::new() };

    //println!("Hello, world!");
    println!("Heres your deck {:?}", deck);
    println!("Heres your deck {:#?}", deck);
}
