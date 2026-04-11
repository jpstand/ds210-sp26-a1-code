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

        fn check_win (player:Player, score: i32) -> bool{
            if matches!(player,Player::X) && score > 0 { 
                return true;
            }
            else if matches!(player,Player::O) && score < 0 {
                return true;
            }
            return false; 

        }

        //need this to spread out like a tree
        //check the first spot and the possible moves then moves on to the next 
        //done under 2 secs
        // if first move and middle is available take it
        let moves = board.moves(); // available moves
        let score = board.score();
        let updated_board = board.get_cells();
        //checking for win condition 
        
        
        let mut best_move = moves[0];
        board.apply_move(best_move, player);
        let mut best_score = board.score();
        board.undo_move(best_move, player);
        if check_win(player,best_score){
                return (best_score, best_move.0, best_move.1);
            }
        
        //check for one move win condition
        for i in 1..moves.len() {
            let m = moves[i];
            board.apply_move(m, player);
            let score = board.score();

            //recursive check for win condition
            let return_me: (i32, usize, usize) = SolutionAgent::solve(board, player, _time_limit );
            board.undo_move(m, player);

            match player {
                Player::X => {
                    if score > best_score {
                        best_move = m;
                        best_score = score;
                    }
                }
                Player::O => {
                    if score < best_score {
                        best_move = m;
                        best_score = score;
                    }
                }
            }
            if check_win(player,return_me.0){
                return return_me;
            }
        }
        
        todo!("start logic implementation here");
        todo!("check corners");

        if moves.len() = 7{
        todo!("if oponinet is in a corner place peice in oposite corner");
        }



        if board.moves().len() == 9{ 
            return (score,1 as usize,1 as usize );
        }

        
        //otherwise put it in a corner
        //then put it in oposite corner
        //then put it in an 
        
       
        //make a move

        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        unimplemented!("Not yet implemented")
    }
    

}

/*if matches!(player, Player::O) && score <=0{
            if 
            
            return (score,row,col);
        }
        if matches!(player, Player::X) && score >=0{
            return (score,row,col);
        } 
        */
