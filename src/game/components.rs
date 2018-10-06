use specs::*;

pub fn create_world() -> World {
    let mut world = World::new();
    world.add_resource(Clock::default());
    world.register::<Position>();
    world.register::<Rotation>();
    world.register::<Velocity>();
    world.register::<AngularVelocity>();
    world.register::<Shape>();
    world
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector {
    pub dx: f64,
    pub dy: f64,
}

impl Vector {
    pub fn new(dx: f64, dy: f64) -> Self {
        Vector { dx, dy }
    }
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

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
// coords in world system
pub struct Position(pub Point);

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position(Point::new(x, y))
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
// rotation in radians
pub struct Rotation(pub f64);

impl Rotation {
    pub fn new(r: f64) -> Self {
        Rotation(r)
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
// ∆ world system coords / s
pub struct Velocity(pub Vector);

impl Velocity {
    pub fn new(dx: f64, dy: f64) -> Self {
        Velocity(Vector::new(dx, dy))
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
// ∆ rotation in radians / s
pub struct AngularVelocity(pub f64);

impl AngularVelocity {
    pub fn new(dr: f64) -> Self {
        AngularVelocity(dr)
    }
}

#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub enum Shape {
    Circle(f64),
    Rectangle(Vector),
    Sprite(String, Vector),
    Compound(Vec<SubShape>),
}

#[derive(Clone, Debug)]
pub struct SubShape {
    pub offset: Vector,
    pub rotation: f64,
    pub shape: Shape,
}
