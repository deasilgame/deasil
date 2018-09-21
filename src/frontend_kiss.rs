extern crate kiss3d;

use consts;
use game;
use game::components::*;

use na::Point3;
use rand::random;
use specs::*;

use self::kiss3d::event::*;
use self::kiss3d::scene::{SceneNode, PlanarSceneNode};
use self::kiss3d::window::{State, Window};

struct GameState<'a, 'b> {
    game: game::Game<'a, 'b>,
}

impl<'a, 'b> GameState<'a, 'b> {
    fn render(&mut self, window: &mut Window) {
        // this code is TFB, it doesn't take into account projections
        // and draws lines too so something is visible
        // It's getting late - I'm giving up for now
        let red: Point3<f32> = Point3::new(1.0, 0.0, 0.0);
        let width = window.width() as f32;
        let height = window.height() as f32;
        for pos in self.game.world.read_storage::<Position>().join() {
            let p = Point3::new(
                pos.x as f32,
                pos.y as f32,
                0.0,
            );
            window.draw_point(&p, &red);
            window.draw_line(
                &p,
                &Point3::new(0.0, 0.0, 0.0),
                &red,
            );
        }
    }
}

impl State for GameState<'static, 'static> {
    fn step(&mut self, window: &mut Window) {
        for mut event in window.events().iter() {
            // handle events
            match event.value {
                WindowEvent::CursorPos(x, y, _) => {
                    self.game.handle_mouse_move([x, y]);
                }
                WindowEvent::MouseButton(MouseButton::Button1, Action::Release, _) => {
                    self.game.handle_mouse_left_click();
                }
                _ => {}
            }
        }

        self.game.update(0.13);
        self.render(window);
    }
}

pub fn main() {
    let mut window = Window::new(consts::TITLE);
    let mut game = game::Game::new();

    window.render_loop(GameState {
        game,
    })
}
