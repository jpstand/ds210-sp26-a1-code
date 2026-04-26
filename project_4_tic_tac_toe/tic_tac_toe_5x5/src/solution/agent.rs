use std::time::{Duration, Instant};
use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}
/* what i changed: 
- added a precheck to only evaluate the windows that dont have any walls(aka we can score in) trying 
to save time
- removed sabatoge ( i think it was sabatoging us lowkey...)
- evaluate-> the base case returns the actual score multiplied by a constant 
- added a late game checker to motivate the bot to play smarter (kinda like our old sabatoge func but this time it doesnt sabatoge ourselves )
- implementing counting instead of checking each cell (counting is faster)
- added a reward for a "fork" when you can still score even if the other player blocks one

 */

// adjusted the heuristic so that it doesn't need to allocate memory for a vector every move
// adjusted the heuristic so that we keep track of moves for late game checker instead of calling board.moves()
 
// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize) {
    // ^ this method will allow us to search as far as possible on any computer because we can fit our depth search acording to the time taken rather than a hard coded number.
    let start_time = Instant::now();
    let limit = Duration::from_millis((time_limit as f64 * 0.989999999) as u64); // shave off a tiny bit so we never go over

    // only precompute the windows once per solve call, not every node
    let valid_windows: Vec<[(usize, usize); 3]> = precompute_valid_windows(board.get_cells());

    let moves: Vec<(usize, usize)> = board.moves();
    if moves.is_empty() { // shouldnt happen but avoids a panic on moves[0]
        return (0, 0, 0);
    }

    let mut best_move: (i32, usize, usize) = (0, moves[0].0, moves[0].1); // default to first available move
    for max_depth in 1..21 { // iterative deepening - go deeper until time runs out
        if start_time.elapsed() >= limit {
            break; // keep whatever best_move we have so far
        }
        // implementing Min Max Alpha/Beta Pruning 
        if let Some(res) = evaluate(
            board,
            0,
            max_depth,
            player,
            start_time,
            limit,
            i32::MIN,
            i32::MAX,
            &valid_windows,
            moves.len(),
        ) {
            best_move = res; // only update if we finished the depth, partial searches get thrown out
        }
    }
    return best_move;

    // code never goes past here
    //------

    // sorts moves so we look at the most promising ones first, makes alpha beta pruning way more effective
    fn order_moves(
        board: &mut Board,
        moves: &[(usize, usize)],
        player: Player,
        valid_windows: &[[(usize, usize); 3]],
        move_count: usize,
    ) -> Vec<(usize, usize)> {
        let mut scored: Vec<(i32, (usize, usize))> = moves
            .iter()
            .map(|&m| {
                board.apply_move(m, player);
                let s = heuristic(board, valid_windows, move_count-1); // quick score for ordering, we do -1 for move count because we just placed
                board.undo_move(m, player);
                (s, m)
            })
            .collect();

        if matches!(player, Player::X) {
            scored.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // descending for X
        } else {
            scored.sort_unstable_by(|a, b| a.0.cmp(&b.0)); // ascending for O
        }
        return scored.into_iter().map(|(_, m)| m).collect();
    }
    //--------

    fn evaluate(
        board: &mut Board,
        mut current_depth: i32,
        max_depth: i32,
        player: Player,
        start_time: Instant,
        limit: Duration,
        mut alpha: i32,
        mut beta: i32,
        valid_windows: &Vec<[(usize, usize); 3]>,
        move_count: usize,
    ) -> Option<(i32, usize, usize)> {
        if start_time.elapsed() >= limit {
            return None; // signal to the caller that this search didnt finish
        }
        if board.game_over() { // base case to end game
            return Some((board.score() * 100_000, 0, 0)); // scale up so terminal states always beat heuristic scores
        }
        if current_depth >= max_depth {
            return Some((heuristic(board, &valid_windows, move_count), 0, 0)); // hit the depth limit, estimate from here
        }

        let moves = board.moves();
        let ordered_moves = order_moves(board, &moves, player, &valid_windows, moves.len());
        current_depth += 1; // we update the current depth

        let mut best_score;
        if matches!(player, Player::X) {
            best_score = i32::MIN;
        } else {
            best_score = i32::MAX;
        }
        let mut best_move: (usize, usize) = if ordered_moves.len() > 0 {
            ordered_moves[0]
        } else {
            (0, 0) // handles case where ordered_moves is empty
        };

        for m in ordered_moves {
            board.apply_move(m, player);
            let result = evaluate(
                board,
                current_depth,
                max_depth,
                player.flip(), // alternate players each level
                start_time,
                limit,
                alpha,
                beta,
                &valid_windows,
                moves.len() - 1, //we just applied a move
            );
            board.undo_move(m, player);

            match result {
                Some(res) => {
                    let score = res.0;

                    if matches!(player, Player::X) {
                        // X wants the highest score
                        if score > best_score {
                            best_score = score;
                            best_move = m;
                        }
                        alpha = i32::max(alpha, score); // updating alpha to the max score
                    } else {
                        // O wants the lowest score
                        if score < best_score {
                            best_score = score;
                            best_move = m;
                        }
                        beta = i32::min(beta, score); // updating beta to the min score
                    }

                    if alpha >= beta {
                        // alpha beta pruning - opponent already has a better option elsewhere so stop searching this branch
                        break;
                    }
                }
                None => {
                    return None; // time ran out mid search, bubble it up
                }
            }
        }
        return Some((best_score, best_move.0, best_move.1));
    }

    //--------

    fn heuristic(board: &mut Board, valid_windows: &[[(usize, usize); 3]], move_count: usize) -> i32 {
        let mut score = 0;
        let cells = board.get_cells();
        // use fewer empty squares as a signal that we're in endgame and should weight threats more heavily
        
        let is_late_game = move_count <= 8;

        // tracks how many windows each empty square belongs to - high overlap means a fork opportunity
        let mut x_potential: [[i32; 5]; 5] = [[0; 5]; 5];
        let mut o_potential: [[i32; 5]; 5] = [[0; 5]; 5];
        
        for window in valid_windows {
            let mut x_count = 0;
            let mut o_count = 0;
            let mut empty_cells: [(usize, usize); 3] = [(0, 0); 3];
            let mut empty_count = 0;

            for &(r, c) in window {
                match cells[r][c] {
                    Cell::X => x_count += 1,
                    Cell::O => o_count += 1,
                    Cell::Empty => {
                        empty_cells[empty_count] = (r, c); 
                        empty_count += 1; 
                    },
                    _ => {}
                }
            }

            // score the window and mark the empty squares as contested
            match (x_count, o_count) {
                (3, 0) => score += 100_000, // already a win
                (0, 3) => score -= 100_000,
                (2, 0) => {
                    score += if is_late_game { 
                        800 
                    } else {
                        500 
                    }; // worth more late game since theres less time to block
                    for &(r, c) in &empty_cells[..empty_count] {
                        x_potential[r][c] += 2;
                    }
                }
                (0, 2) => {
                    score -= if is_late_game { 
                        800 
                    } else { 
                        500
                    };
                    for &(r, c) in &empty_cells[..empty_count] {
                        o_potential[r][c] += 2;
                    }
                }
                (1, 0) => {
                    score += 10;
                    for &(r, c) in &empty_cells[..empty_count] {
                        x_potential[r][c] += 1;
                    }
                }
                (0, 1) => {
                    score -= 10;
                    for &(r, c) in &empty_cells[..empty_count] {
                        o_potential[r][c] += 1;
                    }
                }
                _ => {} // mixed window, no one can win through here
            }
        }

        // fork checker - if an empty square sits in 4+ windows for one player, filling it creates two threats at once
        let fork_bonus = if is_late_game { 1500 } else { 600 };

        for r in 0..5 {
            for c in 0..5 {
                if matches!(cells[r][c], Cell::Empty) {
                    if x_potential[r][c] >= 4 {
                        score += fork_bonus; // good for X
                    }
                    if o_potential[r][c] >= 4 {
                        score -= fork_bonus; // good for O
                    }
                }
            }
        }
        score
    }

    //--------

    // called once at the start of solve so we dont recheck walls on every heuristic call
    fn precompute_valid_windows(cells: &Vec<Vec<Cell>>) -> Vec<[(usize, usize); 3]> {
        let mut valid_win_spots = vec![];

        for row in 0..5 {
            for col in 0..5 {
                // horizontal
                if col <= 2 {
                    let w = [(row, col), (row, col + 1), (row, col + 2)];
                    if w.iter().all(|&(r, c)| !matches!(cells[r][c], Cell::Wall)) {
                        valid_win_spots.push(w);
                    }
                }
                // vertical
                if row <= 2 {
                    let w = [(row, col), (row + 1, col), (row + 2, col)];
                    if w.iter().all(|&(r, c)| !matches!(cells[r][c], Cell::Wall)) {
                        valid_win_spots.push(w);
                    }
                }
                // diagonal
                if row <= 2 && col <= 2 {
                    let w = [(row, col), (row + 1, col + 1), (row + 2, col + 2)];
                    if w.iter().all(|&(r, c)| !matches!(cells[r][c], Cell::Wall)) {
                        valid_win_spots.push(w);
                    }
                }
                // anti-diagonal
                if row <= 2 && col >= 2 {
                    let w = [(row, col), (row + 1, col - 1), (row + 2, col - 2)];
                    if w.iter().all(|&(r, c)| !matches!(cells[r][c], Cell::Wall)) {
                        valid_win_spots.push(w);
                    }
                }
            }
        }
        return valid_win_spots;
    }
}
}



/*
AI use: Student 2( Ricky Cui) used Claude to identify potential improvements, Claude pointed out bugs in the code that caused issues. Student
 2 fixed the bugs Claude pointed out with the assistance of Claude. Claude also pointed how how organizing the moves before implementing alpha/
 beta pruning makes it much more efficent. Student 2 implemented the changes Claude suggested. Also used AI to brainstorm how to improve hureristic func.

Student 1 removed the sabotage portion (without AI use) but used Claude to check if any other improvements could be helpful. Replaced the heap allocated vec![] in 
the heuristic func with a fixed size stack array. This sohuld eliminate repeated heap allocations across the search tree. Also eliminated a redundant board.moves() 
call inside heuristic by passing the move count down through evaluate instead, since board.moves() was only being called there to check is_late_game condition.
 */
