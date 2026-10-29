use crate::config::boid_config::BoidConfig;
use crate::config::boundary::Boundary;
use crate::config::parameters::Parameters;
use log::{info, warn};

use macroquad::prelude::*;

//Struct and Implementation of one boid/bird
#[derive(Clone)]
pub struct Boid {
    pub id: u16,
    pub position: Vec3,         //simple current 3D position
    pub current_velocity: Vec3, //current velocity value and 3D direction
    next_velocity: Vec3,
    pub boid_conf: BoidConfig, //next calculated velocity and direction
    //current is only read by other boids and next is written to.
    //this is a solution to the borrow checker and makes later async much easier, since all boids take a step at the same time and only change velocity when everything is calculated.
    boundary: Boundary,
}

impl Boid {
    pub fn new(
        starting_pos: Vec3,
        starting_vel: Vec3,
        boid_conf: &BoidConfig,
        id: u16,
        boundary: &Boundary,
    ) -> Self {
        Boid {
            id,
            position: starting_pos,
            current_velocity: starting_vel,
            next_velocity: vec3(0.0, 0.0, 0.0),
            boid_conf: boid_conf.clone(),
            boundary: boundary.clone(),
        }
    }

    fn calc_desired_cohesion(&self, boids: &Vec<Boid>) -> Vec3 {
        //calculating center of all boids
        //TODO add viewing range and possibly viewing field
        let mut sum = vec3(0.0, 0.0, 0.0);
        for b in boids {
            if b.id != self.id {
                sum += b.position;
            }
        }
        let center = sum / (boids.len() as f32 - 1.0);
        let desired = (center - self.position).normalize_or_zero() * self.boid_conf.max_velocity;

        desired
    }

    fn calc_steering_force(&self, desired: Vec3) -> Vec3 {
        let mut steering_force = desired - self.current_velocity;
        steering_force =
            steering_force.clamp_length(-(self.boid_conf.max_force), self.boid_conf.max_force);

        steering_force
    }

    fn calc_desired_alignment(&self, boids: &Vec<Boid>) -> Vec3 {
        //calculating average normalized direction of every boid
        //TODO add viewing range and possibly viewing field
        let mut sum: Vec3 = vec3(0.0, 0.0, 0.0);
        for b in boids {
            if b.id != self.id {
                sum += b.current_velocity.normalize_or_zero();
            }
        }
        let avg_velocity = sum / (boids.len() as f32 - 1.0);
        let desired = avg_velocity.normalize_or_zero() * self.boid_conf.max_velocity;
        desired
    }

    fn calc_desired_separation(&self, boids: &Vec<Boid>) -> Vec3 {
        //calculating the avg of vectors pointiung away from other boids scaling with distance
        //TODO add viewing range and possibly viewing field as well as a factor
        let mut sum: Vec3 = vec3(0.0, 0.0, 0.0);
        for b in boids {
            if b.id != self.id {
                let diff = self.position - b.position;
                let distance = self.position.distance(b.position);
                sum += diff.normalize_or_zero() / distance.powi(2);
            }
        }
        let desired = sum / (boids.len() as f32 - 1.0) * self.boid_conf.max_velocity;
        desired
    }

    pub fn calc_next_velocity(&mut self, boids: &Vec<Boid>, param: Parameters) {
        self.next_velocity = {
            let cohesion = self.calc_desired_cohesion(boids);
            let alignment = self.calc_desired_alignment(boids);
            let separation = self.calc_desired_separation(boids);

            let f_cohesion = self.calc_steering_force(cohesion);
            let f_alignment = self.calc_steering_force(alignment);
            let f_separation = self.calc_steering_force(separation);
            let f_boundary = match &self.boundary {
                Boundary::Sphere(s) => s.get_steering_force(param.safe_margin, &self),
                Boundary::Cube(c) => c.get_steering_force(param.safe_margin, &self),
            };

            let f_comb = (param.cohesion_weight * f_cohesion)
                + (param.alignment_weight * f_alignment)
                + (param.separation_weight * f_separation)
                + (param.boundary_weight * f_boundary);

            let acceleation = f_comb / self.boid_conf.mass;

            let v_new = self.current_velocity + acceleation;

            v_new.clamp_length(-self.boid_conf.max_velocity, self.boid_conf.max_velocity)
        };
    }

    pub fn update(&mut self, time_step: f32) {
        self.current_velocity = self.next_velocity.clone();
        self.position = self.position + (self.current_velocity * time_step);

        info!("boid {} is at position: {}", self.id, self.position);
        match &self.boundary {
            Boundary::Cube(c) => {
                if c.is_inside(self.position) {
                    info!("boid {} is in boundary", self.id);
                } else {
                    warn!("boid {} is not in boundary", self.id)
                }
            }
            Boundary::Sphere(s) => {
                if s.is_inside(self.position) {
                    info!("boid {} is in boundary", self.id);
                } else {
                    warn!("boid {} is not in boundary", self.id)
                }
            }
        }

        draw_cube(self.position, vec3(0.5, 0.5, 0.5), None, RED);
    }
}
