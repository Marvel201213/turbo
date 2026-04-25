use crate::engine::config;
use chess::{Board, Color, Piece};

pub fn evaluate_board(board: &Board, eval_config: &config::EvalConfig) -> i32 {
    let mut mg_score = 0;
    let mut eg_score = 0;

    mg_score += calculate_values(board, Piece::Pawn, Color::White, true, eval_config);
    mg_score += calculate_values(board, Piece::Knight, Color::White, true, eval_config);
    mg_score += calculate_values(board, Piece::Bishop, Color::White, true, eval_config);
    mg_score += calculate_values(board, Piece::Rook, Color::White, true, eval_config);
    mg_score += calculate_values(board, Piece::Queen, Color::White, true, eval_config);
    mg_score += calculate_values(board, Piece::King, Color::White, true, eval_config);

    mg_score -= calculate_values(board, Piece::Pawn, Color::Black, true, eval_config);
    mg_score -= calculate_values(board, Piece::Knight, Color::Black, true, eval_config);
    mg_score -= calculate_values(board, Piece::Bishop, Color::Black, true, eval_config);
    mg_score -= calculate_values(board, Piece::Rook, Color::Black, true, eval_config);
    mg_score -= calculate_values(board, Piece::Queen, Color::Black, true, eval_config);
    mg_score -= calculate_values(board, Piece::King, Color::Black, true, eval_config);

    eg_score += calculate_values(board, Piece::Pawn, Color::White, false, eval_config);
    eg_score += calculate_values(board, Piece::Knight, Color::White, false, eval_config);
    eg_score += calculate_values(board, Piece::Bishop, Color::White, false, eval_config);
    eg_score += calculate_values(board, Piece::Rook, Color::White, false, eval_config);
    eg_score += calculate_values(board, Piece::Queen, Color::White, false, eval_config);
    eg_score += calculate_values(board, Piece::King, Color::White, false, eval_config);

    eg_score -= calculate_values(board, Piece::Pawn, Color::Black, false, eval_config);
    eg_score -= calculate_values(board, Piece::Knight, Color::Black, false, eval_config);
    eg_score -= calculate_values(board, Piece::Bishop, Color::Black, false, eval_config);
    eg_score -= calculate_values(board, Piece::Rook, Color::Black, false, eval_config);
    eg_score -= calculate_values(board, Piece::Queen, Color::Black, false, eval_config);
    eg_score -= calculate_values(board, Piece::King, Color::Black, false, eval_config);

    let mut current_phase = 0;
    let bitboard = board.pieces(Piece::Knight);
    current_phase += bitboard.popcnt();
    let bitboard = board.pieces(Piece::Bishop);
    current_phase += bitboard.popcnt();
    let bitboard = board.pieces(Piece::Rook);
    current_phase += 2 * bitboard.popcnt();
    let bitboard = board.pieces(Piece::Queen);
    current_phase += 4 * bitboard.popcnt();
    let current_phase = current_phase as i32;

    // Linearly interpolates based on game phase rating assigned based on # of pieces of each kind
    let score = ((mg_score * current_phase) + (eg_score * (24 - current_phase))) / 24;
    if board.side_to_move() == Color::Black {
        -score
    } else {
        score
    }
}

fn calculate_values(
    board: &Board,
    piece: Piece,
    color: Color,
    mg_flag: bool,
    eval_config: &config::EvalConfig,
) -> i32 {
    // Selects for all pieces of the appropriate color and type
    let pst_values = eval_config.get_array(piece, mg_flag);
    let bit_board = board.pieces(piece) & board.color_combined(color);
    let mut values = 0;
    for i in bit_board {
        let mut index = i.to_index();
        if color == Color::Black {
            index ^= 56;
        }
        values += pst_values[index];
    }
    values += eval_config.get_value(piece) * (bit_board.popcnt() as i32);
    values
}
