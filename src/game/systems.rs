use consts;
use game;
use game::components::*;
use std::f64::consts::FRAC_PI_2;

pub fn process_input(game: &mut game::Game) {
    if game.input.mouse_left {
        // spawn dummy entities
        game::create_dummy_entity(game);
    }

    // use mouse scroll to zoom
    game.camera.adjust_zoom(consts::ZOOM_FACTOR.powf(game.input.mouse_scroll[1]));

    const BASE_ACC: f64 = 20.0;
    let direction = game.input.keyboard_direction();
    let cursor_angle = f64::atan2(
        game.input.mouse_position[0] - consts::WINDOW_SIZE[0] as f64 / 2.0,
        -(game.input.mouse_position[1] - consts::WINDOW_SIZE[1] as f64 / 2.0),
    ) - FRAC_PI_2;
    
    let center_point = {
        let player = game.player_mut();
        player.acceleration = Acceleration::new(direction.dx * BASE_ACC, direction.dy * BASE_ACC);
        player.rotation = Rotation(cursor_angle);
        player.position.0
    };

    // propagate mouse position to camera
    game.camera.center_at(center_point);
}

pub fn process_physics(game: &mut game::Game) {
    let dt = game.clock.delta;
    for option_entity in game.entities.iter_mut() {
        if let Some(entity) = option_entity {
            entity.velocity.0.dx += entity.acceleration.0.dx * dt;
            entity.velocity.0.dy += entity.acceleration.0.dy * dt;

            entity.position.0.x += entity.velocity.0.dx * dt;
            entity.position.0.y += entity.velocity.0.dy * dt;

            entity.rotation.0 += entity.angular_velocity.0 * dt;
        }
    }
}
