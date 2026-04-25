//! Logic for engine configuration based on Piece-Square Tables.
//!
//! This module handles deserialization of JSON configuration files
//! and provides the tapered evaluation values for different game stages utilized in evaluation.

use anyhow::Context;
use chess::Piece;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::fs;

/// Stores the weights and values for piece evaluation.
#[derive(Serialize, Deserialize, Debug)]
pub struct EvalConfig {
    #[serde(with = "BigArray")]
    pub mg_pawn: [i32; 64],
    #[serde(with = "BigArray")]
    pub eg_pawn: [i32; 64],
    #[serde(with = "BigArray")]
    pub mg_knight: [i32; 64],
    #[serde(with = "BigArray")]
    pub eg_knight: [i32; 64],
    #[serde(with = "BigArray")]
    pub mg_bishop: [i32; 64],
    #[serde(with = "BigArray")]
    pub eg_bishop: [i32; 64],
    #[serde(with = "BigArray")]
    pub mg_rook: [i32; 64],
    #[serde(with = "BigArray")]
    pub eg_rook: [i32; 64],
    #[serde(with = "BigArray")]
    pub mg_queen: [i32; 64],
    #[serde(with = "BigArray")]
    pub eg_queen: [i32; 64],
    #[serde(with = "BigArray")]
    pub mg_king: [i32; 64],
    #[serde(with = "BigArray")]
    pub eg_king: [i32; 64],
    pub piece_base_values: [i32; 6],
}

impl EvalConfig {
    /// Loads the configuration from a JSON file using serde, with anyhow error handling.
    pub fn load() -> anyhow::Result<Self> {
        let path = "data/evaluation.json";
        let data = fs::read_to_string(path)
            .with_context(|| format!("Could not find evaluation file at {}", path))?;
        let config: EvalConfig = serde_json::from_str(&data)?;
        Ok(config)
    }

    /// Function to streamline retrieval of specific struct entries for piece value configurations.
    pub fn get_array(&self, piece: Piece, mg_flag: bool) -> &[i32; 64] {
        match piece {
            Piece::Pawn => {
                if mg_flag {
                    &self.mg_pawn
                } else {
                    &self.eg_pawn
                }
            }
            Piece::Knight => {
                if mg_flag {
                    &self.mg_knight
                } else {
                    &self.eg_knight
                }
            }
            Piece::Bishop => {
                if mg_flag {
                    &self.mg_bishop
                } else {
                    &self.eg_bishop
                }
            }
            Piece::Rook => {
                if mg_flag {
                    &self.mg_rook
                } else {
                    &self.eg_rook
                }
            }
            Piece::Queen => {
                if mg_flag {
                    &self.mg_queen
                } else {
                    &self.eg_queen
                }
            }
            Piece::King => {
                if mg_flag {
                    &self.mg_king
                } else {
                    &self.eg_king
                }
            }
        }
    }
    /// Returns material base values.
    pub fn get_value(&self, piece: Piece) -> i32 {
        match piece {
            Piece::Pawn => self.piece_base_values[0],
            Piece::Knight => self.piece_base_values[1],
            Piece::Bishop => self.piece_base_values[2],
            Piece::Rook => self.piece_base_values[3],
            Piece::Queen => self.piece_base_values[4],
            Piece::King => self.piece_base_values[5],
        }
    }
}
