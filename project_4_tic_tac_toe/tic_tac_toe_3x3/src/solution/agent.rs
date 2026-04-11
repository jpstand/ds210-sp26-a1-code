use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

use crate::solution;

// Your solution solution.
pub struct SolutionAgent {}


// Put your solution here.
impl Agent for SolutionAgent {
    
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
    
        let moves = board.moves(); // available moves
        //make a move
        let move_here = moves[0];
        let (score,row,col) = SolutionAgent::solve(board, player, _time_limit );
        let move_attempt = (row,col);


        // if first move and middle is available take it
        if moves.contains(&(1 as usize,1 as usize)) && board.moves().len() == 9{ 
            return (score,1 as usize,1 as usize );
        }

        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        unimplemented!("Not yet implemented")
    }

}
