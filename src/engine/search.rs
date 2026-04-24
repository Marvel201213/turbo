use crate::engine::config::{EvalConfig};
use chess::{ChessMove, Board, MoveGen, Piece};

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

    }

    fn negamax(&mut self, board: &mut Board, depth: usize, mut alpha: i32, beta: i32) -> i32 {
        if depth == 0 {
            self.capture_checker(board, alpha, beta);
        }
        let movegen = self.order_moves(board);
        if movegen.is_empty() && board.checkers().popcnt() > 0 {
            -(MATE_VALUE as i32) + (depth as i32)
        } else if movegen.is_empty() {
            0
        } else {
            1 // Filler
        }
    }

    fn order_moves(&mut self, board: &mut Board) -> Vec<ChessMove> {
        let mut movegen: Vec<ChessMove> = MoveGen::new_legal(board).collect();
        let board_ref: &Board = board;
        movegen.sort_by_key(|m| -self.score_move(m, board_ref));
        movegen
    }

    fn score_move(&mut self, mv: &ChessMove, board: &Board) -> i32 {
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

    fn capture_checker(&mut self, board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
        1
    }
}
