//! English-learning domain contracts for Nuntius.
//!
//! Discord transport, persistence, speech models, and LLM backends are kept
//! outside this crate. This crate owns deterministic policy that must remain
//! consistent regardless of the selected backend.

pub mod channels;
pub mod interest;
pub mod model;
pub mod planner;
pub mod pronunciation;
