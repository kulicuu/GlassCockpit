#![allow(unused)]
#![feature(drain_filter)]

mod routines;
use routines::basic_2d_676::render_game;

use gloo_console::log;

fn main() {
    log!("Entry.");
    routines::basic_2d_676::render_game();
}