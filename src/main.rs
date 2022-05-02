use macroquad::prelude::*;
use planet::Planet;
use universal_time::UniversalTime;

mod planet;
mod planet_data;
mod universal_time;

const TIME_SPEED: i64 = 1000;
const MOVE_SPEED: f32 = 100.1;
const ROTATION_SPEED: f32 = 1.1;
const LOOK_SPEED: f32 = 0.1;

#[macroquad::main("3D")]
async fn main() {
    let mut position = vec3(-100.0, 50.0, 10.0);
    let mut time = UniversalTime::from_now();

    // let mercury = Planet {
    //     name: String::from("Mercury"),
    //     color: ORANGE,
    //     pos: vec3(0., 0., 0.),
    //     mass: 0.33,
    //     orbitRadius: 57.9,
    //     radius: 4.8 / 2.,
    //     orbitPeriod: 88.0,
    //     rotationPeriod: 1407.6,
    // };

    // let venus = Planet {
    //     name: String::from("Venus"),
    //     color: GREEN,
    //     pos: vec3(0., 0., 0.),
    //     mass: 4.87,
    //     orbitRadius: 108.9,
    //     radius: 12.1 / 2.,
    //     orbitPeriod: 224.7,
    //     rotationPeriod: -5832.5,
    // };

    // let earth = Planet {
    //     name: String::from("Eart"),
    //     color: BLUE,
    //     pos: vec3(0., 0., 0.),
    //     mass: 5.97,
    //     orbitRadius: 149.6,
    //     radius: 12.7 / 2.,
    //     orbitPeriod: 365.2,
    //     rotationPeriod: 23.9,
    // };

    // let mars = Planet {
    //     name: String::from("Mars"),
    //     color: RED,
    //     pos: vec3(0., 0., 0.),
    //     mass: 0.642,
    //     orbitRadius: 228.6,
    //     radius: 6.792 / 2.,
    //     orbitPeriod: 687.0,
    //     rotationPeriod: 24.7,
    // };

    let planets = vec![mercury, venus, earth, mars];
    loop {
        time.add_time(TIME_SPEED);
        let delta = get_frame_time();
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            position.y += MOVE_SPEED * delta;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            position.y -= MOVE_SPEED * delta;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            position.x += MOVE_SPEED * delta;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            position.x -= MOVE_SPEED * delta;
        }
        if is_key_down(KeyCode::Space) || is_key_down(KeyCode::Q) {
            position.z += MOVE_SPEED * delta;
        }
        if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::E) {
            position.z -= MOVE_SPEED * delta;
        }

        clear_background(BLACK);

        // Going 3d!

        set_camera(&Camera3D {
            // position: vec3(-20., 15., 0.),
            position,
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);
        for planet in planets.iter() {
            draw_sphere_wires(
                vec3(planet.orbitRadius, 0., 0.),
                planet.radius,
                None,
                planet.color,
            );
        }
        draw_sphere_wires(vec3(0., 0., 0.), 10., None, YELLOW);

        set_default_camera();
        draw_text(&time.to_iso_string(), 10.0, 20.0, 30.0, WHITE);

        next_frame().await
    }
}
