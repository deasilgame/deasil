pub mod components;
mod systems;

use rand::random;
use specs::shred::{FetchMut, Resource};
use specs::*;

pub struct Game<'a, 'b> {
    pub world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new() -> Self {
        Game {
            world: components::create_world(),
            dispatcher: DispatcherBuilder::new()
                .with(systems::InputSys, "Input", &[])
                .with(systems::AccelerationSys, "Acceleration", &[])
                .with(systems::LinearMovementSys, "Linear Movement", &[])
                .with(systems::AngularMovementSys, "Angular Movement", &[])
                .build(),
        }
    }

    pub fn add_resource<T: Resource>(&mut self, resource: T) {
        self.world.add_resource(resource);
    }

    pub fn write_resource<T: Resource>(&self) -> FetchMut<T> {
        self.world.write_resource()
    }

    pub fn update(&mut self, dt: f64) {
        // andvance game clock
        (*self.world.write_resource::<components::Clock>()).advance(dt);

        // update the world
        self.dispatcher.dispatch(&self.world.res);

        // process events generated by systems
        // TODO

        // process async entity creation/deletion
        self.world.maintain();
    }

    pub fn render(&mut self, rendering_dispatcher: &mut Dispatcher) {
        rendering_dispatcher.dispatch(&self.world.res);
    }

    pub fn create_player(&mut self) {
        let player_entity = self
            .world
            .create_entity()
            .with(components::Position(components::Point::new(0.0, 0.0)))
            .with(components::Rotation::default())
            .with(components::Velocity::default())
            .with(components::Acceleration::default())
            .with(components::AngularVelocity::default())
            .with(player_shape())
            .build();
        (*self.world.write_resource::<Option<components::Player>>()) =
            Some(components::Player(player_entity));
    }

    pub fn input_mut(&mut self) -> FetchMut<components::Input> {
        self.world.write_resource::<components::Input>()
    }
}

pub fn create_dummy_entity<B: Builder>(builder: B) -> B {
    // create a dummy "particle"
    const MAX_V: f64 = 20.0;
    builder
        .with(components::Position(components::Point::new(0.0, 0.0)))
        .with(components::Rotation::default())
        .with(components::Velocity::new(
            random_range(-MAX_V, MAX_V),
            random_range(-MAX_V, MAX_V),
        )).with(components::AngularVelocity::new(random_range(-3.14, 3.14)))
        .with(random_shape())
}

fn random_range(from: f64, to: f64) -> f64 {
    from + (to - from) * random::<f64>()
}

const SHAPE_SIZE: f64 = 1.0;

fn player_shape() -> components::Shape {
    use self::components::Shape::*;
    use self::components::{SubShape, Vector};

    let mut subshapes = Vec::new();
    subshapes.push(SubShape {
        offset: Vector::new(0.0, 0.0),
        rotation: 0.0,
        shape: Circle(1.0),
    });
    subshapes.push(SubShape {
        offset: Vector::new(0.75, 0.0),
        rotation: 0.0,
        shape: Rectangle(Vector::new(0.75, 0.5)),
    });
    subshapes.push(SubShape {
        offset: Vector::new(-0.75, 0.75),
        rotation: 3.14 / 4.0,
        shape: Rectangle(Vector::new(0.5, 0.5)),
    });
    subshapes.push(SubShape {
        offset: Vector::new(-0.75, -0.75),
        rotation: 3.14 / 4.0,
        shape: Rectangle(Vector::new(0.5, 0.5)),
    });

    Compound(subshapes)
}

fn random_shape() -> components::Shape {
    use self::components::Shape::*;
    use self::components::{SubShape, Vector};

    match (random::<bool>(), random::<bool>()) {
        (true, true) => Circle(SHAPE_SIZE / 2.0),
        (true, false) => Rectangle(Vector {
            dx: SHAPE_SIZE,
            dy: SHAPE_SIZE,
        }),
        (false, true) => Sprite(
            "TODO".to_string(),
            Vector {
                dx: SHAPE_SIZE,
                dy: SHAPE_SIZE,
            },
        ),
        (false, false) => {
            let mut subshapes = Vec::new();
            for _ in 0..3 {
                subshapes.push(SubShape {
                    offset: Vector {
                        dx: random_range(-SHAPE_SIZE * 0.75, SHAPE_SIZE * 0.75),
                        dy: random_range(-SHAPE_SIZE * 0.75, SHAPE_SIZE * 0.75),
                    },
                    rotation: random_range(0.0, 3.14),
                    shape: random_shape(),
                })
            }
            Compound(subshapes)
        }
    }
}
