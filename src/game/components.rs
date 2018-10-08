use consts;
use specs::*;

pub fn create_world() -> World {
    let mut world = World::new();
    world.add_resource(Input::default());
    world.add_resource(Clock::default());
    world.add_resource(Camera::default());
    world.add_resource(None as Option<Player>);
    world.register::<Position>();
    world.register::<Rotation>();
    world.register::<Velocity>();
    world.register::<Acceleration>();
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

#[derive(Default, Debug)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,

    pub mouse_left: bool,
    pub mouse_scroll: [f64; 2],
    pub mouse_position: [f64; 2],
}

impl Input {
    pub fn keyboard_direction(&self) -> Vector {
        use std::f64::consts::SQRT_2;
        if (self.left || self.right) && (self.up || self.down) {
            Vector::new(if self.left { -SQRT_2 } else { SQRT_2 }, if self.up { -SQRT_2 } else { SQRT_2 })
        } else if self.left || self.right {
            Vector::new(if self.left { -1.0 } else { 1.0 }, 0.0)
        } else if self.up || self.down {
            Vector::new(0.0, if self.up { -1.0 } else { 1.0 })
        } else {
            Vector::new(0.0, 0.0)
        }
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

pub struct Camera {
    center: Point,
    zoom: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            center: Point::default(),
            zoom: consts::DEFAULT_ZOOM,
        }
    }
}

impl Camera {
    pub fn get_center_point(&self) -> Point {
        self.center
    }

    pub fn get_zoom(&self) -> f64 {
        self.zoom
    }

    pub fn center_at(&mut self, p: Point) {
        self.center = p
    }

    pub fn adjust_zoom(&mut self, m: f64) {
        self.zoom *= m
    }
}

pub struct Player(pub Entity);

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
// ∆ world system coords / s / s
pub struct Acceleration(pub Vector);

impl Acceleration {
    pub fn new(dx: f64, dy: f64) -> Self {
        Acceleration(Vector::new(dx, dy))
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
