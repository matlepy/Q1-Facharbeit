use boids::boid::Boid;
use boids::config::BoidConfig;
use macroquad::miniquad::start;
use macroquad::prelude::*;

#[macroquad::main("3D Boids")]
async fn main() {
    let camera_target = vec3(0.0, 0.0, 0.0);
    let sensitivity: f32 = 0.5;
    let radius: f32 = 30.0;
    let mut yaw = 0.0f32;
    let mut pitch = 0.2f32;

    let time_step: f32 = 0.1;

    let standard_conf = BoidConfig::new(2.0);
    //creating N boid objects in Vector
    let mut boids: Vec<Boid> = vec!();
    for i in 0..10 {
        let starting_pos = vec3(real_rand::random_range(0..10) as f32, real_rand::random_range(0..10) as f32, real_rand::random_range(0..10) as f32);
        let starting_vel = vec3(real_rand::random_range(0..10) as f32, real_rand::random_range(0..10) as f32, real_rand::random_range(0..10) as f32);
        boids.push(Boid::new(starting_pos, starting_vel, &standard_conf, i))
    }

    loop {
        clear_background(BLACK);
        
        if is_mouse_button_down(MouseButton::Left) {
            yaw += mouse_delta_position().x * sensitivity;
            pitch -= mouse_delta_position().y * sensitivity;

            pitch = pitch.clamp(-1.5, 1.5);
        }

        let cam_x = camera_target.x + radius * pitch.cos() * yaw.sin();
        let cam_y = camera_target.y + radius * pitch.sin();
        let cam_z = camera_target.z + radius * pitch.cos() * yaw.cos();
        
        let camera = Camera3D {
            position: vec3(cam_x, cam_y, cam_z),
            target: camera_target,
            up: vec3(0.0, 0.1, 0.0),
            ..Default::default()
        };
        set_default_camera();
        set_camera(&camera);
        
        let prev_boids = boids.clone();
        for b in 0..boids.len() {
            boids[b].update(&prev_boids, time_step);
        }
        
        next_frame().await;
    }
}