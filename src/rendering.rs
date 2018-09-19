extern crate graphics;
extern crate opengl_graphics;
extern crate specs;

use self::graphics::Viewport;
use self::opengl_graphics::{GlGraphics, OpenGL};
use self::specs::*;

use components::*;

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
                       ReadStorage<'a, Pos>);
    
    fn run(&mut self, (viewport_storage, pos_storage): Self::SystemData) {
        use graphics::*;
        use self::colors::*;

        if let Some(viewport) = *viewport_storage {
            self.gl.draw(viewport, |c, gl| {
                clear(BLACK, gl);

                let transform = c.transform;
                let mid_x = WINDOW_SIZE[0] as f64 / 2.0;
                let mid_y = WINDOW_SIZE[1] as f64 / 2.0;

                for pos in (&pos_storage).join() {
                    ellipse(RED, rectangle::square(pos.x, pos.y, 3.0), transform, gl);
                }
                for i in 1..10 {
                    let f = i as f64 * 10.0;
                    ellipse(WHITE, rectangle::square(mid_x + f, mid_y + f, 3.0), transform, gl);
                    ellipse(WHITE, rectangle::square(mid_x - f, mid_y + f, 3.0), transform, gl);
                    ellipse(WHITE, rectangle::square(mid_x + f, mid_y - f, 3.0), transform, gl);
                    ellipse(WHITE, rectangle::square(mid_x - f, mid_y - f, 3.0), transform, gl);
                }
            });
        }
    }
}
