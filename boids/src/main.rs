use macroquad::prelude::*;

#[macroquad::main("3D Boids")]
async fn main() {
    let camera_target = vec3(0.0, 0.0, 0.0);
    let sensitivity: f32 = 0.5;
    let radius: f32 = 10.0;
    let mut yaw = 0.0f32;
    let mut pitch = 0.2f32;

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
        

        // Für jeden Boid einfach einen Punkt/Würfel zeichnen
        draw_cube(vec3(0.0, 0.0, 0.0), vec3(0.5, 0.5, 0.5), None, RED);
        
        next_frame().await;
    }
}