use boids::boid::Boid;
use boids::config::boid_config::BoidConfig;
use boids::config::boundary::*;
use boids::config::parameters::Parameters;
use boids::debug_tools::warning_handler::WarningHandler;
use log::{info, warn};
use macroquad::prelude::*;

#[macroquad::main("3D Boids")]
async fn main() {
    let boundary_size = 100.0;

    let _env_log = env_logger::init();
    let camera_target = vec3(0.0, 0.0, 0.0);
    let sensitivity: f32 = 0.5;
    let radius: f32 = &boundary_size * 1.7;
    let mut yaw = 0.0f32;
    let mut pitch = 0.2f32;

    /*
    let s = Sphere::new(vec3(0.0, 0.0, 0.0), boundary_size/2.0);
    let boundary = Boundary::Sphere(s);
    */
    //*
    let c = Cube::new(
        vec3(
            -boundary_size / 2.0,
            -boundary_size / 2.0,
            -boundary_size / 2.0,
        ),
        boundary_size,
    );
    let boundary = Boundary::Cube(c);
    //*/
    let warn_handler = WarningHandler::new(boundary.clone(), false);

    let standard_conf = BoidConfig {
        view_radius: 25.0,
        view_angle: 360.0,
        max_velocity: 5.0,
        max_force: 10.0,
        mass: 1.0,
    };

    let param = Parameters {
        // used to easily modify parameters for later testing
        boid_count: 1000,
        cohesion_weight: 1.0,
        alignment_weight: 1.5,
        separation_weight: 1.,
        boundary_weight: 1.,
        safe_margin: 15.0,
        time_step: get_frame_time(),
    };

    //creating N boid objects in Vector
    let starting_range = 20.0;
    let mut boids: Vec<Boid> = vec![];
    for i in 0..param.boid_count {
        let starting_pos = vec3(
            real_rand::random_range(-starting_range..starting_range) as f32,
            real_rand::random_range(-starting_range..starting_range) as f32,
            real_rand::random_range(-starting_range..starting_range) as f32,
        );
        let starting_vel = vec3(
            real_rand::random_range(-10..10) as f32,
            real_rand::random_range(-10..10) as f32,
            real_rand::random_range(-10..10) as f32,
        );
        boids.push(Boid::new(
            starting_pos,
            starting_vel,
            &standard_conf,
            i,
            &boundary,
        ))
    }

    loop {
        clear_background(BLACK);
        match &boundary {
            Boundary::Cube(c) => c.draw(),
            Boundary::Sphere(s) => s.draw(),
        }

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

        //info!(target: "boid_positions", "starting calculations");
        //current solution to above
        let old_boids = boids.clone();
        for b in 0..boids.len() {
            boids[b].calc_next_velocity(&old_boids, param);
        }
        //info!("updating boid positions");
        for b in 0..boids.len() {
            boids[b].update(param, &warn_handler);
        }

        next_frame().await;
    }
}
