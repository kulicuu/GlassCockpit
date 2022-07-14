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
use crate::structures::{PlayerDrawStuff};

const AMORTIZATION: f32 = 0.95;
const LOCALIZED_SCALE : f32 = 0.001;
const CORRECTION : f32 = LOCALIZED_SCALE / 2.0;
const RESOLUTION : f32 = 8.0;
const SCALE : f32 = 0.08;
const HALF : f32 = SCALE / 2.0;
const STEP : f32 = SCALE / RESOLUTION;
const NUM_PARTICLES : u32 = 9680;


pub fn draw_player() {}

pub fn setup_shader() {}


// setup shaders and some objects:
pub fn setup_prepare_player
(
    gl: Arc<GL>,
)
-> Result<PlayerDrawStuff, String>
{

    let vehicle_100_vertices: Vec<f32> = vec![
        0.021, 0.0, 
         -0.008, -0.008,
        -0.008, 0.008,
    ];

    let vert_code = include_str!("../shaders/vehicle_100.vert");
    let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
    gl.shader_source(&vert_shader, vert_code);
    gl.compile_shader(&vert_shader);
    let vert_shader_log = gl.get_shader_info_log(&vert_shader);
    log!("vehicle_100 shader log: ", vert_shader_log);
    
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
    let js_vertices = Arc::new(js_sys::Float32Array::from(vehicle_100_vertices.as_slice()));
    let pos_deltas_loc = Arc::new(gl.get_uniform_location(&shader_program, "pos_deltas").unwrap());
    let vifo_theta_loc =  Arc::new(gl.get_uniform_location(&shader_program, "vifo_theta").unwrap());
    let vertices_position = Arc::new((gl.get_attrib_location(&shader_program, "a_position") as u32));
    
    Ok(
        PlayerDrawStuff {
            shader_program: shader_program,
            vertex_buffer: vertex_buffer,
            js_vertices: js_vertices,
            vertices_position: vertices_position,
            pos_deltas_loc: pos_deltas_loc,
            vifo_theta_loc: vifo_theta_loc,
            time_loc: time_loc
        }
    )
}