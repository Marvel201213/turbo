pub mod engine;
use chess::{BoardStatus, ChessMove};
pub use engine::config::EvalConfig;
pub use engine::search::Searcher;

pub struct SearchOutput {
    pub status: BoardStatus,
    pub chess_move: Option<ChessMove>,
    pub score: i32,
    pub nodes: u64,
}

impl std::fmt::Display for SearchOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m_str = self
            .chess_move
            .map_or("None".to_string(), |m| m.to_string());
        let eval = self.score as f32 / 100.0;
        if self.score.abs() > 15000 {
            write!(
                f,
                "Best Move: {}\nEvaluation: Mating Sequence Found\nNodes Searched: {}\nStatus: {:?}",
                m_str, self.nodes, self.status
            )
        } else {
            write!(
                f,
                "Best Move: {}\nEvaluation: {:.2}\nNodes Searched: {}\nStatus: {:?}",
                m_str, eval, self.nodes, self.status
            )
        }
    }
}
