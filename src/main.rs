use rand:: {thread_rng, seq::SliceRandom};


#[derive(Debug)]
struct Deck{
    cards : Vec<String>,
}

impl Deck{ //implimenting deck
    fn new() -> Self {
            //vec![] for vectors and []for arrays
        //vec! allows things to grow or shrink in size
        let suits = ["💖","🍀","💎","🌿"];
        let values = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"];

        let mut cards = vec![];

        for suite in suits{
            for value in values{
                let card = format!("{value} of {suite}");
                cards.push(card);
            }

        }



         
        
        Deck{cards} //or Vec::new() 
        //implicit function

    }
    fn shuffl(&mut self){
        let mut random = thread_rng();
        self.cards.shuffle(&mut random);
    }
}

fn main() {

    let mut deck = Deck::new(); //or Vec::new()

    deck.shuffl();

    println!("here is your deck {deck:#?}");
}