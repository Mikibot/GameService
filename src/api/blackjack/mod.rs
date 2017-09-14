mod cards;
mod deck;
mod hand;
mod blackjack;
mod response;

pub use self::cards::Card;
pub use self::deck::Deck;
pub use self::hand::Hand;
pub use self::blackjack::BlackJack;
pub use self::blackjack::GameState;
pub use self::response::Response as BlackJackResponse;
//#[cfg(not(any(test, bench)))]
pub use models::BJSession as Session;
/*
#[cfg(any(test, bench))]
pub struct Session {
    pub player_id: i64,
    pub bet: Option<i64>, // None means it was claimed
    /*
    None - In progress
    true - Player Won
    false - Player Lost
    */
    pub win_loss: i64,
    pub status: Option<bool>,
    pub deck: Vec<String>, // Empty when game ends
    pub player_hand: Vec<String>, // Empty when game ends
    pub dealer_hand: Vec<String>, // Empty when game ends
    pub player_stay: bool, // False by default
    pub dealer_stay: bool, // False by default
    pub first_turn: bool, // True by default
}
*/
