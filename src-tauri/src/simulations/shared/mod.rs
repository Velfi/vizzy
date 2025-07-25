//! # Shared Simulation Components
//!
//! Common utilities and systems that provide consistent behavior across all
//! simulations in the Vizzy application. These components ensure a unified
//! user experience while reducing code duplication and maintenance overhead.
//!
//! ## Design Philosophy
//!
//! The shared components are designed to provide essential functionality
//! that every simulation needs, while remaining flexible enough to support
//! the unique requirements of each simulation type. This approach ensures
//! consistency without sacrificing the individuality of each simulation.
//!
//! ## Component Areas
//!
//! The shared components cover the fundamental aspects of interactive
//! simulation: user interaction, visual presentation, and system
//! management. Each area provides both basic functionality and
//! advanced features for sophisticated simulation experiences.

pub mod camera;
pub mod coordinates;
pub mod lut;
pub mod position_generators;

pub use lut::{LutData, LutManager, SimulationLutManager};
pub use position_generators::{PositionGenerator, SlimeMoldPositionGenerator};
