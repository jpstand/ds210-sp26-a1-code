use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::{self, Player};

use crate::solution;
use tic_tac_toe_stencil::board::Cell;

// Your solution solution.
pub struct SolutionAgent2 {}

// Put your solution here.
impl Agent for SolutionAgent2 {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // we are hard coding all the possible moves
        let moves = board.moves(); // available moves
        let score = board.score();
        let cells = board.get_cells(); // we see which parts of the board are taken and whether they are x or o

        let center: (usize, usize) = (1, 1);
        let corners: Vec<(usize, usize)> = vec![(0, 0), (0, 2), (2, 0), (2, 2)];
        let edges: Vec<(usize, usize)> = vec![(0, 1), (1, 0), (1, 2), (2, 1)];

        if board.game_over() {
            // base case
            return (score, 0, 0);
        }

        // turn 1 logic
        if moves.len() == 9 || (moves.len() == 8 && moves.contains(&center)) {
            // if you're first take the center, or if you're second and the center is open, take it
            return (score, 1, 1);
        }

        // turn 2 logic
        if moves.len() == 8 && moves.contains(&center) == false {
            //if you go second but the person took the center, take the top left corner
            return (score, 0, 0);
        }

        //turn 3 logic <- when we get to this point there are only three possible board states.
        // 1. you have center and opponent is in an edge
        // 2. you have center and opponent is in a corner
        // 3. the opponent has the center and you are in the top left corner <- in this case turn 3 never occurs for us, but turn 4 will

        if moves.len() == 7
            && (matches!(player, Player::X) && cells[1][1] == Cell::X
                || matches!(player, Player::O) && cells[1][1] == Cell::O)
        {
            //for us to make a move on turn 3 we need to have a center cell meaning we moved first
            // if 1, you need to take a corner opposite to the edge.
            if let Some(edge) = edges.iter().find(|edge| !moves.contains(edge)) {
                // I used claude to figure out this syntax of using find and the |   | to see if a vector contains a specific value
                if edge.0 == 0 && edge.1 == 1 {
                    return (score, 2, 0);
                }

                if edge.0 == 2 && edge.1 == 1 {
                    return (score, 0, 0);
                }

                if edge.0 == 1 {
                    if edge.1 == 0 {
                        return (score, 0, 2);
                    }
                    if edge.1 == 2 {
                        return (score, 0, 0);
                    }
                }
            } else {
                //if 2, I need to take the corner on the other end of the diagnol
                if let Some(corner) = corners.iter().find(|corner| !moves.contains(corner)) {
                    if corner.0 == 0 {
                        if corner.1 == 0 {
                            return (score, 2, 2);
                        }
                        if corner.1 == 2 {
                            return (score, 2, 0);
                        }
                    }
                    if corner.0 == 2 {
                        if corner.1 == 0 {
                            return (score, 0, 2);
                        }
                        if corner.1 == 2 {
                            return (score, 0, 0);
                        }
                    }
                }
            }
        }

        //turn 4 logic <- when we get to this point there are 2 possible board states. This means case 3 above happened (from turn 2)
        // 1. the opponent has the center, i have top left corner, and they selected a corner
        // 2. the opponent has the center, i have the top left corner, and they selected an edge
        if moves.len() == 6
            && (matches!(player, Player::X) && cells[1][1] == Cell::O
                || matches!(player, Player::O) && cells[1][1] == Cell::X)
        {
            //if I don't have center cell <- necessary for us to do turn 6
            if let Some(corner) = corners
                .iter()
                .filter(|corner| **corner != (0, 0))
                .find(|corner| !moves.contains(corner))
            {
                //so if we're in 1. and they select the corner adjacent to us we have to pick the corner opposite to that
                if corner.0 == 0 {
                    return (score, 2, 0);
                }
                if corner.0 == 2 {
                    if corner.1 == 0 {
                        return (score, 0, 2);
                    }
                    if corner.1 == 2 {
                        return (score, 0, 2); //we return the top right corner if they pick the bottom right one
                    }
                }
            } else {
                //otherwise we're in 2.
                if let Some(edge) = edges.iter().find(|edge| !moves.contains(edge)) {
                    if edge.0 == 0 {
                        return (score, 2, 1);
                    }
                    if edge.0 == 1 {
                        if edge.1 == 0 {
                            return (score, 1, 2);
                        } else {
                            return (score, 1, 0);
                        }
                    }
                    if edge.0 == 2 {
                        return (score, 0, 1);
                    }
                }
            }
        }

        // atp I can just ask can I win? since edges/corners are predetermined up to here
        for &(r, c) in &moves {
            // index each possible board space
            board.apply_move((r, c), player); //apply a move there 
            if board.game_over() {
                //if this wins the board, then we just return that
                board.undo_move((r, c), player);
                if matches!(player, Player::X) {
                    return (1, r, c);
                } else {
                    return (-1, r, c);
                }
            }
            board.undo_move((r, c), player);
        }

        // block opponent from winning
        for &(r, c) in &moves {
            board.apply_move((r, c), player.flip());
            if board.game_over() && board.score() != 0 {
                board.undo_move((r, c), player.flip());
                return (score, r, c);
            }
            board.undo_move((r, c), player.flip());
        }
        // if we don't hit above logic, pick a corner that's available. This is better than just returning 0, 0.
        for &(r, c) in &corners {
            if moves.contains(&(r, c)) {
                return (score, r, c);
            }
        }

        // same case but if no corner then just do an edge. We prioritize corners over edges because a corner gives you options of block things from up to 3 directions
        for &(r, c) in &edges {
            if moves.contains(&(r, c)) {
                return (score, r, c);
            }
        }

        return (score, 0, 0); //shouldn't get to this point but it's here
    }
}
