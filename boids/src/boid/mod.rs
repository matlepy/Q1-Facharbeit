use crate::config::BoidConfig;

use macroquad::prelude::*;

//Struct and Implementation of one boid/bird
struct Boid {
    pub position: Vec3, //simple current 3D position
    pub current_velocity: Vec3, //current velocity value and 3D direction
    next_velocity: Vec3,
    boid_conf: BoidConfig, //next calculated velocity and direction
    //current is only read by other boids and next is written to. 
    //this is a solution to the borrow checker and makes later async much easier, since all boids take a step at the same time and only change velocity when everything is calculated.
}

impl Boid {

    fn new(starting_pos: Vec3, starting_vel: Vec3, boid_conf: BoidConfig) -> Self {
        Boid{
            position: starting_pos,
            current_velocity: starting_vel,
            next_velocity: vec3(0.0,0.0,0.0),
            boid_conf,
        }
    }
    
    fn calc_velocity(&mut self, boids: &Vec<Boid>) {

        //calculating center of all boids
        let mut sum = vec3(0.0, 0.0, 0.0);
        for b in boids {
            sum += b.position;
        }
        let center = sum / boids.len() as f32;
        //calculate vector between current position and center to calculate next direction
        let vec_pc: Vec3 = center - self.position;
        let d = vec_pc.normalize() + self.current_velocity.normalize();
        //use directional vector and Vmax to calculate next velocity
        self.next_velocity = d.normalize() * self.boid_conf.max_velocity;




    }

    fn update(&mut self) {
        self.current_velocity = self.next_velocity.clone();

        draw_cube(self.position, vec3(0.5, 0.5, 0.5), None, RED);
    }

}