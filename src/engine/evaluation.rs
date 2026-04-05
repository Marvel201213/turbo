use chess::{Board, Piece, Color, BitBoard};

pub fn evaluate_board(board: &Board) -> i32 {
    let mut score = 0;

    // Basic Point Counting with centipawns so that small positional differences can be added
    let pawn_val = 100;
    let knight_val = 300;
    let bishop_val = 320;
    let rook_val = 500;
    let queen_val = 900;

    // Simple Point Material Counting
    score += (count_pieces(board, Piece::Pawn, Color::White) as i32) * pawn_val;
    score -= (count_pieces(board, Piece::Pawn, Color::Black) as i32) * pawn_val;

    score += (count_pieces(board, Piece::Knight, Color::White) as i32) * knight_val;
    score -= (count_pieces(board, Piece::Knight, Color::Black) as i32) * knight_val;

    score += (count_pieces(board, Piece::Bishop, Color::White) as i32) * bishop_val;
    score -= (count_pieces(board, Piece::Bishop, Color::Black) as i32) * bishop_val;

    score += (count_pieces(board, Piece::Rook, Color::White) as i32) * rook_val;
    score -= (count_pieces(board, Piece::Rook, Color::Black) as i32) * rook_val;

    score += (count_pieces(board, Piece::Queen, Color::White) as i32) * queen_val;
    score -= (count_pieces(board, Piece::Queen, Color::Black) as i32) * queen_val;

    
    if board.side_to_move() == Color::White {
        score
    } else {
        -score
    }
}

fn count_pieces(board : &Board, piece : Piece, color : Color) -> u32 {
    // Piece must be of right type and color to be counted
    let bitboard = board.pieces(piece) & board.color_combined(color);

    bitboard.popcnt()
}

fn flip_index(index: usize) -> usize {
    index ^ 56
}