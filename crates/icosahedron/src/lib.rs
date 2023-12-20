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
    None,
    Transparent,
    Full,
    Reflective,
    Spectral,
    BeachBall,
}

struct DrawConfig {
    rectangles: bool,
    wires: bool,
    light: bool,
    frags: FragLevel,
    num_triangles: i32,
}

#[derive(Clone)]
struct Globals {
    last_tick_time: std::sync::Arc<std::sync::Mutex<u64>>,
    context: web_sys::WebGl2RenderingContext,
    camera_matrix_location: web_sys::WebGlUniformLocation,
    model_matrix_location: web_sys::WebGlUniformLocation,
    light_pos_location: web_sys::WebGlUniformLocation,
    frag_enum_location: web_sys::WebGlUniformLocation,
    mouse: std::sync::Arc<std::sync::Mutex<MouseState>>,
    camera_rotate_matrix: std::sync::Arc<std::sync::Mutex<cgmath::Matrix4<f32>>>,
    camera_offset_matrix: cgmath::Matrix4<f32>,
}

fn make_globals(
    context: web_sys::WebGl2RenderingContext,
    program: web_sys::WebGlProgram,
) -> Globals {
    let camera_rotate_matrix =
        std::sync::Arc::new(std::sync::Mutex::new(cgmath::Matrix4::identity()));
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
    let light_pos_location = context.get_uniform_location(&program, "lightPos").unwrap();
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
        light_pos_location,
        mouse,
        camera_rotate_matrix,
        camera_offset_matrix: cgmath::Matrix4::from_translation(cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: -3.0,
        }),
    }
}
#[wasm_bindgen]
pub fn andy_main() {
    set_canvas("big_canvas", 3, false, false, true, FragLevel::BeachBall);
    set_canvas("small_canvas0", 0, true, false, false, FragLevel::None);
    set_canvas("small_canvas1", 0, true, true, false, FragLevel::None);
    set_canvas(
        "small_canvas2",
        0,
        true,
        true,
        false,
        FragLevel::Transparent,
    );
    set_canvas(
        "small_canvas3",
        1,
        true,
        true,
        false,
        FragLevel::Transparent,
    );
    set_canvas(
        "small_canvas4",
        2,
        true,
        true,
        false,
        FragLevel::Transparent,
    );
    set_canvas(
        "small_canvas5",
        3,
        true,
        true,
        false,
        FragLevel::Transparent,
    );
    set_canvas("small_canvas6", 3, false, true, false, FragLevel::Full);
    set_canvas("small_canvas7", 3, false, true, true, FragLevel::Reflective);
    set_canvas("small_canvas8", 3, false, true, true, FragLevel::Spectral);
    set_canvas("small_canvas9", 3, false, false, true, FragLevel::Spectral);
    set_canvas(
        "small_canvas10",
        3,
        false,
        false,
        true,
        FragLevel::BeachBall,
    );
}

fn set_canvas(
    name: &str,
    iters: u32,
    rectangles: bool,
    wires: bool,
    light: bool,
    frags: FragLevel,
) {
    let mut verts: Vec<f32> = GOLDEN_RECTANGLE_VERTS.to_vec();
    let mut ico_verts = icosahedron::generate_verticies();
    for _ in 0..iters {
        ico_verts = icosahedron::sphere_recurse_verts(ico_verts);
    }

    verts.extend(
        ico_verts
            .iter()
            .flat_map(|t| t.iter().flat_map(|v| [v.x, v.y, v.z]).collect::<Vec<f32>>())
            .collect::<Vec<f32>>(),
    );
    let (_context, _program) = andy_webgl_utils::setup_canvas(
        name,
        andy_webgl_utils::ShaderProg {
            vert_shader_source: include_str!("./shaders_source/vertex.glsl"),
            frag_shader_source: include_str!("./shaders_source/frag.glsl"),
            verts: &verts,
            vertex_attrib_name: "position",
            draw_func: move |globals: Globals| {
                draw(
                    globals,
                    DrawConfig {
                        light,
                        rectangles,
                        wires,
                        frags,
                        num_triangles: 20 * (4_i32.pow(iters)),
                    },
                )
            },
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
        let mut camera_rotate_matrix = globals.camera_rotate_matrix.lock().unwrap();
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
            *camera_rotate_matrix = cgmath::Matrix4::from_axis_angle(
                normalize(cross),
                cgmath::Rad(length(cross) / 200.0),
            ) * (*camera_rotate_matrix);
        }
        globals.context.uniform_matrix4fv_with_f32_array(
            Some(&globals.camera_matrix_location),
            false,
            &matrix_to_vec(globals.camera_offset_matrix * *camera_rotate_matrix),
        );
    }

    let model_matrix = cgmath::Matrix4::identity();

    if config.rectangles {
        draw_rectangle(&globals, model_matrix, FragEnum::Red);
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
        draw_rectangle(&globals, model_matrix * rotated_1, FragEnum::Green);

        draw_rectangle(&globals, model_matrix, FragEnum::Red);
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
        draw_rectangle(&globals, model_matrix * rotated_2, FragEnum::Blue);
    }

    if config.light {
        let light_pos = (cgmath::Matrix4::from_axis_angle(
            normalize(cgmath::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }),
            cgmath::Rad((((current_time_milis % 3000) as f32) / 3000.0) * std::f32::consts::TAU),
        ) * cgmath::Vector4 {
            x: 2.5,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        })
        .truncate();
        globals.context.uniform3f(
            Some(&globals.light_pos_location),
            light_pos.x,
            light_pos.y,
            light_pos.z,
        );

        let poo = cgmath::Matrix4::from_translation(light_pos) * cgmath::Matrix4::from_scale(0.2);
        draw_triangles_frags(
            &globals,
            poo,
            Some(FragEnum::White),
            6,
            config.num_triangles,
            false,
        );
    }

    let bigger = cgmath::Matrix4::from_scale(((PHI * PHI) + (1.0 * 1.0)).sqrt());
    draw_triangles_frags(
        &globals,
        model_matrix * bigger,
        match config.frags {
            FragLevel::Transparent => Some(FragEnum::ClearRed),
            FragLevel::None => None,
            FragLevel::Full => Some(FragEnum::Red),
            FragLevel::Reflective => Some(FragEnum::RedReflective),
            FragLevel::Spectral => Some(FragEnum::RedSpectral),
            FragLevel::BeachBall => Some(FragEnum::BeachBall),
        },
        6,
        config.num_triangles,
        config.wires,
    );
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
fn draw_triangles_frags(
    globals: &Globals,
    model_mat: cgmath::Matrix4<f32>,
    frag_enum: Option<FragEnum>,
    start: i32,
    count: i32,
    do_wires: bool,
) {
    globals.context.uniform_matrix4fv_with_f32_array(
        Some(&globals.model_matrix_location),
        false,
        &matrix_to_vec(model_mat),
    );
    if let Some(frag_enum) = frag_enum {
        if TRIANGLE_FRAGS {
            globals
                .context
                .uniform1ui(Some(&globals.frag_enum_location), frag_enum as u32);
            globals.context.draw_arrays(
                web_sys::WebGl2RenderingContext::TRIANGLES,
                start,
                count * 3,
            );
        }
    }
    if do_wires {
        for i in 0..count {
            globals
                .context
                .uniform1ui(Some(&globals.frag_enum_location), FragEnum::Black as u32);
            globals.context.draw_arrays(
                web_sys::WebGl2RenderingContext::LINE_LOOP,
                start + (3 * i),
                3,
            );
        }
    }
}
