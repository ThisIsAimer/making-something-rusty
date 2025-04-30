#[derive(Debug)]
struct Deck{
    cards : Vec<String>,
}

fn main() {

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



    let deck = Deck{cards}; //or Vec::new()

    println!("here is your deck {deck:#?}");
}