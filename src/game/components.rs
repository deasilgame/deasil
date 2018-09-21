use specs::*;

pub fn create_world() -> World {
    let mut world = World::new();
    world.add_resource(Clock::default());
    world.register::<Position>();
    world.register::<Rotation>();
    world.register::<Velocity>();
    world.register::<AngularVelocity>();
    world
}

pub struct Clock {
    // last update delta
    pub delta: f64,

    // current world time
    pub time: f64,

    // 0 = paused, 1 = real-time
    pub simulation_speed: f64,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            delta: 0.0,
            time: 0.0,
            simulation_speed: 1.0,
        }
    }
}

impl Clock {
    pub fn advance(&mut self, delta: f64) {
        self.delta = delta * self.simulation_speed;
        self.time += self.delta;
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    // coords in world system
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Rotation {
    // rotation in radians
    pub r: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    // ∆ world system coords / s
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct AngularVelocity {
    // ∆ rotation in radians / s
    pub r: f64,
}
