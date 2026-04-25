use chess::{Board, BoardStatus};
use clap::Parser;
use std::process;
use std::str::FromStr;
use std::time::Instant;
use turbo::{SearchOutput, engine};

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

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Args) -> Result<SearchOutput, GenericError> {
    let eval_parameters =
        engine::config::EvalConfig::load().map_err(|e| -> GenericError { e.into() })?;
    let mut board = Board::from_str(&config.fen).map_err(|e| format!("{e}"))?;
    let status = board.status();
    if status != BoardStatus::Ongoing {
        println!("Game has already concluded: {:?}", status);
        return Ok(SearchOutput {
            status,
            chess_move: None,
            score: 0,
            nodes: 0,
        });
    }
    let search_depth = if config.depth <= 0 {
        1
    } else {
        config.depth as usize
    };
    println!(
        "We shall analyze the position at depth {}. \n",
        search_depth
    );

    let mut searcher = engine::search::Searcher::new(&eval_parameters);

    let start = Instant::now();
    let (best_move, score) = searcher.find_best_move(&mut board, search_depth);
    let duration = start.elapsed();
    println!("Search Completed in {:?}", duration);
    let result = SearchOutput {
        status: board.status(),
        chess_move: best_move,
        score,
        nodes: searcher.get_nodes(),
    };
    // Implementation of Display Trait for SearchOutput cleans up command line output
    println!("{}", result);
    Ok(result)
}
