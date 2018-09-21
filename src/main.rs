// all commonly used crates should be here
extern crate nalgebra as na;
extern crate rand;
extern crate specs;
#[macro_use]
extern crate specs_derive;

// shared modules
mod consts;
mod game;

// Kiss3D frontend
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "kiss3d"))]
mod frontend_kiss;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "kiss3d"))]
fn main() {
    frontend_kiss::main()
}

// Piston frontend
#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs", feature = "kiss3d")))]
mod frontend_piston;

#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs", feature = "kiss3d")))]
fn main() {
    frontend_piston::main()
}
