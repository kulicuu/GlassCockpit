#![allow(unused)]
#![feature(drain_filter)]

mod routines;
use routines::particles_refresh::particles_refresh;
use routines::basic_2d_676::render_game;



use gloo_console::log;

fn main() {
    log!("Entry.");
    routines::particles_refresh::particles_refresh();
    // routines::basic_2d_676::render_game();
}