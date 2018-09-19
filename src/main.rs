extern crate glutin_window;
extern crate graphics;
extern crate piston;
extern crate specs;

mod components;
mod rendering;

use glutin_window::GlutinWindow as Window;
use graphics::Viewport;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use specs::*;

use components::*;


fn main() {
    let mut window: Window = WindowSettings::new("deasil", rendering::WINDOW_SIZE)
        .opengl(rendering::OPENGL)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // create ECS world
    let mut world = create_world();
    world.add_resource(None as Option<Viewport>);

    let mut dispatcher = DispatcherBuilder::new()
    // TODO: add systems
        .with_thread_local(rendering::RenderSys::default())
        .build();

    let mut events = Events::new(EventSettings::new());
    let mut mouse = None;
    while let Some(e) = events.next(&mut window) {

        // follow mouse
        if let Some(m) = e.mouse_cursor_args() {
            mouse = Some(m);
        }

        // add Pos on left button release
        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            if let Some(pos) = mouse {
                world.create_entity()
                    .with(Pos::new(pos[0], pos[1]))
                    .build();
            }
        }

        // advance the Clock
        (*world.write_resource::<Clock>()).advance(match e.update_args() {
            Some(u) => Some(u.dt),
            None => None,
        });

        // pass Viewport for rendering
        (*world.write_resource::<Option<Viewport>>()) = match e.render_args() {
            Some(r) => Some(r.viewport()),
            None => None
        };

        // run the update and rendering
        dispatcher.dispatch(&world.res);

        // apply async changes to the world
        world.maintain();
    }
}
