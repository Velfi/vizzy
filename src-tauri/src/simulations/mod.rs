//! # Simulations Module
//!
//! Contains the collection of interactive simulations that form the core of the
//! Vizzy application. Each simulation explores different aspects of complex
//! systems, from particle physics to emergent behavior and pattern formation.
//!
//! ## Simulation Philosophy
//!
//! The simulations are designed to reveal the beauty and complexity that emerges
//! from simple rules. By providing interactive exploration of mathematical
//! systems, they offer both aesthetic experiences and insights into how
//! complexity arises from simplicity.
//!
//! ## Available Experiences
//!
//! Each simulation offers a unique perspective on complex systems:
//! - **Particle Physics**: Gravitational interactions and orbital dynamics
//! - **Emergent Behavior**: Simple rules creating complex patterns
//! - **Reaction-Diffusion**: Chemical pattern formation and evolution
//! - **Agent Systems**: Collective behavior and pathfinding
//! - **Flow Visualization**: Vector fields and particle advection
//!
//! ## Design Principles
//!
//! All simulations share common design principles that ensure consistent
//! user experience while allowing each to explore its unique domain.
//! The unified interface enables users to seamlessly transition between
//! different types of complex system exploration.

pub mod ecosystem;
pub mod flow;
pub mod gradient;
pub mod gray_scott;
pub mod main_menu;
pub mod particle_life;
pub mod pellets;
pub mod shared;
pub mod slime_mold;
pub mod traits;
