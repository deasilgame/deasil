// all commonly used crates should be here
extern crate nalgebra as na;
extern crate rand;

// shared modules
mod consts;
mod game;

// Piston frontend
mod frontend_piston;
fn main() {
    frontend_piston::main()
}
