use crate::config::boid_config::BoidConfig;
use crate::config::boundary::Boundary;
use crate::config::parameters::Parameters;
use crate::debug_tools::warning_handler::WarningHandler;

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
    pub inside_boundary: bool, //DEBUG bool to check if boid is inside boundary
}

impl Boid {
    pub fn new(
        starting_pos: Vec3,
        starting_vel: Vec3,
        boid_conf: &BoidConfig,
        id: u16
    ) -> Self {
        Boid {
            id,
            position: starting_pos,
            current_velocity: starting_vel,
            next_velocity: vec3(0.0, 0.0, 0.0),
            boid_conf: boid_conf.clone(),
            inside_boundary: true, //assuming starting position is inside boundary
        }
    }

    //converting desired velocity to steering force, which is the difference between desired and current velocity, capped at max_force.
    fn calc_steering_force(&self, desired: Vec3) -> Vec3 {
        let mut steering_force = desired - self.current_velocity;
        steering_force =
            steering_force.clamp_length(-(self.boid_conf.max_force), self.boid_conf.max_force);

        steering_force
    }

    //functions for calculating desired velocity for each of the three rules of boids, cohesion, alignment and separation.
    fn calc_desired_cohesion(&self, boids: &Vec<Boid>) -> Vec3 {
        //calculating center of all boids
        let mut count = 0;
        let mut sum = vec3(0.0, 0.0, 0.0);
        for b in boids {
            if (b.id != self.id)
                && (self.position.distance(b.position) < self.boid_conf.view_radius)
            {
                sum += b.position;
                count += 1;
            }
        }
        if count == 0 {
            return vec3(0., 0., 0.);
        }
        let center = sum / (count as f32);
        let desired = (center - self.position).normalize_or_zero() * self.boid_conf.max_velocity;

        desired
    }

    fn calc_desired_alignment(&self, boids: &Vec<Boid>) -> Vec3 {
        //calculating average normalized direction of every boid
        let mut count = 0;
        let mut sum: Vec3 = vec3(0.0, 0.0, 0.0);
        for b in boids {
            if (b.id != self.id)
                && (self.position.distance(b.position) < self.boid_conf.view_radius)
            {
                sum += b.current_velocity.normalize_or_zero();
                count += 1;
            }
        }
        if count == 0 {
            return vec3(0., 0., 0.);
        }
        let avg_velocity = sum / (count as f32);
        let desired = avg_velocity.normalize_or_zero() * self.boid_conf.max_velocity;
        desired
    }

    fn calc_desired_separation(&self, boids: &Vec<Boid>) -> Vec3 {
        //calculating the avg of vectors pointiung away from other boids scaling with distance
        let mut count = 0;
        let mut sum: Vec3 = vec3(0.0, 0.0, 0.0);
        for b in boids {
            if (b.id != self.id)
                && (self.position.distance(b.position) < self.boid_conf.view_radius)
            {
                let diff = self.position - b.position;
                let distance = self.position.distance(b.position);
                let distance = distance.max(1e-3);
                sum += diff.normalize_or_zero() / distance.powi(2); //force scales with distance: small distance -> large force, big distance -> almost no force.
                count += 1;
            }
        }
        if count == 0 {
            return vec3(0., 0., 0.);
        }
        let desired = (sum / (count as f32)).normalize_or_zero() * self.boid_conf.max_velocity;
        desired
    }

    pub fn calc_next_velocity<B: Boundary>(&mut self, boids: &Vec<Boid>, param: &Parameters<B>) {
        self.next_velocity = {
            let cohesion = self.calc_desired_cohesion(boids);
            let alignment = self.calc_desired_alignment(boids);
            let separation = self.calc_desired_separation(boids);

            let f_cohesion = self.calc_steering_force(cohesion);
            let f_alignment = self.calc_steering_force(alignment);
            let f_separation = self.calc_steering_force(separation);
            let f_boundary = param.boundary.get_steering_force(param.safe_margin, self);

            let f_comb = (param.cohesion_weight * f_cohesion)
                + (param.alignment_weight * f_alignment)
                + (param.separation_weight * f_separation)
                + (param.boundary_weight * f_boundary);

            let acceleration = f_comb / self.boid_conf.mass;

            let v_new = self.current_velocity + acceleration * param.time_step;

            v_new.clamp_length(-self.boid_conf.max_velocity, self.boid_conf.max_velocity)
        };
    }

    pub fn update<B: Boundary> (&mut self, param: &Parameters<B>, handler: &WarningHandler<B>) {
        self.current_velocity = self.next_velocity.clone();
        self.position = self.position + (self.current_velocity * param.time_step);

        //DEBUG tmp
        self.inside_boundary = handler.check(self);

        draw_cube(self.position, vec3(0.5, 0.5, 0.5), None, RED);
    }
}
