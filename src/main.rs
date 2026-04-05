mod engine;
use clap::{Parser};
use chess::{Board, BoardStatus, Square, ChessMove};
use std::str::FromStr;
use std::error::Error;
use std::process;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 5)]
    // Defaults to 5 as the standard depth, follow cargo run
    // with -- before using depth flag and inputting number
    depth: i32,
    // FEN-String encoding
    #[arg(required = true)]
    fen: String,
}
pub struct Move {
    status: BoardStatus,
    chess_move: Option<ChessMove>,
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Args) -> Result<Move, GenericError> {
    let eval_parameters = engine::config::EvalConfig::load().map_err(|e| -> GenericError { e.into() })?;
    let board = Board::from_str(&config.fen).map_err(|e| format!("{e}"))?;
    let status = board.status();
    if status != BoardStatus::Ongoing {
        println!("Game has already concluded {:?}", status);
        return Ok(Move {
            status, 
            chess_move: None,
        });
    }
    println!("We shall analyze the position at depth {}.", config.depth);

    // Put actual move generation here
    let res = engine::evaluation::evaluate_board(&board);
    println!("{}", res);

    let best_move = ChessMove::new(Square::E2, Square::E4, None);
    Ok(Move {
        status, 
        chess_move: Some(best_move),
    })
}
