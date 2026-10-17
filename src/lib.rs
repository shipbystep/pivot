//! Build pivot: an Excel-style pivot table in Rust, step by step.
//!
//! Layers, top to bottom. Code only calls down (see docs/design.md):
//! `app` > `data` > `model`.

pub mod app;
pub mod data;
pub mod model;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
