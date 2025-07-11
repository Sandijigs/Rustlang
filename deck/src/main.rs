use rand :: {thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
        // let mut cards = self.cards.clone();
        // cards.shuffle(&mut rng);
        // self.cards = cards;
    }
    fn deal(&mut self,num_cards:usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let cards = deck.deal(3);
    println!("Heres your hand: {:#?}", cards);
    println!("Heres your deck: {:#?}", deck);
}

// fn change(s: &mut String) {
//     s.push_str(", world!");
// }

// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1); // pass a reference

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len() // just reading, not taking ownership
// }

// fn main(){
//     let mut s1= String::from("web3bridge Rust");
//     let len = describe(&s1);

//     println!("the length of '{}' is {}", s1, len);
//     append_exclamation(&mut s1);
//     println!("Modified:{}", s1);
// }

// fn describe(s: &String) -> usize {
//    s.len()
// }

// fn append_exclamation(s: &mut String) {
//     s.push_str("! duh!");
// }
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     &s[..]
// }

// fn main() {
//     let text = String::from("web3bridge rocks");
//     let word = first_word(&text);

//     println!("First word: {}", word); // prints "web3bridge"
// }