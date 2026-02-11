use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part1 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part1 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        for guess_val in min..max {
            if player.ask_if_equal(guess_val) {
                return guess_val;
            }
        } return 222
    }
}
