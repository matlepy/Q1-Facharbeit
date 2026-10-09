#[derive(Clone)]
pub struct BoidConfig {
    pub view_distance: isize,
    pub max_velocity: f32,
    pub max_force: f32, 
}

impl BoidConfig {
    
    pub fn new(max_velocity: f32) -> Self{
        BoidConfig {
            max_velocity,
            max_force: 0.0,
            view_distance: 0,
        }
    }
}