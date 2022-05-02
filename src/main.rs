use macroquad::prelude::*;
use planet::Planet;
use universal_time::UniversalTime;

mod planet;
mod planet_data;
mod universal_time;

const UNIVERSAL_SCALE_DEVIDER: f32 = 1000.0;
const PLANET_RADIUS_SCALE_DEVIDER: f32 = 1000.0;
const TIME_SPEED: i64 = 1000;
const MOVE_SPEED: f32 = 100.1;
const LOOK_SPEED: f32 = 0.1;

#[macroquad::main("3D")]
async fn main() {
    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up: Vec3;

    let mut position = vec3(100.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    let mut position = vec3(100.0, 50.0, 10.0);
    let mut time = UniversalTime::from_now();

    let planets = vec![
        Planet::SUN,
        Planet::MERCURY,
        Planet::VENUS,
        Planet::EARTH,
        Planet::MARS,
    ];
    loop {
        time.add_time(TIME_SPEED);
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            position += front * MOVE_SPEED * delta;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            position -= front * MOVE_SPEED * delta;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            position -= right * MOVE_SPEED * delta;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            position += right * MOVE_SPEED * delta;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * delta * LOOK_SPEED;
        pitch += mouse_delta.y * delta * -LOOK_SPEED;

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };

        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }

        clear_background(BLACK);

        // Going 3d!
        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });

        // draw_grid(200, 1., BLACK, GRAY);
        for planet in planets.iter() {
            let pos = vec3(planet.planet_data.distance_from_sun, 0., 0.);
            draw_sphere_wires(
                pos,
                (planet.planet_data.diameter / 2.0) / PLANET_RADIUS_SCALE_DEVIDER,
                None,
                planet.color,
            );
        }

        set_default_camera();
        draw_text(&time.to_iso_string(), 10.0, 20.0, 30.0, WHITE);

        next_frame().await
    }
}
