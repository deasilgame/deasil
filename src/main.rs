// all commonly used crates should be here
extern crate nalgebra as na;
extern crate rand;
extern crate specs;
#[macro_use]
extern crate specs_derive;

// shared modules
mod consts;
mod game;

// Piston frontend
mod frontend_piston;
fn main() {
    frontend_piston::main()
}

// SDL2 frontend
// mod frontend_sdl;
// fn main() {
//     frontend_sdl::main()
// }
