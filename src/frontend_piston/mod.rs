extern crate glutin_window;
extern crate graphics;
extern crate piston;

mod rendering;

use consts;
use game;

use self::glutin_window::GlutinWindow as Window;
use self::graphics::Viewport;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::piston::window::WindowSettings;

pub fn main() {
    use specs::DispatcherBuilder;

    let mut window: Window = WindowSettings::new(consts::TITLE, rendering::WINDOW_SIZE)
        .opengl(rendering::OPENGL)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new();
    game.add_resource(None as Option<graphics::Viewport>);

    let mut events = Events::new(EventSettings::new());
    let mut rendering_dispatcher = DispatcherBuilder::new()
        .with_thread_local(rendering::RenderSys::default())
        .build();

    while let Some(e) = events.next(&mut window) {
        // pass events to game
        if let Some(m) = e.mouse_cursor_args() {
            game.handle_mouse_move(m);
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            game.handle_mouse_left_click();
        }
        if let Some(u) = e.mouse_scroll_args() {
            game.handle_mouse_y_scroll(u[1]);
        }

        // update
        if let Some (u) = e.update_args() {
            game.update(u.dt);
        }

        // render
        if let Some(r) = e.render_args() {
            (*game.write_resource::<Option<Viewport>>()) = Some(r.viewport());
            game.render(&mut rendering_dispatcher);
        }
    }
}
