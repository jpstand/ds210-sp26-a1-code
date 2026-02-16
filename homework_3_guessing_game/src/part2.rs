use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        let current_guess = (min + max) / 2;
        let x = player.ask_to_compare(current_guess);
        if x == 0 {
            return current_guess;
        } 
        else if x == 1 {
            return Self::guess_the_number(player, (min + max) / 2, max);
        }
        else if x == -1 {
            return Self::guess_the_number(player, min, (min + max) / 2);
        }
        return 1 // it gets mad unless I leave this here
    }
}
