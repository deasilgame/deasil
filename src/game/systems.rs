use game::components::*;

use specs::*;

pub struct LinearMovementSys;
impl<'a> System<'a> for LinearMovementSys {
    type SystemData = (Read<'a, Clock>,
                       ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, (clock_storage, vel_storage, mut pos_storage): Self::SystemData) {
        let dt = (*clock_storage).delta;
        for (v, p) in (&vel_storage, &mut pos_storage).join() {
            p.x += v.x * dt;
            p.y += v.y * dt;
        }
    }
}

pub struct AngularMovementSys;
impl<'a> System<'a> for AngularMovementSys {
    type SystemData = (Read<'a, Clock>,
                       ReadStorage<'a, AngularVelocity>,
                       WriteStorage<'a, Rotation>);

    fn run(&mut self, (clock_storage, vel_storage, mut rot_storage): Self::SystemData) {
        let dt = (*clock_storage).delta;
        for (v, r) in (&vel_storage, &mut rot_storage).join() {
            r.r += v.r * dt;
        }
    }
}