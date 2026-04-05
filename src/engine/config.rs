use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::fs;
use anyhow::{Context, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct EvalConfig {
    #[serde(with = "BigArray")]
    pub mg_pawn : [i32; 64],
    #[serde(with = "BigArray")]
    pub eg_pawn : [i32; 64],
    #[serde(with = "BigArray")]
    pub mg_knight : [i32; 64],
    #[serde(with = "BigArray")]
    pub eg_knight : [i32; 64],
    #[serde(with = "BigArray")]
    pub mg_bishop : [i32; 64],
    #[serde(with = "BigArray")]
    pub eg_bishop : [i32; 64],
    #[serde(with = "BigArray")]
    pub mg_rook : [i32; 64],
    #[serde(with = "BigArray")]
    pub eg_rook : [i32; 64],
    #[serde(with = "BigArray")]
    pub mg_queen : [i32; 64],
    #[serde(with = "BigArray")]
    pub eg_queen : [i32; 64],
    #[serde(with = "BigArray")]
    pub mg_king : [i32; 64],
    #[serde(with = "BigArray")]
    pub eg_king : [i32; 64],
}

impl EvalConfig {
    pub fn load() -> anyhow::Result<Self> {
        let path = "data/evaluation.json";
        let data = fs::read_to_string(path).with_context(||format!("Could not find evaluation file at {}", path))?;
        let config: EvalConfig  = serde_json::from_str(&data)?;
        Ok(config)
    }
}