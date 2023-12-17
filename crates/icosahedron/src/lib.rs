mod icosahedron;
use wasm_bindgen::prelude::*;

use cgmath::SquareMatrix;
const TRIANGLE_FRAGS: bool = true;
const PHI: f32 = 1.618_034;
const GOLDEN_RECTANGLE_VERTS: &[f32] = &[
    -1.0, PHI, 0.0, 1.0, PHI, 0.0, 1.0, -PHI, 0.0, -1.0, PHI, 0.0, -1.0, -PHI, 0.0, 1.0, -PHI, 0.0,
];

#[repr(u32)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
enum FragEnum {
    Red = 0,
    Green = 1,
    Blue = 2,
    Black = 3,
    ClearRed = 4,
    ClearGreen = 5,
    ClearBlue = 6,
}

struct MouseState {
    mouse_down: bool,
    mouse_drag_pos_prev: Option<(i32, i32)>,
    mouse_drag_pos: Option<(i32, i32)>,
}

#[derive(Clone)]
struct Globals {
    last_tick_time: std::sync::Arc<std::sync::Mutex<u64>>,
    context: web_sys::WebGl2RenderingContext,
    model_matrix_location: web_sys::WebGlUniformLocation,
    frag_enum_location: web_sys::WebGlUniformLocation,
    mouse: std::sync::Arc<std::sync::Mutex<MouseState>>,
    camera_matrix: std::sync::Arc<std::sync::Mutex<cgmath::Matrix4<f32>>>,
}

fn make_globals(
    context: web_sys::WebGl2RenderingContext,
    program: web_sys::WebGlProgram,
) -> Globals {
    let camera_matrix = std::sync::Arc::new(std::sync::Mutex::new(cgmath::Matrix4::identity()));
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
    let frag_enum_location = context.get_uniform_location(&program, "fragEnum").unwrap();
    let mouse = std::sync::Arc::new(std::sync::Mutex::new(MouseState {
        mouse_down: false,
        mouse_drag_pos_prev: None,
        mouse_drag_pos: None,
    }));
    Globals {
        context,
        last_tick_time: std::sync::Arc::new(std::sync::Mutex::new(0)),
        model_matrix_location,
        frag_enum_location,
        mouse,
        camera_matrix,
    }
}
#[wasm_bindgen]
pub async fn andy_main() {
    let mut verts: Vec<f32> = GOLDEN_RECTANGLE_VERTS.to_vec();
    let ico_verts = icosahedron::sphere_recurse_verts(icosahedron::sphere_recurse_verts(
        icosahedron::generate_verticies(),
    ));
    verts.extend(
        ico_verts
            .iter()
            .flat_map(|t| t.iter().flat_map(|v| [v.x, v.y, v.z]).collect::<Vec<f32>>())
            .collect::<Vec<f32>>(),
    );

    let (context, program) = andys_webgl_main::setup_canvas(
        "big_canvas",
        andys_webgl_main::ShaderProg {
            vert_shader_source: include_str!("./shaders_source/vertex.glsl"),
            frag_shader_source: include_str!("./shaders_source/frag.glsl"),
            verts: &verts,
            vertex_attrib_name: "position",
            draw_func: draw,
        },
        Some(andy_mousedown_callback),
        Some(andy_mouseup_callback),
        Some(andy_mousemove_callback),
        make_globals,
    );

    let vao = context.create_vertex_array().unwrap();
    context.bind_vertex_array(Some(&vao));

    let position_attrib = context.get_attrib_location(&program, "position");
    context.vertex_attrib_pointer_with_i32(
        position_attrib as u32,
        3,
        web_sys::WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    context.enable_vertex_attrib_array(position_attrib as u32);
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

/*
fn poo(){
    canvas
        .add_event_listener_with_callback(
            "mousedown",
            andy_mousedown_callback.as_ref().dyn_ref().unwrap(),
        )
        .unwrap();
    std::mem::forget(andy_mousedown_callback);

    let globals1 = globals.clone();
    let andy_mouseup_callback = Closure::wrap(Box::new(move |_e: web_sys::Event| {
        globals1.mouse.lock().unwrap().mouse_down = false;
    }) as Box<dyn FnMut(_)>);
    canvas
        .add_event_listener_with_callback(
            "mouseup",
            andy_mouseup_callback.as_ref().dyn_ref().unwrap(),
        )
        .unwrap();
    std::mem::forget(andy_mouseup_callback);

    let globals2 = globals.clone();
    let andy_mousemove_callback = |e: web_sys::Event| {
        let mouse_event: web_sys::MouseEvent = e.dyn_into().unwrap();
        let mut mouse_state = mouse.lock().unwrap();
        if mouse_state.mouse_down {
            mouse_state.mouse_drag_pos_prev = mouse_state.mouse_drag_pos;
            mouse_state.mouse_drag_pos = Some((mouse_event.x(), mouse_event.y()));
        }
    };
    canvas
        .add_event_listener_with_callback(
            "mousemove",
            andy_mousemove_callback.as_ref().dyn_ref().unwrap(),
        )
        .unwrap();
    std::mem::forget(andy_mousemove_callback);
}
*/
fn draw(globals: Globals) {
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
        let mut camera_matrix = globals.camera_matrix.lock().unwrap();
        let move_direction_vector = cgmath::Vector4 {
            x: vx as f32,
            y: vy as f32,
            z: 0.0,
            w: 1.0,
        };
        let forward_vector = cgmath::Vector4 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 1.0,
        };
        let cross = forward_vector
            .truncate()
            .cross(move_direction_vector.truncate());
        if length(cross) != 0.0 {
            *camera_matrix = cgmath::Matrix4::from_axis_angle(
                normalize(cross),
                cgmath::Rad(length(cross) / 200.0),
            ) * (*camera_matrix);
        }
    }

    let forward_rotated = cgmath::Matrix4::from_translation(cgmath::Vector3 {
        x: 0.0,
        y: 0.0,
        z: -3.0,
    }) * *globals.camera_matrix.lock().unwrap();

    draw_rectangle(&globals, forward_rotated, FragEnum::Red);
    let rotated_1 = cgmath::Matrix4::from_axis_angle(
        normalize(cgmath::Vector3 {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        }),
        cgmath::Rad(std::f32::consts::TAU / 2.0),
    ) * cgmath::Matrix4::from_axis_angle(
        normalize(cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }),
        cgmath::Rad(std::f32::consts::TAU / 4.0),
    );
    draw_rectangle(&globals, forward_rotated * rotated_1, FragEnum::Green);

    draw_rectangle(&globals, forward_rotated, FragEnum::Red);
    let rotated_2 = cgmath::Matrix4::from_axis_angle(
        normalize(cgmath::Vector3 {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        }),
        cgmath::Rad(std::f32::consts::TAU / 2.0),
    ) * cgmath::Matrix4::from_axis_angle(
        normalize(cgmath::Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        }),
        cgmath::Rad(std::f32::consts::TAU / 4.0),
    );
    draw_rectangle(&globals, forward_rotated * rotated_2, FragEnum::Blue);

    let bigger = cgmath::Matrix4::from_scale(((PHI * PHI) + (1.0 * 1.0)).sqrt());
    draw_triangle(&globals, forward_rotated * bigger, FragEnum::ClearRed);
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
fn draw_rectangle(globals: &Globals, model_mat: cgmath::Matrix4<f32>, frag_enum: FragEnum) {
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
fn draw_triangle(globals: &Globals, model_mat: cgmath::Matrix4<f32>, frag_enum: FragEnum) {
    globals.context.uniform_matrix4fv_with_f32_array(
        Some(&globals.model_matrix_location),
        false,
        &matrix_to_vec(model_mat),
    );
    if TRIANGLE_FRAGS {
        globals
            .context
            .uniform1ui(Some(&globals.frag_enum_location), frag_enum as u32);
        globals
            .context
            .draw_arrays(web_sys::WebGl2RenderingContext::TRIANGLES, 6, 960);
    }
    for i in 0..320 {
        globals
            .context
            .uniform1ui(Some(&globals.frag_enum_location), FragEnum::Black as u32);
        globals
            .context
            .draw_arrays(web_sys::WebGl2RenderingContext::LINE_LOOP, 6 + (3 * i), 3);
    }
}
