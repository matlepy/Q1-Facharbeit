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

pub trait Boundary {
    fn get_steering_force(&self, safe_margin: f32, boid: &Boid) -> Vec3;
    fn is_inside(&self, position: Vec3) -> bool;
    fn draw(&self);
    
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}
impl Boundary for Sphere {

    fn get_steering_force(&self, safe_margin: f32, boid: &Boid) -> Vec3 {
        let to_boid = boid.position - self.center;
        let distance_to_center = to_boid.length();
        let edge_distance = self.radius - distance_to_center;

        if edge_distance >= safe_margin {
            return vec3(0., 0., 0.);
        }
        let t = 1. - (edge_distance.max(0.0) / safe_margin).clamp(0.0, 1.0);
        let desired = (-to_boid / distance_to_center.max(1e-6)) * t * boid.boid_conf.max_velocity;
        (desired - boid.current_velocity).clamp_length_max(boid.boid_conf.max_force)
    }

    fn is_inside(&self, position: Vec3) -> bool {
        if position.distance(self.center) > self.radius {
            return false;
        } else {
            return true;
        }
    }

    fn draw(&self) {
        draw_sphere_wires(self.center, self.radius, None, DARKBLUE);
    }
}

impl Cube {
    pub fn new(origin: Vec3, size: f32) -> Self {
        Cube { origin, size }
    }
}

impl Boundary for Cube {

    fn get_steering_force(&self, safe_margin: f32, boid: &Boid) -> Vec3 {
        // calculating distance to all planes of the cube and checking if they're in the margin
        let boid_pos = boid.position.to_array();
        let origin_pos = self.origin.to_array();

        let mut desired = boid.current_velocity;
        let mut active = false;

        for i in 0..3 {
            let low = boid_pos[i] - origin_pos[i];
            let high = origin_pos[i] + self.size - boid_pos[i];

            let (dist, sign) = if low <= high { (low, 1.) } else { (high, -1.) };

            if dist < safe_margin {
                active = true;
                //scales with distance to wall, not just on/off on margin edge
                let t = 1. - (dist / safe_margin).clamp(0.0, 1.0);
                desired[i] = sign * t * boid.boid_conf.max_velocity;
            }
        }

        if !active {
            return vec3(0., 0., 0.);
        }

        (desired - boid.current_velocity).clamp_length_max(boid.boid_conf.max_force)
    }

    fn is_inside(&self, position: Vec3) -> bool {
        for (p, o) in position
            .to_array()
            .iter()
            .zip(self.origin.to_array().iter())
        {
            if p > &(o + self.size) || p < o {
                return false;
            }
        }

        return true;
    }

    fn draw(&self) {
        draw_cube_wires(
            self.origin + self.size / 2.,
            vec3(self.size, self.size, self.size),
            DARKBLUE,
        );
    }
}
