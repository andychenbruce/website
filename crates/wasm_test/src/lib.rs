use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn andy_main() -> Result<(), JsValue> {
    web_sys::console::log_1(&"hi lol\n".into());
    setup_canvas().await;
    Ok(())
}
thread_local!(
    static LAST_TICK_TIME: std::cell::RefCell<f64> = std::cell::RefCell::new(0.0);
);

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
    context.clear_color(0.1, 0.1, 0.9, 1.0);
    context.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT);

    let vert_shader_source = include_str!("./shaders_source/vertex.glsl");
    let frag_shader_source = include_str!("./shaders_source/frag.glsl");

    let program = make_program(&context, vert_shader_source, frag_shader_source);
    context.use_program(Some(&program));

    let buffer = context.create_buffer().unwrap();
    context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    let vertices: &[f32] = &[-0.2, 0.5, 0.0, -0.5, -0.4, 0.0, 0.5, -0.1, 0.0];

    let positions_array_buf_view = unsafe { js_sys::Float32Array::view(vertices) };
    context.buffer_data_with_array_buffer_view(
        web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
        &positions_array_buf_view,
        web_sys::WebGl2RenderingContext::STATIC_DRAW,
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

    context.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 3) as i32;

    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        draw(&context, vert_count);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn draw(context: &web_sys::WebGl2RenderingContext, vert_count: i32) {
    context.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT);
    context.draw_arrays(web_sys::WebGl2RenderingContext::TRIANGLES, 0, vert_count);
    let current_time_milis = js_sys::Date::new_0().get_time();
    LAST_TICK_TIME.with(|f| {
        let diff_milis = current_time_milis - *f.borrow();
        web_sys::console::log_1(&(format!("drew stuff diff: {}", diff_milis)).into());
        *f.borrow_mut() = current_time_milis
    });
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

#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}
