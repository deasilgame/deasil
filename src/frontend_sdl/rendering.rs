extern crate specs;

use super::sdl2::gfx::primitives::DrawRenderer;
use super::sdl2::pixels::Color;
use super::sdl2::video::Window;
use super::sdl2::render::Canvas;
use self::specs::*;

use game::components::*;

pub const WINDOW_SIZE: [u32; 2] = [800, 800];

pub struct RenderSys {
    canvas: Canvas<Window>
}

impl RenderSys {
    pub fn new(canvas: Canvas<Window>) -> Self {
        RenderSys { canvas }
    }
}

impl<'a> System<'a> for RenderSys {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, pos_storage: Self::SystemData) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        for pos in (&pos_storage).join() {
            self.canvas.filled_circle(
                pos.x as i16,
                pos.y as i16,
                2,
                Color::RGB(255, 0, 0),
            ).unwrap();
        }
        self.canvas.present();
    }
}
