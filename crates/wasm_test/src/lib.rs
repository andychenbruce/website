mod icosahedron;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub async fn andy_main() -> Result<(), JsValue> {
    setup_canvas().await;
    Ok(())
}

struct Globals {
    last_tick_time: u64,
    context: web_sys::WebGl2RenderingContext,
    model_matrix_location: web_sys::WebGlUniformLocation,
    frag_enum_location: web_sys::WebGlUniformLocation,
}

async fn setup_canvas() {
    let context: web_sys::WebGl2RenderingContext = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("my_canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    context.clear_color(0.8, 0.6, 0.0, 1.0);
    context.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT);
    context.enable(web_sys::WebGl2RenderingContext::DEPTH_TEST);
    context.enable(web_sys::WebGl2RenderingContext::BLEND);

    context.blend_func(
        web_sys::WebGl2RenderingContext::SRC_ALPHA,
        web_sys::WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
    );
    let vert_shader_source = include_str!("./shaders_source/vertex.glsl");
    let frag_shader_source = include_str!("./shaders_source/frag.glsl");

    let program = make_program(&context, vert_shader_source, frag_shader_source);
    context.use_program(Some(&program));

    let buffer = context.create_buffer().unwrap();
    context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    let mut verts: Vec<f32> = GOLDEN_RECTANGLE_VERTS.to_vec();
    let ico_verts = icosahedron::sphere_recurse_verts(icosahedron::generate_verticies());
    verts.extend(
        ico_verts
            .iter()
            .flat_map(|t| t.iter().flat_map(|v| [v.x, v.y, v.z]).collect::<Vec<f32>>())
            .collect::<Vec<f32>>(),
    );

    context.buffer_data_with_u8_array(
        web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
        &floats_as_bytes(&verts),
        web_sys::WebGl2RenderingContext::STATIC_DRAW,
    );

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

    let mut globals = Globals {
        last_tick_time: 0,
        context,
        model_matrix_location,
        frag_enum_location,
    };

    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        draw(&mut globals);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn draw(globals: &mut Globals) {
    globals
        .context
        .clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT);

    let current_time_milis = js_sys::Date::new_0().get_time() as u64;
    globals.last_tick_time = current_time_milis;
    let divisor: f64 = 1000.0;
    let radians =
        ((current_time_milis as f64).rem_euclid(std::f64::consts::TAU * divisor) / divisor) as f32;

    let rotate_vector = cgmath::Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let forward_rotated =
        cgmath::Matrix4::from_translation(cgmath::Vector3 {
            x: 0.0,
            y: -0.5,
            z: -3.0,
        }) * cgmath::Matrix4::from_axis_angle(normalize(rotate_vector), cgmath::Rad(radians));

    draw_rectangle(globals, forward_rotated, FragEnum::Red);
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
    draw_rectangle(globals, forward_rotated * rotated_1, FragEnum::Green);

    draw_rectangle(globals, forward_rotated, FragEnum::Red);
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
    draw_rectangle(globals, forward_rotated * rotated_2, FragEnum::Blue);

    let bigger = cgmath::Matrix4::from_scale(((PHI * PHI) + (1.0 * 1.0)).sqrt());
    draw_triangle(globals, forward_rotated * bigger, FragEnum::ClearBlue);
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().dyn_ref().unwrap())
        .unwrap()
}

fn make_program(
    context: &web_sys::WebGl2RenderingContext,
    vert_shader_code: &str,
    frag_shader_code: &str,
) -> web_sys::WebGlProgram {
    let vert_shader = make_shader(
        context,
        web_sys::WebGl2RenderingContext::VERTEX_SHADER,
        vert_shader_code,
    );

    let frag_shader = make_shader(
        context,
        web_sys::WebGl2RenderingContext::FRAGMENT_SHADER,
        frag_shader_code,
    );

    let program = context.create_program().unwrap();
    context.attach_shader(&program, &vert_shader);
    context.attach_shader(&program, &frag_shader);
    context.link_program(&program);
    program
}

fn make_shader(
    context: &web_sys::WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> web_sys::WebGlShader {
    let shader = context.create_shader(shader_type).unwrap();

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    let shader_log = context.get_shader_info_log(&shader).unwrap();

    if !shader_log.is_empty() {
        web_sys::console::warn_1(&format!("shader compilation has errors: {}", shader_log).into());
    }
    if context
        .get_shader_parameter(&shader, web_sys::WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap()
    {
        shader
    } else {
        web_sys::console::error_1(
            &context
                .get_shader_info_log(&shader)
                .unwrap()
                .as_str()
                .into(),
        );
        panic!("compiling shader failed")
    }
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
    let len_squared: f32 = (vec.x * vec.x) + (vec.y * vec.y) + (vec.z * vec.z);

    vec / (len_squared.sqrt())
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
            .draw_arrays(web_sys::WebGl2RenderingContext::TRIANGLES, 6, 240);
    }
    for i in 0..80 {
        globals
            .context
            .uniform1ui(Some(&globals.frag_enum_location), FragEnum::Black as u32);
        globals
            .context
            .draw_arrays(web_sys::WebGl2RenderingContext::LINE_LOOP, 6 + (3 * i), 3);
    }
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

fn floats_as_bytes(floats: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(4 * floats.len());
    for f in floats {
        bytes.extend(f.to_le_bytes());
    }
    bytes
}
