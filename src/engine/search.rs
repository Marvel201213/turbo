use crate::engine::config::{EvalConfig};
use chess::{ChessMove, Board, MoveGen, Piece};
use super::evaluation::evaluate_board;

const MATE_VALUE: u32 = 20000;
const EXTREME: u32 = 500000;
pub struct Searcher<'a> {
    config: &'a EvalConfig, 
    nodes: u64
}

impl<'a> Searcher<'a> {
    pub fn new(config: &'a EvalConfig) -> Self {
        Searcher { config: config , nodes: 0 }
    }

    pub fn find_best_move(&mut self, board: &mut Board, depth: usize) -> (Option<ChessMove>, i32) {
        let mut best_move = None;
        // Alpha serves as the maximizing score found so far for the current player to move, not int min because int overflow possibilities
        let mut alpha = -(EXTREME as i32);
        // Beta is the minimizing score so far for the opposition (assuming the opposition plays competently,
        // they will not lead you down a path with a score better for you than beta, allowing pruning)
        let beta = (EXTREME as i32);
        if depth == 0 {
            return (None, evaluate_board(board, self.config));
        }
        let movegen = self.order_moves(board, false);
        for  m in movegen {
            let mut next_board = Board::default();
            board.make_move(m, &mut next_board);
            let score = -self.negamax(&mut next_board, depth - 1, -beta, -alpha);
            if score > alpha {
                alpha = score;
                best_move = Some(m);
            }
        }
        (best_move, alpha)
    }

    fn negamax(&mut self, board: &mut Board, depth: usize, mut alpha: i32, beta: i32) -> i32 {
        self.nodes+=1;
        if depth == 0 {
            return self.capture_checker(board, alpha, beta);
        }
        let movegen = self.order_moves(board, false);
        if movegen.is_empty() && board.checkers().popcnt() > 0 {
            -(MATE_VALUE as i32) + (depth as i32)
        } else if movegen.is_empty() {
            0
        } else {
            for m in movegen {
                let mut next_board = Board::default();
                board.make_move(m, &mut next_board);
                let score = -self.negamax(&mut next_board, depth - 1, -beta, -alpha);
                if score >= beta {
                    return beta;
                }
                if score > alpha {
                    alpha = score;
                }
            }
            alpha
        }
    }

    fn order_moves(&mut self, board: &mut Board, noisy_flag: bool) -> Vec<ChessMove> {
        let movegen: Vec<ChessMove> = if noisy_flag {
            MoveGen::new_legal(board).filter(|m| board.piece_on(m.get_dest()).is_some() || m.get_promotion().is_some()).collect()
        } else {
            MoveGen::new_legal(board).collect()
        };
        // Basically stores tuple with evaluation using the move scoring with MVV-LVA
        let mut score_moves: Vec<(ChessMove, i32)> = movegen.into_iter().map(|m| {
            let score = self.score_move(&m, board);
            (m, score)
        }).collect();
        score_moves.sort_by_key(|&(_, score)| -score);
        score_moves.into_iter().map(|(m, _)| m).collect()
    }

    fn score_move(&self, mv: &ChessMove, board: &Board) -> i32 {
        let mut score = 0;
        let attacker = board.piece_on(mv.get_source()).unwrap();
        if let Some(victim) = board.piece_on(mv.get_dest()) {
            score = (10 * self.config.get_value(victim)) - self.config.get_value(attacker);
            score+=10000;
        }

        if let Some(promotion) = mv.get_promotion() {
            score += self.config.get_value(promotion)
        }
        score
    }

    // Won't recurse infinitely as number of pieces on board or other opportunites for noisy moves decrease with each noisy move, validating a termination argument
    fn capture_checker(&mut self, board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
        self.nodes+=1;
        let static_eval = evaluate_board(board, self.config);
        if static_eval >= beta {
            return beta;
        } 
        if static_eval > alpha {
            alpha = static_eval;
        }
        let noisy_moves = self.order_moves(board, true);
        for m in noisy_moves {
            let mut next_board = Board::default();
            board.make_move(m, &mut next_board);
            let score = -self.capture_checker(&mut next_board, -beta, -alpha);
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }
        alpha
    }
}
