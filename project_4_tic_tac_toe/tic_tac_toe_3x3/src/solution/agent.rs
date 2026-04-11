use std::time::Instant;

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
        let start_time = Instant::now();
        fn check_win (player:Player, score: i32) -> bool{
            if matches!(player,Player::X) && score > 0 { 
                return true;
            }
            else if matches!(player,Player::O) && score < 0 {
                return true;
            }
            return false; 

        }
        fn check_space(player:Player, location:(usize,usize))->bool {
        todo!("check location of O");

        }
        //need this to spread out like a tree
        //check the first spot and the possible moves then moves on to the next 
        //done under 2 secs
        // if first move and middle is available take it
        let moves = board.moves(); // available moves
        let score = board.score();
        let moves_len = moves.len();
        
         
        if board.moves().len() == 9{ // if no moves are made. go middle
            return (score,1 as usize,1 as usize );
        }
        //check for one move win condition
        if moves_len >= 3{ //check for win when possiable. if know to go against a good bot can switch 3 to 5 to save time 
            for m in moves{
                board.apply_move(m, player);
                let score = board.score();
                board.undo_move(m, player);
                if check_win(player,score){ // if player has won
                    return (score, m.0, m.1);
                }
            }
        }
        
        
       
       
        todo!("start logic implementation here");
        todo!("check corners");
        for m in moves{
            match player{
                Player::X=>{
                    if moves_len == 7{
                        todo!("implement logic");
                    }
                    else if moves_len == 5{
                        check_space(player,m);
                    }
                    else if moves_len == 3{
                        todo!("implement logic");
                    }
                }
        
                Player::O=>{
                    if moves_len == 8{
                    todo!("if oponinet is in a corner place peice in oposite corner");
                    }
                    else if moves_len == 6{
                        todo!("implement logic");
                    }
                    else if moves_len == 4{
                        todo!("implement logic");
                    }
                    else if moves_len == 2{
                        todo!("implement logic");
                    }
                }
        }
    }

        
        //otherwise put it in a corner
        //then put it in oposite corner
        //then put it in an 
        
       
        //make a move

        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        unimplemented!("Not yet implemented");
        let time_taken = start_time.elapsed().as_millis();
        println!("It took {}ms to make a move",time_taken);
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
