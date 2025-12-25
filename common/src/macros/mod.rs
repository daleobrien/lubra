//! Declarative macros for generating shape families
//!
//! This module provides macros to eliminate code duplication in fragment generation.
//! The main macro `define_shape_family!` allows defining a base shape and automatically
//! generating all 6 rotational/reflectional variants.

pub mod shape_family;

// Re-export the main macro
pub use shape_family::*;
