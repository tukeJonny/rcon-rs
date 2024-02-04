//! Rust implementation of SRCDS RCON Protocol.
//!
//! Reference: https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
//!
//! Example implementations:
//! - Palworld `Info` command execution example: [palworld.rs](https://github.com/tukeJonny/rcon-rs/blob/master/examples/palworld.rs)
#![warn(missing_docs)]
extern crate anyhow;
extern crate num_derive;

/// This module provides RCON client traits.
pub mod client;

/// This module provides error types caused by RCON client.
pub mod error;
mod packet;
