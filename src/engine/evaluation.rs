use chess::{Board, Piece, Color, BitBoard};

pub fn evaluate_board(board: &Board) -> i32 {
    let mut mg_score = 0;
    let mut eg_score = 0;

    mg_score += calculate_values(board, Piece::Pawn, Color::White, true) as i32;
    mg_score += calculate_values(board, Piece::Knight, Color::White, true) as i32;
    mg_score += calculate_values(board, Piece::Bishop, Color::White, true) as i32;
    mg_score += calculate_values(board, Piece::Rook, Color::White, true) as i32;
    mg_score += calculate_values(board, Piece::Queen, Color::White, true) as i32;
    mg_score += calculate_values(board, Piece::King, Color::White, true) as i32;

    mg_score -= calculate_values(board, Piece::Pawn, Color::Black, true) as i32;
    mg_score -= calculate_values(board, Piece::Knight, Color::Black, true) as i32;
    mg_score -= calculate_values(board, Piece::Bishop, Color::Black, true) as i32;
    mg_score -= calculate_values(board, Piece::Rook, Color::Black, true) as i32;
    mg_score -= calculate_values(board, Piece::Queen, Color::Black, true) as i32;
    mg_score -= calculate_values(board, Piece::King, Color::Black, true) as i32;

    eg_score += calculate_values(board, Piece::Pawn, Color::White, true) as i32;
    eg_score += calculate_values(board, Piece::Knight, Color::White, true) as i32;
    eg_score += calculate_values(board, Piece::Bishop, Color::White, true) as i32;
    eg_score += calculate_values(board, Piece::Rook, Color::White, true) as i32;
    eg_score += calculate_values(board, Piece::Queen, Color::White, true) as i32;
    eg_score += calculate_values(board, Piece::King, Color::White, true) as i32;

    eg_score -= calculate_values(board, Piece::Pawn, Color::Black, true) as i32;
    eg_score -= calculate_values(board, Piece::Knight, Color::Black, true) as i32;
    eg_score -= calculate_values(board, Piece::Bishop, Color::Black, true) as i32;
    eg_score -= calculate_values(board, Piece::Rook, Color::Black, true) as i32;
    eg_score -= calculate_values(board, Piece::Queen, Color::Black, true) as i32;
    eg_score -= calculate_values(board, Piece::King, Color::Black, true) as i32;

    let mut current_phase = 0;
    let bitboard = board.pieces(Piece::Knight) & board.combined();
    current_phase += bitboard.popcnt();
    let bitboard = board.pieces(Piece::Bishop) & board.combined();
    current_phase += bitboard.popcnt();
    let bitboard = board.pieces(Piece::Rook) & board.combined();
    current_phase += 2 * bitboard.popcnt();
    let bitboard = board.pieces(Piece::Queen) & board.combined();
    current_phase += 4 * bitboard.popcnt();
    let current_phase = current_phase as i32;

    // Linearly interpolates based on game phase rating assigned based on # of pieces of each kind
    ((mg_score * current_phase) + (eg_score * (24 - current_phase)))/24
}

fn calculate_values(board : &Board, piece : Piece, color : Color, mg_flag: bool) -> u32 {
    0
    
}