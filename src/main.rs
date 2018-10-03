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
#[cfg(feature = "piston")]
mod frontend_piston;

#[cfg(feature = "piston")]
fn main() {
    frontend_piston::main()
}

// SDL2 frontend
#[cfg(not(feature = "piston"))]
mod frontend_sdl;

#[cfg(not(feature = "piston"))]
fn main() {
    frontend_sdl::main()
}
