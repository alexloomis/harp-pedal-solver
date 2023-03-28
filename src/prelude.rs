pub use crate::prelude::harp::*;
pub use crate::prelude::note::*;
pub use crate::prelude::pitch_class::*;

// The ways of storing notes are
// Note - a single note suitable for human readability,
// or if enharmonics should be treated differently.
// PichClass - a single note suitible for numeric manipulation,
// or if enharmonics should be treated identically.
// Harp - A collection of upto one note per scale degree.

pub mod harp;
pub mod note;
pub mod pitch_class;
