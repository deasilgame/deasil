extern crate sdl2;

mod rendering;

use consts;
use game;

use std::time::{Duration, Instant};

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;

fn seconds_from_duration(d: Duration) -> f64 {
    d.as_secs() as f64 + (d.subsec_micros() as f64 / 1_000_000.0)
}

pub fn main() {
    use specs::DispatcherBuilder;

    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window(consts::TITLE, rendering::WINDOW_SIZE[0], rendering::WINDOW_SIZE[1])
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut game = game::Game::new();

    let mut rendering_dispatcher = DispatcherBuilder::new()
        .with_thread_local(rendering::RenderSys::new(window.into_canvas().build().unwrap()))
        .build();

    let mut events = sdl_context.event_pump().unwrap();
    let mut last_time = Instant::now();

    'running: loop {
        let current = Instant::now();

        // pass events to the game
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape {
                        break 'running
                    }
                }

                Event::MouseButtonDown {x, y, ..} => {
                    game.handle_mouse_move([x as f64, y as f64]);
                    game.handle_mouse_left_click();
                }

                _ => {}
            }
        }

        // update the game
        game.update(seconds_from_duration(current - last_time));
        last_time = current;

        // draw
        game.render(&mut rendering_dispatcher);
    }
}
