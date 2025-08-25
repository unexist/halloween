///
/// @package halloween
///
/// @file App functions
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, HtmlImageElement, WebGlFramebuffer, WebGlRenderingContext as GL, WebGlRenderingContext, WebGlTexture};
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
        let win = window().expect("Window must be present");
        let maybe_useragent = win.navigator().user_agent();
        let regex = Regex::new("Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini").expect("Regex failed");

        Self {
            width: (win.inner_width().unwrap().as_f64().unwrap() * win.device_pixel_ratio()) as u32,
            height: (win.inner_height().unwrap().as_f64().unwrap() * win.device_pixel_ratio()) as u32,
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
                        <h2>{ "Auch dieses Jahr oeffnen sich die Pforten" }</h2>
                        <h2>{ "Es stehen kaltes Bier und allerlei toedliche Speisen bereit!" }</h2>
                        <div id="box">
                            <h3>{ "Wann: 31.10." }</h3>
                            <h3>{ "Wie: Verkleidet!" }</h3>
                            <h3>{ "Wo: Essen" }</h3>
                        </div>
                    </div>
                </div>
                <canvas ref={self.node_ref.clone()} />
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        // Only start the render loop if it's the first render
        // There's no loop cancellation taking place, so if multiple renders happen,
        // there would be multiple loops running. That doesn't *really* matter here because
        // there's no props update and no SSR is taking place, but it is something to keep in
        // consideration
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

    fn init_texture(gl: &WebGlRenderingContext, texture: &WebGlTexture, width: u32, height: u32) {
        gl.bind_texture(GL::TEXTURE_2D, Some(texture));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(GL::TEXTURE_2D, 0, GL::RGBA as i32,
                                                                                               width as i32, height as i32, 0, GL::RGBA,
                                                                                               GL::UNSIGNED_BYTE, None)
            .expect("Failed to create texture");
    }

    fn create_framebuffer(gl: &WebGlRenderingContext, texture: &WebGlTexture) -> WebGlFramebuffer {
        let framebuffer = gl.create_framebuffer().unwrap();

        gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&framebuffer));
        gl.framebuffer_texture_2d(GL::FRAMEBUFFER, GL::COLOR_ATTACHMENT0, GL::TEXTURE_2D, Some(texture), 0);

        framebuffer
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
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

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

        // Create textures and framebuffers for green and red shaders
        let texture_green = Self::create_texture(&gl);

        Self::init_texture(&gl, &texture_green, self.width, self.height);

        let _framebuffer_green = Self::create_framebuffer(&gl, &texture_green);

        let texture_red = Self::create_texture(&gl);

        Self::init_texture(&gl, &texture_red, self.width, self.height);

        let _framebuffer_red = Self::create_framebuffer(&gl, &texture_red);

        // Load and set up image texture
        let tex_image = Self::create_texture(&gl);

        let image = HtmlImageElement::new().unwrap();

        image.set_src(if self.is_mobile { "images/sprites.png" }  else { "images/circle.png" });

        // Gloo-render's request_animation_frame has this extra closure
        // wrapping logic running every frame, unnecessary cost.
        // Here constructing the wrapped closure just once.

        let cb = Rc::new(RefCell::new(None));

        *cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let cb = cb.clone();
            let width = self.width;
            let height = self.height;
            let is_mobile = self.is_mobile;

            let performance = web_sys::window().unwrap()
                .performance().expect("Performance should be available");

            move || {
                // This should repeat every frame
                gl.bind_texture(GL::TEXTURE_2D, Some(&tex_image));
                gl.tex_image_2d_with_u32_and_u32_and_image(GL::TEXTURE_2D, 0, GL::RGBA as i32, GL::RGBA,
                                                           GL::UNSIGNED_BYTE, &image).expect("Failed to set image");

                gl.viewport(0, 0, width as i32, height as i32);

                gl.bind_framebuffer(GL::FRAMEBUFFER, None);
                gl.uniform2f(Some(&gl.get_uniform_location(&shader_program, "u_resolution").unwrap()),
                             width as f32, height as f32);
                gl.uniform2fv_with_f32_array(Some(&gl.get_uniform_location(&shader_program, "u_imageSize").unwrap()),
                                             if is_mobile { &[200 as f32, 102 as f32] } else { &[2000 as f32, 1024 as f32] });

                let current_time = performance.now() * 0.001;

                gl.uniform1f(Some(&gl.get_uniform_location(&shader_program, "u_time").unwrap()),
                             current_time as f32);

                //let aspect_ratio = (width / height) as f32;

                //gl.uniform1f(Some(&gl.get_uniform_location(&shader_program, "u_aspectRatio").unwrap()), aspect_ratio);

                gl.uniform1i(Some(&gl.get_uniform_location(&shader_program, "u_isMobile").unwrap()),
                             if is_mobile { 1 } else { 0 });

                gl.active_texture(GL::TEXTURE1);
                gl.bind_texture(GL::TEXTURE_2D, Some(&tex_image));
                gl.uniform1i(Some(&gl.get_uniform_location(&shader_program, "iChannel0").unwrap()), 0);
                gl.draw_arrays(GL::TRIANGLES, 0, 6);

                App::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));

        App::request_animation_frame(cb.borrow().as_ref().unwrap());
    }
}
