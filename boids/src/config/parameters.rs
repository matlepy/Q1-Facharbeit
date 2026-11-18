use crate::config::boundary::Boundary;
#[derive(Clone, Copy)]
pub struct Parameters<B: Boundary> {
    pub boid_count: u16,
    pub cohesion_weight: f32,
    pub alignment_weight: f32,
    pub separation_weight: f32,
    pub boundary_weight: f32,
    pub safe_margin: f32,
    pub time_step: f32,
    pub boundary: B,
} // general Rule for these should be: separation > alignment ca. == cohesion
