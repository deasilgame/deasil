extern crate specs;

use self::specs::*;

pub fn create_world() -> World {
    let mut world = World::new();
    world.add_resource(Clock::default());
    world.register::<Pos>();
    world
}

pub struct Clock {
    // last update delta
    delta: Option<f64>,

    // current world time
    time: f64,

    // 0 = paused, 1 = real-time
    simulation_speed: f64,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            delta: None,
            time: 0.0,
            simulation_speed: 1.0,
        }
    }
}

impl Clock {
    pub fn advance(&mut self, delta: Option<f64>) {
        self.delta = match delta {
            Some(dt) => {
                let d = dt * self.simulation_speed;
                self.time += d;
                Some(d)
            },
            None => None
        }
    }

    pub fn dt(&self) -> Option<f64> {
        self.delta
    }
}

#[derive(Debug)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn new(x: f64, y: f64) -> Self {
        Pos { x, y }
    }
}

impl Component for Pos {
    type Storage = VecStorage<Self>;
}
