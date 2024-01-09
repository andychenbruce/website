use wasm_bindgen::prelude::*;

use cgmath::SquareMatrix;
const SQUARE_VERTS: &[f32] = &[
    0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0,
];
#[repr(u32)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
enum FragEnum {
    Red = 0,
    Green = 1,
    Blue = 2,
    Black = 3,
    White = 4,
    ClearRed = 5,
    ClearGreen = 6,
    ClearBlue = 7,
    RedReflective = 8,
    RedSpectral = 9,
    BeachBall = 10,
}

struct MouseState {
    mouse_down: bool,
    mouse_drag_pos_prev: Option<(i32, i32)>,
    mouse_drag_pos: Option<(i32, i32)>,
}

#[derive(Copy, Clone)]
enum FragLevel {
    Water,
    Black,
}

struct DrawConfig {
    wires: bool,
    frags: FragLevel,
}

#[derive(Clone)]
struct Globals {
    last_tick_time: std::sync::Arc<std::sync::Mutex<u64>>,
    context: web_sys::WebGl2RenderingContext,
    camera_matrix_location: web_sys::WebGlUniformLocation,
    model_matrix_location: web_sys::WebGlUniformLocation,
    frag_enum_location: web_sys::WebGlUniformLocation,
    mouse: std::sync::Arc<std::sync::Mutex<MouseState>>,
    camera_offset_matrix: cgmath::Matrix4<f32>,
}

fn make_globals(
    context: web_sys::WebGl2RenderingContext,
    program: web_sys::WebGlProgram,
) -> Globals {
    let perspective_matrix: cgmath::Matrix4<f32> = cgmath::Matrix4::from(cgmath::PerspectiveFov {
        fovy: cgmath::Rad(2.0),
        aspect: 1.0,
        near: 0.001,
        far: 10.0,
    });
    let perspective_matrix_location = context
        .get_uniform_location(&program, "perspectiveMatrix")
        .unwrap();
    context.uniform_matrix4fv_with_f32_array(
        Some(&perspective_matrix_location),
        false,
        &matrix_to_vec(perspective_matrix),
    );
    let model_matrix_location = context
        .get_uniform_location(&program, "modelMatrix")
        .unwrap();
    let camera_matrix_location = context
        .get_uniform_location(&program, "cameraMatrix")
        .unwrap();
    let frag_enum_location = context.get_uniform_location(&program, "fragEnum").unwrap();
    let mouse = std::sync::Arc::new(std::sync::Mutex::new(MouseState {
        mouse_down: false,
        mouse_drag_pos_prev: Some((0, 0)),
        mouse_drag_pos: Some((2, 1)),
    }));
    Globals {
        context,
        last_tick_time: std::sync::Arc::new(std::sync::Mutex::new(0)),
        model_matrix_location,
        camera_matrix_location,
        frag_enum_location,
        mouse,
        camera_offset_matrix: cgmath::Matrix4::from_translation(cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: -3.0,
        }),
    }
}
#[wasm_bindgen]
pub fn andy_main() {
    set_canvas("big_canvas", true, FragLevel::Water);
}

fn set_canvas(name: &str, wires: bool, frags: FragLevel) {
    let mut verts: Vec<f32> = SQUARE_VERTS.to_vec();

    let (_context, _program) = andy_webgl_utils::setup_canvas(
        name,
        andy_webgl_utils::ShaderProg {
            vert_shader_source: include_str!("./shaders_source/vertex.glsl"),
            frag_shader_source: include_str!("./shaders_source/frag.glsl"),
            verts: &verts,
            vertex_attrib_name: "position",
            draw_func: move |globals: Globals| draw(globals, DrawConfig { wires, frags }),
        },
        Some(andy_mousedown_callback),
        Some(andy_mouseup_callback),
        Some(andy_mousemove_callback),
        make_globals,
    );
}

fn andy_mousedown_callback(globals: Globals, _e: web_sys::Event) {
    let mut mouse = globals.mouse.lock().unwrap();
    mouse.mouse_drag_pos_prev = None;
    mouse.mouse_drag_pos = None;
    mouse.mouse_down = true;
}
fn andy_mouseup_callback(globals: Globals, _e: web_sys::Event) {
    globals.mouse.lock().unwrap().mouse_down = false;
}
fn andy_mousemove_callback(globals: Globals, e: web_sys::Event) {
    let mouse_event: web_sys::MouseEvent = e.dyn_into().unwrap();
    let mut mouse_state = globals.mouse.lock().unwrap();
    if mouse_state.mouse_down {
        mouse_state.mouse_drag_pos_prev = mouse_state.mouse_drag_pos;
        mouse_state.mouse_drag_pos = Some((mouse_event.x(), mouse_event.y()));
    }
}

fn draw(globals: Globals, config: DrawConfig) {
    globals
        .context
        .clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT);

    let current_time_milis = js_sys::Date::new_0().get_time() as u64;
    let mut old_time = globals.last_tick_time.lock().unwrap();
    *old_time = current_time_milis;
    //let diff = current_time_milis - *old_time;

    if let Some((vx, vy)) = {
        let mouse = globals.mouse.lock().unwrap();
        if let (Some((oldx, oldy)), Some((newx, newy))) =
            (mouse.mouse_drag_pos_prev, mouse.mouse_drag_pos)
        {
            Some((newx - oldx, -(newy - oldy))) //negate the y since up is negative on the canvas
        } else {
            None
        }
    } {
        todo!();
    }

    let model_matrix = cgmath::Matrix4::identity();

    draw_square(&globals, model_matrix, FragEnum::Green);
}

fn matrix_to_vec(mat: cgmath::Matrix4<f32>) -> [f32; 16] {
    [
        mat.x.x, mat.x.y, mat.x.z, mat.x.w, //x col
        mat.y.x, mat.y.y, mat.y.z, mat.y.w, //y col
        mat.z.x, mat.z.y, mat.z.z, mat.z.w, //z col
        mat.w.x, mat.w.y, mat.w.z, mat.w.w, //w col
    ]
}

fn normalize(vec: cgmath::Vector3<f32>) -> cgmath::Vector3<f32> {
    vec / length(vec)
}

fn length(vec: cgmath::Vector3<f32>) -> f32 {
    let len_squared: f32 = (vec.x * vec.x) + (vec.y * vec.y) + (vec.z * vec.z);

    len_squared.sqrt()
}
fn draw_square(globals: &Globals, model_mat: cgmath::Matrix4<f32>, frag_enum: FragEnum) {
    globals.context.uniform_matrix4fv_with_f32_array(
        Some(&globals.model_matrix_location),
        false,
        &matrix_to_vec(model_mat),
    );
    globals
        .context
        .uniform1ui(Some(&globals.frag_enum_location), frag_enum as u32);
    globals
        .context
        .draw_arrays(web_sys::WebGl2RenderingContext::TRIANGLES, 0, 6);
}
