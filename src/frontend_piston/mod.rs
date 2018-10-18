extern crate glutin_window;
extern crate graphics;
extern crate piston;

mod rendering;

use consts;
use game;

use self::glutin_window::GlutinWindow as Window;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::piston::window::WindowSettings;

fn handle_input_event(game: &mut game::Game, e: &Event) {
    let mut input = game.input_mut();
    if let Some(m) = e.mouse_cursor_args() {
        input.mouse_position = m;
    }
    if let Some(u) = e.mouse_scroll_args() {
        input.mouse_scroll = u;
    }
    if let Some(k) = e.press_args() {
        match k {
            Button::Mouse(MouseButton::Left) => input.mouse_left = true,
            Button::Keyboard(Key::W) => input.up = true,
            Button::Keyboard(Key::S) => input.down = true,
            Button::Keyboard(Key::A) => input.left = true,
            Button::Keyboard(Key::D) => input.right = true,
            _ => {}
        }
    }
    if let Some(k) = e.release_args() {
        match k {
            Button::Mouse(MouseButton::Left) => input.mouse_left = false,
            Button::Keyboard(Key::W) => input.up = false,
            Button::Keyboard(Key::S) => input.down = false,
            Button::Keyboard(Key::A) => input.left = false,
            Button::Keyboard(Key::D) => input.right = false,
            _ => {}
        }
    }
}

pub fn main() {
    let mut window: Window = WindowSettings::new(consts::TITLE, consts::WINDOW_SIZE)
        .opengl(rendering::OPENGL)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new();
    let mut events = Events::new(EventSettings::new());
    let mut rendering_sys = rendering::RenderSys::default();

    while let Some(e) = events.next(&mut window) {
        // pass events to game
        handle_input_event(&mut game, &e);

        // update
        if let Some(u) = e.update_args() {
            game.update(u.dt);
        }

        // render
        if let Some(r) = e.render_args() {
            rendering_sys.render(r.viewport(), &mut game);
        }
    }
}
