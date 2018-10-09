extern crate graphics;
extern crate opengl_graphics;
extern crate specs;

use self::graphics::{Graphics, Transformed, Viewport};
use self::graphics::math::{Matrix2d, Vec2d};
use self::opengl_graphics::{GlGraphics, OpenGL};
use self::specs::*;

use consts;
use game::components::*;
use std::collections::HashMap;

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
    gl: GlGraphics,
    parallax: Parallax,
}

impl Default for RenderSys {
    fn default() -> Self {
        RenderSys {
            gl: GlGraphics::new(OPENGL),
            parallax: Parallax::new(),
        }
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

            let mut parallax = &mut self.parallax;
            self.gl.draw(viewport, |c, gl| {
                clear(BLACK, gl);
                let transform = transform_with_center_and_zoom(c.transform, camera_center, camera_zoom);
                parallax.draw(gl, camera_center, camera_zoom, c.transform);
                for (pos, rot, shape) in (&pos_storage, &rot_storage, &shape_storage).join() {
                    draw_shape(gl, &shape, transform.trans(pos.0.x, pos.0.y).rot_rad(rot.0))
                }
            });
        }
    }
}

fn transform_with_center_and_zoom(transform: Matrix2d, center: Point, zoom: f64) -> Matrix2d {
    transform
        .trans(consts::WINDOW_SIZE[0] as f64 / 2.0, consts::WINDOW_SIZE[1] as f64 / 2.0)
        .zoom(zoom)
        .trans(-center.x, -center.y)
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

struct Parallax {
    seeds: HashMap<(usize, isize, isize), [u8; 16]>,
}

impl Parallax {
    const PLANES: usize = 4;
    const ZOOM_STEPS_PER_PLANE: f64 = 10.0;
    const SECTOR_SIZE: f64 = 100.0;
    const STARS_PER_SECTOR: usize = 6;
    const STAR_RADIUS: f64 = 0.05;

    fn new() -> Self {
        Parallax { seeds: HashMap::new() }
    }

    fn draw<G: Graphics>(&mut self, g: &mut G, center: Point, zoom: f64, base_transform: Matrix2d) {
        use self::graphics::*;

        for plane in 0..Parallax::PLANES {
            let zoom_change = consts::ZOOM_FACTOR.powf(plane as f64 * Parallax::ZOOM_STEPS_PER_PLANE);
            let radius = Parallax::STAR_RADIUS * zoom_change / zoom * consts::DEFAULT_ZOOM;
            let transform = transform_with_center_and_zoom(base_transform, center, zoom / zoom_change);
            self.draw_parallax_points(transform, plane, |p, c| {
                ellipse(c, rectangle::square(-radius, -radius, radius * 2.0), transform.trans(p[0], p[1]), g);
            });
        }
    }

    fn draw_parallax_points<F>(&mut self, transform: Matrix2d, plane: usize, mut draw_fun: F)
        where F: FnMut(Vec2d, Color) -> ()
    {
        use rand::prng::XorShiftRng;
        use rand::SeedableRng;
        use rand::Rng;

        // get the bounds for this plane
        let ([min_x, min_y], [max_x, max_y]) = world_bounds_for_transform(transform);

        // iterate through visible sectors
        let mut x = min_x - min_x % Parallax::SECTOR_SIZE - Parallax::SECTOR_SIZE;
        while x < max_x {
            let mut y = min_y - min_y % Parallax::SECTOR_SIZE - Parallax::SECTOR_SIZE;
            while y < max_y {
                // get pseudo random number generator for this sector
                let mut random = XorShiftRng::from_seed(self.rng_seed(plane, x, y));

                // generate stars
                for _ in 0..Parallax::STARS_PER_SECTOR {
                    // pick random colour component (use the same so it's grey), sqrt to push it into brighter colours, 0.9 to add randomness later
                    let c = random.gen::<f32>().sqrt() * 0.9;
                    draw_fun(
                        // random coords in the sector
                        [x + random.gen::<f64>() * Parallax::SECTOR_SIZE, y + random.gen::<f64>() * Parallax::SECTOR_SIZE],
                        // add a random colour tint (still mostly grey)
                        [c + random.gen::<f32>() * 0.1, c + random.gen::<f32>() * 0.1, c + random.gen::<f32>() * 0.1, 1.0],
                    );
                }
                y += Parallax::SECTOR_SIZE;
            }
            x += Parallax::SECTOR_SIZE;
        }
    }

    fn rng_seed(&mut self, p: usize, x: f64, y: f64) -> [u8; 16] {
        use rand;
        // get stored seed or generate a random one (using global RNG)
        *self.seeds.entry((p, x.floor() as isize, y.floor() as isize)).or_insert_with(rand::random::<[u8; 16]>)
    }
}

fn world_bounds_for_transform(transform: Matrix2d) -> (Vec2d, Vec2d) {
    // in the end transform matrix has to translate all points into (-1, 1) range
    // this function reverses the matrix assuming it doesn't have any rotation, aka it's of the form:
    // [ A 0 B ]
    // [ 0 C D ]
    // x is in the range (±1 - B) / A
    // y is in the range (±1 - D) / C
    let (min_x, max_x) = {
        let x1 = (1.0 - transform[0][2]) / transform[0][0];
        let x2 = (-1.0 - transform[0][2]) / transform[0][0];
        if x1 < x2 {
            (x1, x2)
        } else {
            (x2, x1)
        }
    };
    let (min_y, max_y) = {
        let y1 = (1.0 - transform[1][2]) / transform[1][1];
        let y2 = (-1.0 - transform[1][2]) / transform[1][1];
        if y1 < y2 {
            (y1, y2)
        } else {
            (y2, y1)
        }
    };
    ([min_x, min_y], [max_x, max_y])
}
