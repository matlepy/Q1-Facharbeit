use crate::config::BoidConfig;

use macroquad::prelude::*;

//Struct and Implementation of one boid/bird
#[derive(Clone)]
pub struct Boid {
    pub id: u16,
    pub position: Vec3, //simple current 3D position
    pub current_velocity: Vec3, //current velocity value and 3D direction
    next_velocity: Vec3,
    boid_conf: BoidConfig, //next calculated velocity and direction
    //current is only read by other boids and next is written to. 
    //this is a solution to the borrow checker and makes later async much easier, since all boids take a step at the same time and only change velocity when everything is calculated.
}

impl Boid {

    pub fn new(starting_pos: Vec3, starting_vel: Vec3, boid_conf: &BoidConfig, id: u16) -> Self {
        Boid{
            id,
            position: starting_pos,
            current_velocity: starting_vel,
            next_velocity: vec3(0.0,0.0,0.0),
            boid_conf: boid_conf.clone(),
        }
    }
    
    fn calc_desired_cohesion(&self, boids: &Vec<Boid>) -> Vec3 {

        //calculating center of all boids
        //TODO add viewing range and possibly viewing field
        let mut sum = vec3(0.0, 0.0, 0.0);
        for b in boids {
            if b.id != self.id {
                sum += b.position;
        }}
        let center = sum / boids.len() as f32;
        let desired = (center - self.position).normalize() * self.boid_conf.max_velocity;

        desired

    }

    fn calc_desired_alignment(&self, boids: &Vec<Boid>) -> Vec3 {

        //calculating average normalized direction of every boid
        //TODO add viewing range and possibly viewing field
        let mut sum: Vec3 = vec3(0.0, 0.0, 0.0);
        for b in boids {
            if b.id != self.id {
                sum += b.current_velocity.normalize();
        }}
        let avg_velocity = sum / boids.len() as f32;
        let desired = avg_velocity.normalize() * self.boid_conf.max_velocity;
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
                sum += diff.normalize() / distance.powi(2);


            }
        };
        let desired = sum / (boids.len() as f32 - 1.0) * self.boid_conf.max_velocity;
        desired
    }


    pub fn update(&mut self, boids:&Vec<Boid>, time_step: f32 ) {
        self.next_velocity = {
            let cohesion = self.calc_desired_cohesion(boids);
            let alignment = self.calc_desired_alignment(boids);
            let separation = self.calc_desired_separation(boids);
            let comb_direction = (cohesion.normalize() + alignment.normalize() + separation.normalize()) / 3.0;
            ((self.current_velocity.normalize() + comb_direction.normalize()) / 2.0) * self.boid_conf.max_velocity
        };
        self.current_velocity = self.next_velocity.clone();
        self.position = self.position + (self.current_velocity * time_step);

        draw_cube(self.position, vec3(0.5, 0.5, 0.5), None, RED);
    }

}