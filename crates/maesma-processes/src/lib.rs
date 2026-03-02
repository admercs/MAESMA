//! Concrete process implementations for all 11 MAESMA process families.
//!
//! Each family module provides implementations at multiple fidelity rungs
//! (R0–R3), from simple empirical parameterizations to full physics-based models.

pub mod atmosphere;
pub mod biogeochemistry;
pub mod cryosphere;
pub mod ecology;
pub mod evolution;
pub mod fire;
pub mod human_systems;
pub mod hydrology;
pub mod ocean;
pub mod radiation;
pub mod trophic_dynamics;
