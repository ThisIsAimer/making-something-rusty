use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    //implimenting deck
    fn new() -> Self {
        //vec![] for vectors and []for arrays
        //vec! allows things to grow or shrink in size
        let suits = ["💖", "🍀", "💎", "🌿"];
        let values = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ];

        let mut cards = vec![];

        for suite in suits {
            for value in values {
                let card = format!("{value} of {suite}");
                cards.push(card);
            }
        }

        Deck { cards } //or Vec::new() 
        //implicit function
    }

    fn shuffle(&mut self) {
        let mut random = rng();
        //self.cards.shuffle(&mut random);
        self.cards.shuffle(&mut random);
        //rand::seq::SliceRandom::shuffle(&mut self.cards[..], &mut random);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
        //what splitoff does is it takes devides the items by the number we devide and
        //gives us the last items.
    }
}

fn main() {
    let mut deck = Deck::new(); //or Vec::new()
    deck.shuffle();

    //probably need to handle error handling
    let cards = deck.deal(3);

    println!("here is your deck {cards:#?}");
}
