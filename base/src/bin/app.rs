#![allow(unused)]
#![feature(drain_filter)]

mod routines;
mod utils;

use routines::explosion::explosion;
use routines::particles_refresh::particles_refresh;
use routines::basic_2d_676::render_game;
use routines::transforms_lab::transforms_lab;


// use utils::time_polyfill



use gloo_console::log;

fn main() {
    // log!("Entry.");
    routines::explosion::explosion();
    // routines::transforms_lab::transforms_lab();
    // routines::particles_refresh::particles_refresh();
    // routines::basic_2d_676::render_game();
}