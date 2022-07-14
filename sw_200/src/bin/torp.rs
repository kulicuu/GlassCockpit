use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, MouseEvent, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};
use serde_json::{Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::sync::{Arc, Mutex};
use cgmath::prelude::*;
use cgmath::Rad;
use std::cell::RefCell;
use std::rc::Rc;
use std::convert::{TryInto};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use gloo_console::log;
use std::f32::consts::PI;


use crate::utils::time_polyfill::Instant;
use crate::structures::{
    TorpDrawStuff,
    GameState,
};

const AMORTIZATION: f32 = 0.95;
const LOCALIZED_SCALE : f32 = 0.001;
const CORRECTION : f32 = LOCALIZED_SCALE / 2.0;
const RESOLUTION : f32 = 8.0;
const SCALE : f32 = 0.08;
const HALF : f32 = SCALE / 2.0;
const STEP : f32 = SCALE / RESOLUTION;
const NUM_PARTICLES : u32 = 9680;



pub fn draw_torps
(
    gl: Arc<GL>,
    game_state: Arc<Mutex<GameState>>,
    torp_draw_stuff: Arc<TorpDrawStuff>,
)
{

    let shader_program = &torp_draw_stuff.shader_program;
    let vertex_buffer = &torp_draw_stuff.vertex_buffer;
    let js_vertices = &torp_draw_stuff.js_vertices;
    let vertices_position = &torp_draw_stuff.vertices_position;
    let vifo_theta_loc = &torp_draw_stuff.vifo_theta_loc;
    let pos_deltas_loc = &torp_draw_stuff.pos_deltas_loc;
    let time_loc = &torp_draw_stuff.time_loc;

    gl.use_program(Some(&shader_program));
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_vertices, GL::STATIC_DRAW);
    gl.vertex_attrib_pointer_with_i32(**vertices_position, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(**vertices_position);

    gl.use_program(Some(&shader_program));
    gl.uniform1f(Some(&time_loc), 0.4 as f32);

    for (idx, torp) in game_state.lock().unwrap()
    .torps_in_flight.lock().unwrap()
    .iter().enumerate() {
        let new_pos_dx = torp.lock().unwrap().position_dx;
        let new_pos_dy = torp.lock().unwrap().position_dy;
        let vifo_theta = torp.lock().unwrap().vifo_theta;
        gl.uniform2f(Some(&pos_deltas_loc), new_pos_dx, new_pos_dy);
        gl.uniform1f(Some(&vifo_theta_loc), vifo_theta.0);
        gl.draw_arrays(GL::TRIANGLES, 0, 6);
    }
}


pub fn setup_prepare_torp_draw
(gl: Arc<GL>) 
-> Result<Arc<TorpDrawStuff>, String>
{

    let torpedo_100_vertices: Vec<f32> = vec![
        0.007, 0.0,
        -0.0038, -0.0038, 
        -0.0038, 0.0038, 
    ];

    let vert_code = include_str!("../shaders/torpedo_100.vert");
    let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
    gl.shader_source(&vert_shader, vert_code);
    gl.compile_shader(&vert_shader);
    let vert_shader_log = gl.get_shader_info_log(&vert_shader);
    log!("shader log: ", vert_shader_log);
    
    let frag_code = include_str!("../shaders/basic.frag");
    let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
    gl.shader_source(&frag_shader, frag_code);
    gl.compile_shader(&frag_shader);
    let frag_shader_log = gl.get_shader_info_log(&frag_shader);
    log!("player frag shader log", frag_shader_log);

    let shader_program = Arc::new(gl.create_program().unwrap());
    gl.attach_shader(&shader_program, &vert_shader);
    gl.attach_shader(&shader_program, &frag_shader);
    gl.link_program(&shader_program);
    
    let time_loc = Arc::new(gl.get_uniform_location(&shader_program, "u_time").unwrap());

    let vertex_buffer = Arc::new(gl.create_buffer().unwrap());
    let js_vertices = Arc::new(js_sys::Float32Array::from(torpedo_100_vertices.as_slice()));
    let pos_deltas_loc = Arc::new(gl.get_uniform_location(&shader_program, "pos_deltas").unwrap());
    let vifo_theta_loc =  Arc::new(gl.get_uniform_location(&shader_program, "vifo_theta").unwrap());
    let vertices_position = Arc::new((gl.get_attrib_location(&shader_program, "b_position") as u32));
    
    Ok(
        Arc::new(
            TorpDrawStuff {
                shader_program: shader_program,
                vertex_buffer: vertex_buffer,
                js_vertices: js_vertices,
                vertices_position: vertices_position,
                pos_deltas_loc: pos_deltas_loc,
                vifo_theta_loc: vifo_theta_loc,
                time_loc: time_loc
            }
        )
    )


}