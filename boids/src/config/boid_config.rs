#[derive(Clone)]
pub struct BoidConfig {
    pub view_distance: f32,
    pub view_angle: f32, //limiting angle a boid can see other boids
    pub max_velocity: f32,
    pub max_force: f32, //Maximum Force the boid can experience, above will be capped
    pub mass: f32,      //Mass of the boid in kg for acceleration
}

impl BoidConfig {
    pub fn new(max_velocity: f32, max_force: f32, mass: f32) -> Self {
        BoidConfig {
            view_distance: 0.0,
            view_angle: 360.0,
            max_velocity,
            max_force,
            mass,
        }
    }
}
