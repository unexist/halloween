///
/// @package halloween
///
/// @file App functions
/// @copyright 2026-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, HtmlImageElement, WebGlRenderingContext as GL, WebGlRenderingContext, WebGlTexture};
use yew::{html, Component, Context, Html, NodeRef};
use regex::Regex;

#[derive(Default)]
pub struct App {
    node_ref: NodeRef,
    width: u32,
    height: u32,
    is_mobile: bool,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let win = window().expect("Window should be available");
        let maybe_useragent = win.navigator().user_agent();
        let regex = Regex::new("Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini")
            .expect("Regex failed");

        Self {
            width: (win.inner_width().expect("Inner width unknown?").as_f64().unwrap()
                * win.device_pixel_ratio()) as u32,
            height: (win.inner_height().expect("Inner height unknown?").as_f64().unwrap()
                * win.device_pixel_ratio()) as u32,
            is_mobile: maybe_useragent.is_ok() && regex.is_match(&*maybe_useragent.unwrap()),
            ..Default::default()
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div id="wrapper">
                    <div>
                        <h1>{ "Welcome to Halloween Land!" }</h1>
                        <div id="box">
                            <h3>{ "Wann: 31.10. / 14:00" }</h3>
                            <h3>{ "Wie: Verkleidet!" }</h3>
                            <h3>{ "Wo: Essen" }</h3>
                        </div>
                    </div>
                    <div id="disclaimer">
                        <div>{ "Best viewed in a desktop browser! (Moar shader, Moar effects, Moar all!)" }</div>
                    </div>
                </div>
                <div id="save">{ "Save the date!" }</div>
                <canvas ref={self.node_ref.clone()} />
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        // Create canvas
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        canvas.set_width(self.width);
        canvas.set_height(self.height);

        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.render_gl(gl);
    }
}

impl App {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn create_texture(gl: &WebGlRenderingContext) -> WebGlTexture {
        let texture = gl.create_texture().unwrap();

        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);

        texture
    }

    fn render_gl(&self, gl: WebGlRenderingContext) {
        let vert_code = include_str!("./hw.vert");
        let frag_code = include_str!("./hw.frag");

        // Create a buffer for the squares's positions
        let vertices: Vec<f32> = vec![
            -1.0, -1.0,
            1.0, -1.0,
            -1.0, 1.0,
            -1.0, 1.0,
            1.0, -1.0,
            1.0, 1.0,
        ];
        let vertex_buffer = gl.create_buffer().unwrap();
        let vertex_ary = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertex_ary, GL::STATIC_DRAW);

        // Compile vert shader
        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();

        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        // Compile frag shader
        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();

        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);

        // Create shader program
        let shader_program = gl.create_program().unwrap();

        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);

        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        // Set up position attribute
        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;

        gl.enable_vertex_attrib_array(position);
        gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);

        // Load and set up image texture
        let tex_image = Self::create_texture(&gl);

        let image = HtmlImageElement::new().unwrap();

        image.set_src("images/skeletons.png");

        // Gloo-render's request_animation_frame has this extra closure
        // wrapping logic running every frame, unnecessary cost.
        // Here constructing the wrapped closure just once.

        let cb = Rc::new(RefCell::new(None));

        *cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let cb = cb.clone();
            let width = self.width;
            let height = self.height;
            let is_mobile = self.is_mobile;

            let performance = window().expect("Window should be available")
                .performance()
                .expect("Performance should be available");

            move || {
                // This should repeat every frame
                gl.bind_texture(GL::TEXTURE_2D, Some(&tex_image));
                gl.tex_image_2d_with_u32_and_u32_and_image(GL::TEXTURE_2D, 0, GL::RGBA as i32, GL::RGBA,
                                                           GL::UNSIGNED_BYTE, &image).expect("Failed to set image");

                gl.viewport(0, 0, width as i32, height as i32);

                gl.bind_framebuffer(GL::FRAMEBUFFER, None);

                if let Some(u_resolution) = &gl.get_uniform_location(&shader_program, "u_resolution") {
                    gl.uniform2f(Some(u_resolution), width as f32, height as f32);
                }

                let img_size_x = if is_mobile { width as f32 * 0.5 } else { 2000f32 };
                let img_size_y = if is_mobile { height as f32 * 0.5 } else { 1024f32 };

                if let Some(u_image_size) =  &gl.get_uniform_location(&shader_program, "u_imageSize") {
                    gl.uniform2fv_with_f32_array(Some(u_image_size), &[img_size_x, img_size_y]);
                }

                let current_time = performance.now() * 0.001;

                if let Some(u_time) = &gl.get_uniform_location(&shader_program, "u_time") {
                    gl.uniform1f(Some(u_time), current_time as f32);
                }

                if let Some(u_is_mobile) = &gl.get_uniform_location(&shader_program, "u_isMobile") {
                    gl.uniform1i(Some(u_is_mobile), if is_mobile { 1 } else { 0 });
                }

                gl.active_texture(GL::TEXTURE1);
                gl.bind_texture(GL::TEXTURE_2D, Some(&tex_image));

                if let Some(i_channel0) = &gl.get_uniform_location(&shader_program, "iChannel0") {
                    gl.uniform1i(Some(i_channel0), 0);
                }

                gl.draw_arrays(GL::TRIANGLES, 0, 6);

                App::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));

        App::request_animation_frame(cb.borrow().as_ref().unwrap());
    }
}
