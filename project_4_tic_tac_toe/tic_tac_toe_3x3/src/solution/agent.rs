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
        match player {
            Player::X => {
            }    
            Player::O => {
            }
        }

        let moves = board.moves(); // available moves
        let quicklook= [[0;3];3]; // make a quick look up 
        //check for win condition
        for i in board.get_cells(){ //row
            for j in i{//col
                match j {
                    Cell::X=>{
                        quicklook
                    }
                    Cell::Y=>{

                    }
                }

                }


            }
        


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
