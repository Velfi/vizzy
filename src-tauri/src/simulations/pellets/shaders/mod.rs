//! # Pellets Shaders Module
//!
//! Contains the GPU programs that drive the Pellets simulation, implementing
//! both the physics calculations and visual rendering. These shaders transform
//! simple mathematical rules into complex emergent behaviors.
//!
//! ## Shader Philosophy
//!
//! The shaders implement the core simulation logic through parallel computation.
//! By processing all particles simultaneously on the GPU, the simulation can
//! handle thousands of interacting particles while maintaining real-time
//! performance and responsive user interaction.
//!
//! ## Computational Approach
//!
//! The simulation uses a multi-stage approach: spatial partitioning for efficient
//! neighbor lookups, compute shaders handle the physics calculations that determine
//! particle behavior, while render shaders create the visual representation.

pub const RENDER_SHADER: &str = include_str!("render.wgsl");
pub const PARTICLE_VERTEX_SHADER: &str = include_str!("particle_vertex.wgsl");
pub const PARTICLE_FRAGMENT_SHADER: &str = include_str!("particle_fragment.wgsl");
pub const PHYSICS_COMPUTE_SHADER: &str = include_str!("physics_compute.wgsl");
pub const DENSITY_COMPUTE_SHADER: &str = include_str!("density_compute.wgsl");
pub const GRID_CLEAR_SHADER: &str = include_str!("grid_clear.wgsl");
pub const GRID_POPULATE_SHADER: &str = include_str!("grid_populate.wgsl");
