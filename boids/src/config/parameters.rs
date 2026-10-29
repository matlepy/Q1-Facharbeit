#[derive(Clone, Copy)]
pub struct Parameters {
    pub cohesion_weight: f32,
    pub alignment_weight: f32,
    pub separation_weight: f32,
    pub boundary_weight: f32,
    pub safe_margin: f32,
} // general Rule for these should be: separation > alignment ca. == cohesion
