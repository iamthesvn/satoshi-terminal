// volumes/mod.rs — Satoshi's Terminal story content, chapter data, and public API.
// All story content lives in story.rs, declared below so `crate::volumes::Chapter` etc. resolve correctly.

pub mod story;
pub use story::{Chapter, Volume, all_volumes, rank_title};
