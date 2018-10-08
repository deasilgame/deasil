extern crate graphics;
extern crate opengl_graphics;
extern crate specs;

use self::graphics::{Graphics, Viewport};
use self::graphics::math::Matrix2d;
use self::opengl_graphics::{GlGraphics, OpenGL};
use self::specs::*;

use consts;
use game::components::*;

pub const OPENGL: OpenGL = OpenGL::V3_2;

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
                       Read<'a, Camera>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, Rotation>,
                       ReadStorage<'a, Shape>);
    
    fn run(&mut self, (viewport_storage, camera, pos_storage, rot_storage, shape_storage): Self::SystemData) {
        use self::graphics::*;
        use self::colors::*;

        if let Some(viewport) = *viewport_storage {
            let camera_center = camera.get_center_point();
            let camera_zoom = camera.get_zoom();

            self.gl.draw(viewport, |c, gl| {
                clear(BLACK, gl);
                let transform = c.transform
                    .trans(consts::WINDOW_SIZE[0] as f64 / 2.0, consts::WINDOW_SIZE[1] as f64 / 2.0)
                    .zoom(camera_zoom)
                    .trans(-camera_center.x, -camera_center.y);
                for (pos, rot, shape) in (&pos_storage, &rot_storage, &shape_storage).join() {
                    draw_shape(gl, &shape, transform.trans(pos.0.x, pos.0.y).rot_rad(rot.0))
                }
            });
        }
    }
}

fn draw_shape<G: Graphics>(g: &mut G, shape: &Shape, transform: Matrix2d) {
    use self::Shape::*;
    use self::graphics::*;
    use self::colors::*;

    match shape {
        Circle(radius) => ellipse(RED, rectangle::square(-radius, -radius, radius * 2.0), transform, g),
        Rectangle(Vector { dx: width, dy: height }) => rectangle(BLUE, [width / -2.0, height / -2.0, *width, *height], transform, g),
        Sprite(_name, Vector { dx: width, dy: height }) => {
            let rect = [width / -2.0, height / -2.0, *width, *height];
            rectangle(GREEN, rect, transform, g);
            // TODO: render `name` inside the rectangle;
        }
        Compound(ref subshapes) => for SubShape { offset: Vector {dx, dy}, rotation, shape } in subshapes.iter() {
            draw_shape(g, shape, transform.trans(*dx, *dy).rot_rad(*rotation))
        }
    }
}
