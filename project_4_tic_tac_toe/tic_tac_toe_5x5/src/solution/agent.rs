use std::time::{Duration, Instant};

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize) {

        // ^ this method will allow us to search as far as possible on any computer because we can fit our depth search acording to the time taken rather than a hard coded number.
        let start_time = Instant::now();
        let limit = Duration::from_millis((time_limit as f64 * 0.988999999) as u64);

        // max depth for depth tracking function 4 works to pass all tests but one on my computer 
        let max_depth = 26; // updated to rely on the system timer
        let current_depth = 0; // starting depth

        //implementing Min MaxAplpha/Beta Pruning - gets rid of any options that the opponiente would not allow you to win in
        let alpha = i32::MIN;
        let beta = i32::MAX;
         
        //maybe implement a check here that checks if we are winning, if we are spend the rest of the game making the other team lose. if not, draw the game.
        // let mut sabatoge = false;
        // let score = heuristic(board, true);
        
        // match player {
        //     Player::X=>{
        //         if score > 10000{ // not sure if this is the optomal #
        //             sabatoge = true;
        //         }
        //     }
        //     Player::O=>{
        //         if score < -10000{ // not sure if this is the optomal #
        //             sabatoge = true;
        //         }
        //     }
        // }
        let mut best_move: (i32, usize, usize) = (0, 2, 2); // go center
        for depth in 4..max_depth {
            if start_time.elapsed() >= limit { 
                break; 
            }
            // We pass the start_time and limit into our search // there was a bug here
            if let Some(result) = evaluate(board, current_depth, depth,  &player, start_time,limit, alpha, beta,(best_move.1,best_move.2)) {
                best_move = result;
            } else {// if there is no result break
                break; 
            }
        }
        //let result = depth_tracking(board, current_depth, max_depth, &player, &_time_limit, alpha, beta, moves, sabatoge);
        
        return best_move;
    }
}

fn order_moves(board: &mut Board, moves: &[(usize, usize)], player: &Player) -> Vec<(usize, usize)> {
    let mut scored: Vec<(i32, (usize, usize))> = moves.iter()
    .map(|&m| {
        board.apply_move(m, *player);
        let s = heuristic(board);
        board.undo_move(m, *player);
        (s, m)
    }).collect();
    
    if matches!(player, Player::X) {
        scored.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // descending for X
    } else {
        scored.sort_unstable_by(|a, b| a.0.cmp(&b.0)); // ascending for O
    }
    scored.into_iter().map(|(_, m)| m).collect()
}

fn evaluate(
    board: &mut Board,
    mut current_depth: i32,
    max_depth: i32,
    player: &Player,
    start_time: Instant,
    limit:Duration,
    mut alpha: i32,
    mut beta: i32,
    moves_tuple: (usize, usize),
) -> Option<(i32, usize, usize)> {
    if start_time.elapsed() >= limit { // 
        return None;
    }
    let ordered_moves = order_moves(board, &board.moves(), &player);    
    if board.game_over() { // base case to end game
        return Some((heuristic(board), 0, 0));
    }
    if current_depth == max_depth {
        //if we've reached the max depth, return the board state
        return Some((heuristic(board), ordered_moves[0].0, ordered_moves[0].1)); // todo!(need to return the move);
    }
    current_depth += 1; //we update the current depth

    let mut best_score;
    if matches!(player, Player::X) {
        best_score = i32::MIN; // score could be |2| when finishing so just setting this to something it couldn't be 
    } else {
        best_score = i32::MAX;
    }
    let mut best_move = moves_tuple;
    for m in ordered_moves{
        // for each move
        board.apply_move(m, *player);
        let score ;
        let result = evaluate(board, current_depth, max_depth, &player.flip(), start_time,limit, alpha, beta,m); // this is the same as before execpt now we just keep passing in the depths to keep track
        
        match result {
            Some(res)=>{
                score = res.0;
            },
            None =>{ // there was a bug here. 
                board.undo_move(m, *player);
                return None;
            },
        }
        board.undo_move(m, *player);
        
        if matches!(player, Player::X) {
            // X wants the highest score
            if score > best_score {
                best_score = score;
                best_move = m;
            }
            alpha = i32::max(alpha, score); //updating alpha to the max score
        } else {
            // O wants the lowest score
            if score < best_score {
                best_score = score;
                best_move = m;
            }
            beta = i32::min(beta, score); //ip dating beta to max score
        }

        if alpha >= beta {
            // alpha beta pruning //. if alpha is ever bigger than the previous then there is no use in searching
            break;
        }
    }
    return Some((best_score, best_move.0, best_move.1)); // same as before
}

fn my_score_evaluate(x: &Cell, y: &Cell, z: &Cell) -> i32 {
    if matches!(x, Cell::Wall) || matches!(y, Cell::Wall) || matches!(z, Cell::Wall) {
        return 0;
    }
    // we can make it smarter by adding more cases
    // if !sabatoge {
        return match (x, y, z) {
            // 3 in a row
            (Cell::X, Cell::X, Cell::X) => 1000,
            (Cell::O, Cell::O, Cell::O) => -1000,
            // 2 in a row for X
            (Cell::X, Cell::X, Cell::Empty)
            | (Cell::X, Cell::Empty, Cell::X)
            | (Cell::Empty, Cell::X, Cell::X) => 20,
            // 2 in a row for O
            (Cell::O, Cell::O, Cell::Empty)
            | (Cell::O, Cell::Empty, Cell::O)
            | (Cell::Empty, Cell::O, Cell::O) => -20,
            //blocking O
            (Cell::X, Cell::O, Cell::O)
            | (Cell::O, Cell::X, Cell::O)
            | (Cell::O, Cell::O, Cell::X) => 400,
            //blocking X
            (Cell::O, Cell::X, Cell::X)
            | (Cell::X, Cell::O, Cell::X)
            | (Cell::X, Cell::X, Cell::O) => -400,
            _ => 0, // return 0 for everyother case.
        }
    // } else {
    //     return match (x, y, z) {
    //         // 3 in a row
    //         (Cell::X, Cell::X, Cell::X) => 1000,
    //         (Cell::O, Cell::O, Cell::O) => -1000,
    //         // 2 in a row for X
    //         (Cell::X, Cell::X, Cell::Empty)
    //         | (Cell::X, Cell::Empty, Cell::X)
    //         | (Cell::Empty, Cell::X, Cell::X) => 5,
    //         // 2 in a row for O
    //         (Cell::O, Cell::O, Cell::Empty)
    //         | (Cell::O, Cell::Empty, Cell::O)
    //         | (Cell::Empty, Cell::O, Cell::O) => -5,
    //         //blocking O
    //         (Cell::X, Cell::O, Cell::O)
    //         | (Cell::O, Cell::X, Cell::O)
    //         | (Cell::O, Cell::O, Cell::X) => 40,
    //         //blocking X
    //         (Cell::O, Cell::X, Cell::X)
    //         | (Cell::X, Cell::O, Cell::X)
    //         | (Cell::X, Cell::X, Cell::O) => -40,
    //         _ => 0, // return 0 for everyother case.
    //     }
    }
    

fn heuristic(board: &mut Board) -> i32 { 
    // there are only so many unique 3 in a rows in a 5x5. i realized old version was checking the same rows alot. very inefficent 
    let current_board = board.get_cells(); 
    let mut score: i32 = 0; 
    let size = 5; 
    
    for row in 0..size { 
        for col in 0..size { 
            if col <= 2 { let x = &current_board[row][col]; 
                let y = &current_board[row][col + 1]; 
                let z = &current_board[row][col + 2]; 
                score += my_score_evaluate(x, y, z); 
            } if row <= 2 { 
                let x = &current_board[row][col]; 
                let y = &current_board[row + 1][col]; 
                let z = &current_board[row + 2][col]; 
                score += my_score_evaluate(x, y, z); 
            } if col <= 2 && row <= 2 { 
                let x = &current_board[row][col]; 
                let y = &current_board[row + 1][col + 1]; 
                let z = &current_board[row + 2][col + 2]; 
                score += my_score_evaluate(x, y, z); 
            } if row <= 2 && col >= 2 { 
                let x = &current_board[row][col]; 
                let y = &current_board[row + 1][col - 1]; 
                let z = &current_board[row + 2][col - 2]; 
                score += my_score_evaluate(x, y, z); 
            } 
        } 
    } 
    
    if matches!(&current_board[2][2], Cell::X){
        score += 20;
    }else{
        score -= 20;
    }    
    
    return score; 
}


/*
AI use: Student 2( Ricky Cui) used Claude to identify potential improvements, Claude pointed out bugs in the code that caused issues. Student
 2 fixed the bugs Claude pointed out with the assistance of Claude. Claude also pointed how how organizing the moves before implementing alpha/
 beta pruning makes it much more efficent. Student 2 implemented the changes Claude suggested. 
 */
