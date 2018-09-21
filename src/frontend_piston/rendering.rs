extern crate graphics;
extern crate opengl_graphics;
extern crate specs;

use self::graphics::Viewport;
use self::opengl_graphics::{GlGraphics, OpenGL};
use self::specs::*;

use game::components::*;

pub const OPENGL: OpenGL = OpenGL::V3_2;
pub const WINDOW_SIZE: [u32; 2] = [800, 800];

pub type Color = [f32; 4];

#[allow(dead_code)]
pub mod colors {
    use super::Color;

    pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
    pub const BLUE:  Color = [0.0, 0.0, 1.0, 1.0];
    pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
    pub const RED:   Color = [1.0, 0.0, 0.0, 1.0];
    pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
}

pub struct RenderSys {
    gl: GlGraphics
}

impl Default for RenderSys {
    fn default() -> Self {
        RenderSys { gl: GlGraphics::new(OPENGL) }
    }
}

impl<'a> System<'a> for RenderSys {
    type SystemData = (Read<'a, Option<Viewport>>,
                       ReadStorage<'a, Position>);
    
    fn run(&mut self, (viewport_storage, pos_storage): Self::SystemData) {
        use self::graphics::*;
        use self::colors::*;

        if let Some(viewport) = *viewport_storage {
            self.gl.draw(viewport, |c, gl| {
                clear(BLACK, gl);

                let transform = c.transform;
                for pos in (&pos_storage).join() {
                    ellipse(RED, rectangle::square(pos.x, pos.y, 3.0), transform, gl);
                }
            });
        }
    }
}
