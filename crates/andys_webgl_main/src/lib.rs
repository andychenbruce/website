use wasm_bindgen::prelude::*;

pub struct ShaderProg<'a, T> {
    pub vert_shader_source: &'a str,
    pub frag_shader_source: &'a str,
    pub verts: &'a [f32],
    pub vertex_attrib_name: &'a str,
    pub draw_func: T,
}

pub fn setup_canvas<T, G, F, C1, C2, C3>(
    canvas_id: &str,
    shader_prog: ShaderProg<T>,
    mousedown_callback: Option<C1>,
    mouseup_callback: Option<C2>,
    mousemove_callback: Option<C3>,
    make_globals: F,
) -> (web_sys::WebGl2RenderingContext, web_sys::WebGlProgram)
where
    T: FnMut(G) + 'static,
    C1: FnMut(G, web_sys::Event) + 'static,
    C2: FnMut(G, web_sys::Event) + 'static,
    C3: FnMut(G, web_sys::Event) + 'static,
    G: Clone + 'static,
    F: FnOnce(web_sys::WebGl2RenderingContext, web_sys::WebGlProgram) -> G,
{
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context: web_sys::WebGl2RenderingContext = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    context.clear_color(0.2, 0.4, 0.5, 1.0);
    context.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT);
    context.enable(web_sys::WebGl2RenderingContext::DEPTH_TEST);
    context.enable(web_sys::WebGl2RenderingContext::BLEND);

    context.blend_func(
        web_sys::WebGl2RenderingContext::SRC_ALPHA,
        web_sys::WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
    );
    let program = make_program(
        &context,
        shader_prog.vert_shader_source,
        shader_prog.frag_shader_source,
    );
    context.use_program(Some(&program));

    let buffer = context.create_buffer().unwrap();
    context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    context.buffer_data_with_u8_array(
        web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
        &floats_as_bytes(shader_prog.verts),
        web_sys::WebGl2RenderingContext::STATIC_DRAW,
    );

    let vao = context.create_vertex_array().unwrap();
    context.bind_vertex_array(Some(&vao));

    let position_attrib = context.get_attrib_location(&program, shader_prog.vertex_attrib_name);
    context.vertex_attrib_pointer_with_i32(
        position_attrib as u32,
        3,
        web_sys::WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    context.enable_vertex_attrib_array(position_attrib as u32);

    setup_callbacks(
        make_globals(context.clone(), program.clone()),
        &canvas,
        shader_prog.draw_func,
        mousedown_callback,
        mouseup_callback,
        mousemove_callback,
    );

    (context, program)
}

fn setup_callbacks<T, G, C1, C2, C3>(
    globals: G,
    canvas: &web_sys::HtmlCanvasElement,
    mut draw_func: T,
    mousedown_callback: Option<C1>,
    mouseup_callback: Option<C2>,
    mousemove_callback: Option<C3>,
) where
    T: FnMut(G) + 'static,
    C1: FnMut(G, web_sys::Event) + 'static,
    C2: FnMut(G, web_sys::Event) + 'static,
    C3: FnMut(G, web_sys::Event) + 'static,
    G: Clone + 'static,
{
    setup_callback(globals.clone(), canvas, "mousedown", mousedown_callback);
    setup_callback(globals.clone(), canvas, "mouseup", mouseup_callback);
    setup_callback(globals.clone(), canvas, "mousemove", mousemove_callback);

    //draw loop
    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        draw_func(globals.clone());
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn setup_callback<C, G>(
    globals: G,
    canvas: &web_sys::HtmlCanvasElement,
    callback_type: &str,
    callback: Option<C>,
) where
    C: FnMut(G, web_sys::Event) + 'static,
    G: Clone + 'static,
{
    if let Some(mut callback) = callback {
        let andy_callback =
            Closure::wrap(
                Box::new(move |e: web_sys::Event| callback(globals.clone(), e))
                    as Box<dyn FnMut(_)>,
            );
        canvas
            .add_event_listener_with_callback(
                callback_type,
                andy_callback.as_ref().dyn_ref().unwrap(),
            )
            .unwrap();
        std::mem::forget(andy_callback); //mem leak, too bad
    }
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
