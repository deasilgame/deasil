use consts;
use game;
use game::components::*;
use specs::*;
use std::f64::consts::FRAC_PI_2;

pub struct InputSys;
impl<'a> System<'a> for InputSys {
    type SystemData = (
        Read<'a, Input>,
        Read<'a, Option<Player>>,
        ReadStorage<'a, Position>,
        Write<'a, Camera>,
        WriteStorage<'a, Acceleration>,
        WriteStorage<'a, Rotation>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            input,
            player_entity_storage,
            position_storage,
            mut camera,
            mut acc_storage,
            mut rot_storage,
            entities,
            updater,
        ): Self::SystemData,
    ) {
        if input.mouse_left {
            // spawn dummy entities
            game::create_dummy_entity(updater.create_entity(&entities)).build();
        }

        // use mouse scroll to zoom
        camera.adjust_zoom(consts::ZOOM_FACTOR.powf(input.mouse_scroll[1]));

        const BASE_ACC: f64 = 20.0;
        let mut player_centered_point: Option<Point> = None;
        if let Some(Player(player_entity)) = *player_entity_storage {
            let direction = input.keyboard_direction();
            match acc_storage.insert(
                player_entity,
                Acceleration::new(direction.dx * BASE_ACC, direction.dy * BASE_ACC),
            ) {
                Ok(_) => {}
                Err(e) => println!("Failed to update acceleration: {:?}", e),
            }

            let cursor_angle = f64::atan2(
                input.mouse_position[0] - consts::WINDOW_SIZE[0] as f64 / 2.0,
                -(input.mouse_position[1] - consts::WINDOW_SIZE[1] as f64 / 2.0),
            ) - FRAC_PI_2;
            match rot_storage.insert(player_entity, Rotation(cursor_angle)) {
                Ok(_) => {}
                Err(e) => println!("Failed to update rotation: {:?}", e),
            }

            if let Some(position) = position_storage.get(player_entity) {
                player_centered_point = Some(position.0)
            }
        }

        // propagate mouse position to camera
        camera.center_at(match player_centered_point {
            Some(point) => point,
            None => Point::default(),
        });
    }
}

pub struct AccelerationSys;
impl<'a> System<'a> for AccelerationSys {
    type SystemData = (
        Read<'a, Clock>,
        ReadStorage<'a, Acceleration>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (clock_storage, acc_storage, mut vel_storage): Self::SystemData) {
        let dt = (*clock_storage).delta;
        for (acc, vel) in (&acc_storage, &mut vel_storage).join() {
            vel.0.dx += acc.0.dx * dt;
            vel.0.dy += acc.0.dy * dt;
        }
    }
}

pub struct LinearMovementSys;
impl<'a> System<'a> for LinearMovementSys {
    type SystemData = (
        Read<'a, Clock>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (clock_storage, vel_storage, mut pos_storage): Self::SystemData) {
        let dt = (*clock_storage).delta;
        for (v, p) in (&vel_storage, &mut pos_storage).join() {
            p.0.x += v.0.dx * dt;
            p.0.y += v.0.dy * dt;
        }
    }
}

pub struct AngularMovementSys;
impl<'a> System<'a> for AngularMovementSys {
    type SystemData = (
        Read<'a, Clock>,
        ReadStorage<'a, AngularVelocity>,
        WriteStorage<'a, Rotation>,
    );

    fn run(&mut self, (clock_storage, vel_storage, mut rot_storage): Self::SystemData) {
        let dt = (*clock_storage).delta;
        for (v, r) in (&vel_storage, &mut rot_storage).join() {
            r.0 += v.0 * dt;
        }
    }
}
