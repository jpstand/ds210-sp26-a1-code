use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::{self, Player};

use crate::solution;

// Your solution solution.
pub struct SolutionAgent {}


// Put your solution here.
impl Agent for SolutionAgent {
    
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        //need this to spread out like a tree
        //check the first spot and the possible moves then moves on to the next 
        //done under 2 secs
        // if first move and middle is available take it
        let moves = board.moves(); // available moves
        let score = board.score();
        
        if board.game_over() {
            return (score,0,0);
        }
         
        if moves.len() == 9{ // if no moves are made. go middle
            return (score, 1, 1);
        }

        let mut best_score;
        if matches!(player,Player::X){
            best_score = -2;
        }else {
            best_score = 2;
        }
        let mut best_move = moves[0].clone();
       
        for m in moves.clone(){// for each move

            board.apply_move(m, player);
            let result = SolutionAgent::solve(board, player.flip(), _time_limit);
            let score = result.0;
            board.undo_move(m, player);

            if matches!(player, Player::X) {
                // X wants the highest score
                if score > best_score {
                    best_score = score;
                    best_move = m;
                }
            } else {
                // O wants the lowest score
                if score < best_score {
                    best_score = score;
                    best_move = m;
                }
            }
        }
    return (best_score,best_move.0,best_move.1);
    }
    // If you want to make a recursive call to this solution, use
    // `SolutionAgent::solve(...)`
    //unimplemented!("Not yet implemented");
}