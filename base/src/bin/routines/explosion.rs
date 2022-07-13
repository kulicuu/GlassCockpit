#![allow(unused)]
#![feature(drain_filter)]

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
use std::time::*;

use std::convert::{TryInto};
use std::ops::{Add, Sub, AddAssign, SubAssign};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;

use gloo_console::log;
use std::f32::consts::PI;

const AMORTIZATION: f32 = 0.95;
const LOCALIZED_SCALE : f32 = 0.5;
const CORRECTION : f32 = LOCALIZED_SCALE / 2.0;
const RESOLUTION : f32 = 8.0;
const SCALE : f32 = 0.08;
const HALF : f32 = SCALE / 2.0;
const STEP : f32 = SCALE / RESOLUTION;
const NUM_PARTICLES : u32 = 9680;

// https://github.com/rust-lang/rust/issues/48564#issuecomment-698712971
// std::time invocation causes panic.  There is a comment linked above which solves this
// with the polyfillish stuff below.

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(std::time::Instant);

#[cfg(not(target_arch = "wasm32"))]
impl Instant {
    pub fn now() -> Self { Self(std::time::Instant::now()) }
    pub fn duration_since(&self, earlier: Instant) -> Duration { self.0.duration_since(earlier.0) }
    pub fn elapsed(&self) -> Duration { self.0.elapsed() }
    pub fn checked_add(&self, duration: Duration) -> Option<Self> { self.0.checked_add(duration).map(|i| Self(i)) }
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> { self.0.checked_sub(duration).map(|i| Self(i)) }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = r#"
export function performance_now() {
  return performance.now();
}"#)]
extern "C" {
    fn performance_now() -> f64;
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(u64);

#[cfg(target_arch = "wasm32")]
impl Instant {
    pub fn now() -> Self { Self((performance_now() * 1000.0) as u64) }
    pub fn duration_since(&self, earlier: Instant) -> Duration { Duration::from_micros(self.0 - earlier.0) }
    pub fn elapsed(&self) -> Duration { Self::now().duration_since(*self) }
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        match duration.as_micros().try_into() {
            Ok(duration) => self.0.checked_add(duration).map(|i| Self(i)),
            Err(_) => None,
        }
    }
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        match duration.as_micros().try_into() {
            Ok(duration) => self.0.checked_sub(duration).map(|i| Self(i)),
            Err(_) => None,
        }
    }
}

impl Add<Duration> for Instant { type Output = Instant; fn add(self, other: Duration) -> Instant { self.checked_add(other).unwrap() } }
impl Sub<Duration> for Instant { type Output = Instant; fn sub(self, other: Duration) -> Instant { self.checked_sub(other).unwrap() } }
impl Sub<Instant>  for Instant { type Output = Duration; fn sub(self, other: Instant) -> Duration { self.duration_since(other) } }
impl AddAssign<Duration> for Instant { fn add_assign(&mut self, other: Duration) { *self = *self + other; } }
impl SubAssign<Duration> for Instant { fn sub_assign(&mut self, other: Duration) { *self = *self - other; } }


pub fn explosion()
{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas33").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let gl: GL = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<GL>()
        .unwrap();
    let gl : Arc<GL> = Arc::new(gl);

    let vert_code = include_str!("../shaders/explosion.vert");
    let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
    gl.shader_source(&vert_shader, vert_code);
    gl.compile_shader(&vert_shader);
    let vert_shader_log = gl.get_shader_info_log(&vert_shader);
    log!("vert shader log: ", vert_shader_log);

    let frag_code = include_str!("../shaders/explosion.frag");
    let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
    gl.shader_source(&frag_shader, frag_code);
    gl.compile_shader(&frag_shader);
    let frag_shader_log = gl.get_shader_info_log(&frag_shader);
    log!("frag shader log:", frag_shader_log);

    let shader_program = gl.create_program().unwrap();
    gl.attach_shader(&shader_program, &vert_shader);
    gl.attach_shader(&shader_program, &frag_shader);
    gl.link_program(&shader_program);


    let position_data : Arc<Mutex<[f32; (NUM_PARTICLES * 3) as usize]>> = Arc::new(Mutex::new([0.0; (NUM_PARTICLES * 3) as usize]));
    let velocity_data : Arc<Mutex<[f32; (NUM_PARTICLES * 3) as usize]>> = Arc::new(Mutex::new([0.0; (NUM_PARTICLES *3) as usize]));
    let color_data : Arc<Mutex<[f32; (NUM_PARTICLES * 3) as usize]>> = Arc::new(Mutex::new([0.0; (NUM_PARTICLES * 3) as usize]));

    

    for i in 0..NUM_PARTICLES {
        let vec3i : usize = (i as usize) * 3;

        position_data.lock().unwrap()[vec3i] = (js_sys::Math::random() as f32) * LOCALIZED_SCALE - CORRECTION;
        position_data.lock().unwrap()[vec3i + 1] = (js_sys::Math::random() as f32) * LOCALIZED_SCALE - CORRECTION;
        position_data.lock().unwrap()[vec3i + 2] = (js_sys::Math::random() as f32) * LOCALIZED_SCALE - CORRECTION;



        // position_data.lock().unwrap()[vec3i] = (js_sys::Math::random() as f32);
        // position_data.lock().unwrap()[vec3i + 1] = (js_sys::Math::random() as f32);
        // position_data.lock().unwrap()[vec3i + 2] = (js_sys::Math::random() as f32);

        color_data.lock().unwrap()[vec3i] = js_sys::Math::random() as f32;
        color_data.lock().unwrap()[vec3i + 1] = js_sys::Math::random() as f32;
        color_data.lock().unwrap()[vec3i + 2] = js_sys::Math::random() as f32;
    }

    let position_buffer_a = Arc::new(gl.create_buffer().unwrap());
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer_a));
    let position_data_js = js_sys::Float32Array::from(position_data.lock().unwrap().as_slice());
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &position_data_js, GL::STREAM_COPY);
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    // let velocity_buffer_a = Arc::new(gl.create_buffer().unwrap());
    // gl.bind_buffer(GL::ARRAY_BUFFER, Some(&velocity_buffer_a));
    // let velocity_data_js = js_sys::Float32Array::from(velocity_data.lock().unwrap().as_slice());
    // gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &velocity_data_js, GL::STREAM_COPY);
    // gl.vertex_attrib_pointer_with_i32(1, 3, GL::FLOAT, false, 0, 0);
    // gl.enable_vertex_attrib_array(1);

    // let color_buffer = Arc::new(gl.create_buffer().unwrap());
    // gl.bind_buffer(GL::ARRAY_BUFFER, Some(&color_buffer));
    // let color_data_js = js_sys::Float32Array::from(color_data.lock().unwrap().as_slice());
    // gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &color_data_js, GL::STATIC_DRAW);
    // gl.enable_vertex_attrib_array(2);

    let transform_feedback_a = Arc::new(gl.create_transform_feedback().unwrap());

    gl.use_program(Some(&shader_program));





    gl.clear_color(0.98, 0.983, 0.992, 1.0);

    let render_loop_closure = Rc::new(RefCell::new(None));
    let alias_rlc = render_loop_closure.clone();
    *alias_rlc.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        gl.clear(GL::COLOR_BUFFER_BIT);


        // let position_buffer_a = Arc::new(gl.create_buffer().unwrap());
        // gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer_a));
        // let position_data_js = js_sys::Float32Array::from(position_data.lock().unwrap().as_slice());
        // gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &position_data_js, GL::STREAM_COPY);
        // gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        // gl.enable_vertex_attrib_array(0);
    
        // let velocity_buffer_a = Arc::new(gl.create_buffer().unwrap());
        // gl.bind_buffer(GL::ARRAY_BUFFER, Some(&velocity_buffer_a));
        // let velocity_data_js = js_sys::Float32Array::from(velocity_data.lock().unwrap().as_slice());
        // gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &velocity_data_js, GL::STREAM_COPY);
        // gl.vertex_attrib_pointer_with_i32(1, 3, GL::FLOAT, false, 0, 0);
        // gl.enable_vertex_attrib_array(1);


        // gl.begin_transform_feedback(GL::POINTS);
        gl.draw_arrays(GL::POINTS, 0, NUM_PARTICLES as i32);
        // gl.end_transform_feedback();


        request_animation_frame(render_loop_closure.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(alias_rlc.borrow().as_ref().unwrap());    


}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window().unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn setup_explosion_shaders(gl: Arc<GL>) {

}


struct ExplosionMaterials {

}