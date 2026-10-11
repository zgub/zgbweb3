//! Pure simulation and maths logic for zgbweb3.
//!
//! Rule of the house: nothing in here knows about egui, wgpu or the browser.
//! Every topic keeps its numbers and its physics here so they can be unit-tested
//! natively (`cargo test -p sims`) and reused by any front end.

pub mod energy;
pub mod fmt;
pub mod scale;
pub mod stats;
