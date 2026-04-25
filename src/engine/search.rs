use crate::engine::config::{EvalConfig};
use chess::{ChessMove, Board, MoveGen, Piece};
use super::evaluation::evaluate_board;

const MATE_VALUE: u32 = 20000;
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
        // Alpha serves as the maximizing score found so far for the current player to move, not int min because int overflow when negated
        let mut alpha = -i32::MAX;
        // Beta is the minimizing score so far for the opposition (assuming the opposition plays competently,
        // they will not lead you down a path with a score better for you than beta, allowing pruning)
        let beta = i32::MAX;
        if depth == 0 {
            return (None, evaluate_board(board, self.config));
        }
        let movegen = self.order_moves(board);
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
        if depth == 0 {
            return self.capture_checker(board, alpha, beta);
        }
        let movegen = self.order_moves(board);
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

    fn order_moves(&mut self, board: &mut Board) -> Vec<ChessMove> {
        let mut movegen: Vec<ChessMove> = MoveGen::new_legal(board).collect();
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

    fn capture_checker(&self, board: &Board, mut alpha: i32, beta: i32) -> i32 {
        1
    }
}
