use macroquad::prelude::*;

use crate::boid::Boid;

#[derive(Clone)]
pub struct Cube {
    origin: Vec3,
    size: f32,
}
#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

#[derive(Clone)]
pub enum Boundary {
    Cube(Cube),
    Sphere(Sphere),
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }

    pub fn get_steering_force(&self, safe_margin: f32, boid: &Boid) -> Vec3 {
        let distance_to_center = (boid.position - self.center).length();

        if distance_to_center > (self.radius - safe_margin) {
            let desired = (self.center - boid.position).normalize_or_zero() * boid.boid_conf.max_velocity;
            return desired - boid.current_velocity;
        } else {
            return vec3(0.0, 0.0, 0.0);
        }
    }

    pub fn is_inside(&self, position: Vec3) -> bool {
        if position.distance(self.center) > self.radius {
            return false;
        } else {
            return true;
        }
    }

    pub fn draw(&self){
        draw_sphere_wires(self.center, self.radius, None, DARKBLUE);
    }
}

impl Cube {
    pub fn new(origin: Vec3, size: f32) -> Self {
        Cube { origin, size }
    }

    pub fn get_steering_force(&self, safe_margin: f32, boid: &Boid) -> Vec3 {
        // calculating distance to all planes of the cube and checking if they're in the margin
        let boid_pos = boid.position.to_array();
        let origin_pos = self.origin.to_array();

        
        let steering_arr = std::array::from_fn(|i| {
            let distance = boid_pos[i] - origin_pos[i];
            if distance < safe_margin || (self.size - distance) < safe_margin {
                let mut s = 5. / distance.powi(2);
                if distance > (self.size / 2.) {
                    s *= -1.;
                }
                s
            } else {
                0.0
            }
        });

        
        let steering = Vec3::from(steering_arr); 
        
        let desired = steering.normalize_or_zero() * boid.boid_conf.max_velocity;
        let mut steering_force = desired - boid.current_velocity;
        
        return steering_force.clamp_length(-boid.boid_conf.max_force, boid.boid_conf.max_force);
    }

    pub fn is_inside(&self, position: Vec3) -> bool {
        let mut inside: bool = true;
        for (p, o) in position
            .to_array()
            .iter()
            .zip(self.origin.to_array().iter())
        {
            if p > &(o + self.size) || p < o {
                inside = false;
            }
        }

        return inside;
    }

    pub fn draw(&self){
        
        draw_cube_wires(self.origin, vec3(self.size, self.size, self.size) - self.origin, DARKBLUE); 

    }
}
