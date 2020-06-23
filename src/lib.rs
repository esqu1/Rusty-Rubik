//! A crate used to model the Rubik's Cube.
//!
//! The crate includes consists of two separate compartments:
//!
//! - An **executable** that allows you to instantly search for a solution to a
//! configuration of the Rubik's Cube.
//!
//! - A **library** that provides utility functions for solver methods, pruning table
//! generation, and an API for Rubik's Cube structure.
//!
//!

pub mod cube;
pub mod parser;
pub mod pruning;
pub mod solver;
