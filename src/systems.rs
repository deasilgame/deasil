extern crate specs;

use self::specs::*;

use components::*;

pub fn add_systems(dispatcher_builder: &mut DispatcherBuilder)  {
    dispatcher_builder.add(LinearMovementSys, "Linear Movement", &[]);
    dispatcher_builder.add(AngularMovementSys, "Angular Movement", &[]);
}

struct LinearMovementSys;

impl<'a> System<'a> for LinearMovementSys {
    type SystemData = (Read<'a, Clock>,
                       ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, (clock_storage, vel_storage, mut pos_storage): Self::SystemData) {
        if let Some(dt) = (*clock_storage).dt() {
            for (v, p) in (&vel_storage, &mut pos_storage).join() {
                p.x += v.x * dt;
                p.y += v.y * dt;
            }
        }
    }
}

struct AngularMovementSys;

impl<'a> System<'a> for AngularMovementSys {
    type SystemData = (Read<'a, Clock>,
                       ReadStorage<'a, AngularVelocity>,
                       WriteStorage<'a, Rotation>);

    fn run(&mut self, (clock_storage, vel_storage, mut rot_storage): Self::SystemData) {
        if let Some(dt) = (*clock_storage).dt() {
            for (v, r) in (&vel_storage, &mut rot_storage).join() {
                r.r += v.r * dt;
            }
        }
    }
}
