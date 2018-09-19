extern crate specs;

use self::specs::*;

pub fn create_world() -> World {
    let mut world = World::new();
    world.add_resource(None as Option<DeltaTime>);
    world.register::<Pos>();
    world
}

#[derive(Default)]
pub struct DeltaTime(f64);

impl From<f64> for DeltaTime {
    fn from(dt: f64) -> Self {
        DeltaTime(dt)
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
