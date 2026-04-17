use std::cmp::max;
use std::thread::current;

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

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
        // let moves = board.moves(); // available moves
        // let score = board.score(); // this is in the helper function
        let max_depth = 4; // max depth for depth tracking function 4 works to pass all tests but one on my computer
        let current_depth = 0; // starting depth
        
        // if board.game_over() {// base case
        //     return (score,0,0);
        // } // added this base case to the helper function
         
        // if moves.len() == 9{ // if no moves are made. go middle
        //     return (score, 1, 1);
        // } // we remove this because it is no longer fixed to 3x3 and the center could have a wall


        return depth_tracking(board, &current_depth, &max_depth, player, _time_limit);
    }
}

fn depth_tracking(board: &mut Board,current_depth: &i32, max_depth: &i32, player: Player, _time_limit: u64) -> (i32, usize, usize) {
    if current_depth == max_depth {//if we've reached the max depth, return the board state
            return (heuristic(board), 0, 0);
        }
    let moves = board.moves(); // available moves
    let new_current = current_depth + 1; //we update the current depth

    if board.game_over() {// base case
            return (board.score(),0,0);
        }
    
    let mut best_score;
        if matches!(player,Player::X){
            best_score = -5; // score could be |2| when finishing so just setting this to something it couldn't be 
        }else {
            best_score = 5;
        }
        let mut best_move = moves[0].clone();
    for m in moves.clone(){// for each move

            board.apply_move(m, player);
            let result = depth_tracking(board, &new_current, max_depth, player.flip(), _time_limit); // this is the same as before execpt now we just keep passing in the depths to keep track
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
        return (best_score,best_move.0,best_move.1); // same as before
    }

fn heuristic(board: &mut Board) -> i32 {
    return board.score(); // placeholder this needs adjusting. We pass all tests but one right now
}