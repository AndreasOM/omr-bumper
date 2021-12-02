//! omr-bumper is not intended to be a library, but it can be embedded into your code
//!
//! Run `omr-bumper --help` for more info
//!
//! *Note:* This is work-in-progress, and _works for us_. Use with care!
//!
//! Sensible pull-requests, and issues welcome at: (https://github.com/AndreasOM/omr-bumper)


mod release;
pub use release::Release as Release;


mod cargo;
mod manifest;
mod repository;

