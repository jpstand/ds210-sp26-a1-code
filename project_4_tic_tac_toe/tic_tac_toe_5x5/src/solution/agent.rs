use std::cmp::max;
use std::thread::current;

use rand::Rng;
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
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let max_depth = 4; // max depth for depth tracking function 4 works to pass all tests but one on my computer 
        let current_depth = 0; // starting depth

        //todo!("maybe set a timer that uses time_limit and returns the best option we have before the timer runs out. so we can explore as many cases that the time allows. ");
        // ^ this method will allow us to search as far as possible on any computer because we can fit our depth search acording to the time taken rather than a hard coded number.

        //implementing Aplpha/Beta Pruning - gets rid of any options that the opponiente would not allow you to win in
        let alpha = i32::MIN;
        let beta = i32::MAX;

        let cells = board.moves();
        
        let mut rng = rand::thread_rng();
        let moves: (usize, usize) = cells[rng.gen_range(0..cells.len())]; // start in a random spot 
         
        if cells.len() >= 18 {
            // first move doesnt matter as much, just go somewhere random. calculations would take too much time
            return (0, moves.0, moves.1);
        }
        //maybe implement a check here that checks if we are winning, if we are spend the rest of the game making the other team lose. if not, draw the game.
        let mut sabatoge = false;
        let score = heuristic(board, sabatoge);
        
        match player {
            Player::X=>{
                if score > 0{
                    sabatoge = true;
                }
            }
            Player::O=>{
                if score < 0{
                    sabatoge = true;
                }
            }

        }
        let result = depth_tracking(board, current_depth, max_depth, &player, &_time_limit, alpha, beta, moves, sabatoge);
        
        return result;
    }
}

fn depth_tracking(
    board: &mut Board,
    mut current_depth: i32,
    max_depth: i32,
    player: &Player,
    _time_limit: &u64,
    mut alpha: i32,
    mut beta: i32,
    mut moves_tuple: (usize, usize),
    sabatoge: bool,
) -> (i32, usize, usize) {
    if board.game_over() {
        // base case
        return (board.score(), 0, 0);
    }
    if current_depth == max_depth {
        //if we've reached the max depth, return the board state
        let cells = board.moves();
        let mut rng = rand::thread_rng();
        let moves: (usize, usize) = cells[rng.gen_range(0..cells.len())]; // start in a random spot 
        return (heuristic(board, sabatoge), moves.0, moves.1); // todo!(need to return the move);
        //return (heuristic(board, sabatoge), 0, 0);
    }
    current_depth += 1; //we update the current depth

    let mut best_score;
    if matches!(player, Player::X) {
        best_score = i32::MIN; // score could be |2| when finishing so just setting this to something it couldn't be 
    } else {
        best_score = i32::MAX;
    }
    let moves = board.moves(); // available moves
    for m in moves.clone() {
        // for each move
        board.apply_move(m, *player);
        let result = depth_tracking(board, current_depth, max_depth, &player.flip(), _time_limit, alpha, beta,moves_tuple, sabatoge); // this is the same as before execpt now we just keep passing in the depths to keep track
        let score = result.0;
        board.undo_move(m, *player);

        if matches!(player, Player::X) {
            // X wants the highest score
            if score > best_score {
                best_score = score;
                moves_tuple = m;
            }
            alpha = i32::max(alpha, score); //updating alpha to the max score
        } else {
            // O wants the lowest score
            if score < best_score {
                best_score = score;
                moves_tuple = m;
            }
            beta = i32::min(beta, score); //ip dating beta to max score
        }

        if alpha >= beta {
            // alpha beta pruning //. if alpha is ever bigger than the previous then there is no use in searching
            break;
        }
    }
    return (best_score, moves_tuple.0, moves_tuple.1); // same as before
}

fn my_score_evaluate(x: &Cell, y: &Cell, z: &Cell, sabatoge: bool) -> i32 {
    if matches!(x, Cell::Wall) || matches!(y, Cell::Wall) || matches!(z, Cell::Wall) {
        return 0;
    }
    let mut total = 0;
    // we can make it smarter by adding more cases
    if !sabatoge {
        total = match (x, y, z) {
            // Walls: Early exit
            (Cell::Wall, _, _) | (_, Cell::Wall, _) | (_, _, Cell::Wall) => 0,

            // 3 in a row
            (Cell::X, Cell::X, Cell::X) => 100,
            (Cell::O, Cell::O, Cell::O) => -100,

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
            | (Cell::O, Cell::O, Cell::X) => 10,

            //blocking X
            (Cell::O, Cell::X, Cell::X)
            | (Cell::X, Cell::O, Cell::X)
            | (Cell::X, Cell::X, Cell::O) => -10,

            _ => 0, // return 0 for everyother case.
        }
    }else{
        if !matches!(x, Cell::Wall) && !matches!(y, Cell::Wall) && !matches!(z, Cell::Wall) {
            total = match (x, y, z) {
                // Walls: Early exit
                (Cell::Wall, _, _) | (_, Cell::Wall, _) | (_, _, Cell::Wall) => 0,

                // 3 in a row
                (Cell::X, Cell::X, Cell::X) => 90,
                (Cell::O, Cell::O, Cell::O) => -90,

                // 2 in a row for X
                (Cell::X, Cell::X, Cell::Empty)
                | (Cell::X, Cell::Empty, Cell::X)
                | (Cell::Empty, Cell::X, Cell::X) => 5,

                // 2 in a row for O
                (Cell::O, Cell::O, Cell::Empty)
                | (Cell::O, Cell::Empty, Cell::O)
                | (Cell::Empty, Cell::O, Cell::O) => -5,

                //blocking O
                (Cell::X, Cell::O, Cell::O)
                | (Cell::O, Cell::X, Cell::O)
                | (Cell::O, Cell::O, Cell::X) => 40,

                //blocking X
                (Cell::O, Cell::X, Cell::X)
                | (Cell::X, Cell::O, Cell::X)
                | (Cell::X, Cell::X, Cell::O) => -40,

                _ => 0, // return 0 for everyother case.
            }
        }
    }
    
    
    return total;
}
    
fn my_score(board: &Board, row: usize, col: usize, sabatoge: bool) -> i32 {
    //TODO = change so it checks for walls
    let current_board = board.get_cells();
    let mut score: i32 = 0;

    for i in 0..3 {
        for j in 0..3 {
            let check_row = i + row;
            let check_col = j + col;
            let size = 5;

            // Count row.
            if check_col + 2 < size {
                let x = &current_board[check_row][check_col];
                let y = &current_board[check_row][check_col + 1];
                let z = &current_board[check_row][check_col + 2];
                score += my_score_evaluate(x, y, z, sabatoge);
            }
            // Count col.
            if check_row + 2 < size {
                let x = &current_board[check_row][check_col];
                let y = &current_board[check_row + 1][check_col];
                let z = &current_board[check_row + 2][check_col];
                score += my_score_evaluate(x, y, z, sabatoge);
            }

            // 1st diagonal
            if check_row + 2 < size && check_col + 2 < size {
                let x = &current_board[check_row][check_col];
                let y = &current_board[check_row + 1][check_col + 1];
                let z = &current_board[check_row + 2][check_col + 2];
                score += my_score_evaluate(x, y, z, sabatoge);
            }

            // 2nd diagonal
            if check_row + 2 < size && check_col >= 2 {
                let x = &current_board[check_row][check_col];
                let y = &current_board[check_row + 1][check_col - 1];
                let z = &current_board[check_row + 2][check_col - 2];
                score += my_score_evaluate(x, y, z, sabatoge);
            }
        }
    }

    return score;
}

fn heuristic(board: &mut Board, sabatoge: bool) -> i32 {
    // only need to check 9 times
    // break the 5x5 into 9 small 3x3 boards
    // i = 0-2 j = 0-2
    // i = 0-2 j = 1-3
    // i = 0-2 j = 2-4
    // i = 1-3 j = 0-2
    // i = 2-4 j = 0-2
    // i = 1-3 j = 1-3
    // i = 2-4 j = 1-3
    // i = 1-3 j = 1-3
    // i = 2-4 j = 2-4
    let mut score = 0;

    for row in 0..3 {
        for col in 0..3 {
            score += my_score(board, row, col, sabatoge);
        }
    }

    return score; // placeholder this needs adjusting. We pass all tests but one right now
}
