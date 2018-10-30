extern crate rs_poker;
use rs_poker::core::Card;
use rs_poker::core::Deck;
use rs_poker::core::Flattenable;
use rs_poker::core::Hand;
use rs_poker::holdem::MonteCarloGame;

extern crate elapsed;
use elapsed::measure_time;

fn main() {
    let mut deck = Deck::default().flatten();
    deck.shuffle();
    assert_eq!(52, deck.len());
    let mut first_cards: Vec<Card> = Vec::new();
    for _ in (0..6) {
        first_cards.push(deck.deal().unwrap())
    }
    println!("first_cards = {:?}", first_cards);

    let mut second_cards: Vec<Card> = Vec::new();
    for _ in (0..6) {
        second_cards.push(deck.deal().unwrap())
    }
    println!("second_cards = {:?}", second_cards);
    assert_eq!(40, deck.len());

    let mut hole_cards: Vec<Hand> = Vec::new();
    for (first_card, second_card) in first_cards.iter().zip(second_cards.iter()) {
        hole_cards.push(Hand::new_with_cards(vec![*first_card, *second_card]))
    }
    println!("Hands = {:?}", hole_cards);

    let mut g = MonteCarloGame::new_with_hands(hole_cards).expect("Should be able to create a game.");
    let mut wins: [u64; 6] = [0, 0, 0, 0, 0, 0];

    let elapsed = measure_time(|| {
        for _ in 0..1000000 {
            let r = g.simulate().expect("There should be one best rank.");
            g.reset();
            wins[r.0] += 1
        }
        println!("Wins = {:?}", wins);
    });
    println!("elapsed = {:?}", elapsed);
}