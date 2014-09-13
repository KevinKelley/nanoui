#![feature(globs)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(unsafe_destructor)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variable)]

extern crate native;
extern crate glfw;
extern crate gl;

extern crate nanovg;
extern crate nanoui;

use glfw::Context;
use std::gc::{Gc,GC};
use std::cell::Cell;

use nanovg::{Ctx, ANTIALIAS,STENCIL_STROKES};
use nanoui::blendish::theme::ThemedContext;
use nanoui::oui::Context as UIContext;

use resources::Resources;
use ui::Widget;

//mod blendish;
//mod oui;
mod ui;
mod resources;


/// evaluate the expression, then check for GL error.
macro_rules! glcheck(
    ($e: expr) => (
        {
            $e;
            assert_eq!(gl::GetError(), 0);
        }
    )
)

///////////////////////////////////////////////////////////////////////
// AppData (some simulated state for UI to modify)

#[deriving(Show)]
pub struct AppData {
    // some persistent variables for demonstration
    pub enum1:     Gc<Cell<i32>>,
    pub progress1: Gc<Cell<f32>>,
    pub progress2: Gc<Cell<f32>>,
    pub option1:  Gc<Cell<bool>>,
    pub option2:  Gc<Cell<bool>>,
    pub option3:  Gc<Cell<bool>>,
}
pub fn init_app_data() -> AppData {
    // fake load-from-storage
    AppData {
        enum1:     box (GC) Cell::new(0),
        progress1: box (GC) Cell::new(0.25),
        progress2: box (GC) Cell::new(0.75),
        option1:   box (GC) Cell::new(true),
        option2:   box (GC) Cell::new(false),
        option3:   box (GC) Cell::new(false),
    }
}
#[unsafe_destructor]
impl Drop for AppData {
    fn drop(&mut self) {
        // fake save-to-storage
        println!("drop appdata {}", self);
    }
}
///////////////////////////////////////////////////////////////////////

pub struct App<'a> {
    mouse: (i32,i32),           // current mouse pos
    button: bool,               // is mousebutton pressed
    elapsed_time: f64,          // seconds since app start
    data: AppData,
    themed: ThemedContext<'a>,  // wrap nvg ctx w/ themed-draw fns
    ui: UIContext<Widget>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let themed = {
            let nvg = Ctx::create_gl3(ANTIALIAS|STENCIL_STROKES);
            let resources = Resources::load(&nvg, "./res");
            let font = resources.fontNormal;
            let icons = resources.iconsheet;
            // move nvg & resources into the ThemedContext
            ThemedContext::wrap(nvg, icons, font)
        };
        App {
            mouse: (0,0),
            button: false,
            elapsed_time: 0.0,         // time since app start
            data: init_app_data(),
            themed: themed,
            ui: ui::create(),
        }
    }
    fn nvg(&mut self) -> &mut Ctx { self.themed.nvg() }

    // life cycle methods

    fn load(&mut self) {
        ui::init(self);
    }

    fn update(&mut self, dt: f64) {
        self.elapsed_time += dt;
        ui::update(&mut self.ui, self.mouse, self.button, self.elapsed_time as f32);
    }

    fn render(&mut self, w:i32, h:i32, pxRatio: f32) {
        let (w,  h) = (w as f32, h as f32);

        self.nvg().begin_frame(w as i32, h as i32, pxRatio);

        ui::draw(&mut self.ui, &mut self.themed, w,h);

        self.nvg().end_frame();
    }
}



#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {

    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::ContextVersion(3, 2));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
    glfw.window_hint(glfw::OpenglDebugContext(true));
    let (window, events) = glfw.create_window(600,480, "nanoUI demo", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_sticky_keys(true);
    window.set_all_polling(true);
    window.make_current();

    // use glfw to load GL function pointers
    glcheck!(gl::load_with(|name| glfw.get_proc_address(name)));
    //init_gl();

    //glfw.set_swap_interval(0);
    glfw.set_time(0.0);
    let mut prevt = glfw.get_time();

    let mut app = App::new();
    app.load();

    while !window.should_close()
    {
        // get current timestamp and delta
        let t: f64 = glfw.get_time();
        let dt: f64 = t - prevt;
        prevt = t;

        // process outstanding window events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, &mut app, (t, event));
        }

        let (mousex, mousey) = window.get_cursor_pos(); // (f64,f64)
        let (winWidth, winHeight) = window.get_size();  // (i32,i32)
        let (fbWidth, fbHeight) = window.get_framebuffer_size();
        // Calculate pixel ration for hi-dpi devices.
        let pxRatio = fbWidth as f32 / winWidth as f32;

        // clear framebuffer
        glcheck!(gl::Viewport(0, 0, fbWidth, fbHeight));
        glcheck!(gl::ClearColor(0.0, 0.0, 0.0, 0.0));
        glcheck!(gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT|gl::STENCIL_BUFFER_BIT));

        // shrug
        glcheck!(gl::Enable(gl::BLEND));
        glcheck!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
        glcheck!(gl::Enable(gl::CULL_FACE));
        glcheck!(gl::Disable(gl::DEPTH_TEST));

        // Update ui, and render to framebuffer
        app.update(dt);
        app.render(winWidth, winHeight, pxRatio);

        glcheck!(gl::Enable(gl::DEPTH_TEST));

        // swap in the freshened buffer
        window.swap_buffers();
    }
}


fn handle_window_event(
    window: &glfw::Window,
    app: &mut App,
    (time, event): (f64, glfw::WindowEvent)
) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => window.set_should_close(true),
        glfw::MouseButtonEvent(_, glfw::Press, _) => app.button = true,
        glfw::MouseButtonEvent(_, glfw::Release, _) => app.button = false,
        glfw::CursorPosEvent(xpos, ypos)    => app.mouse = (xpos as i32, ypos as i32),

        glfw::PosEvent(x, y)                => println!("Time: {}, Window pos: ({}, {})", time, x, y),
        glfw::SizeEvent(w, h)               => println!("Time: {}, Window size: ({}, {})", time, w, h),
        glfw::CloseEvent                    => println!("Time: {}, Window close requested.", time),
        glfw::RefreshEvent                  => println!("Time: {}, Window refresh callback triggered.", time),
        glfw::FocusEvent(true)              => println!("Time: {}, Window focus gained.", time),
        glfw::FocusEvent(false)             => println!("Time: {}, Window focus lost.", time),
        glfw::IconifyEvent(true)            => println!("Time: {}, Window was minimised", time),
        glfw::IconifyEvent(false)           => println!("Time: {}, Window was maximised.", time),
        glfw::FramebufferSizeEvent(w, h)    => println!("Time: {}, Framebuffer size: ({}, {})", time, w, h),
        glfw::CharEvent(character)          => println!("Time: {}, Character: {}", time, character),
        glfw::MouseButtonEvent(btn, action, mods) => println!("Time: {}, Button: {}, Action: {}, Modifiers: [{}]", time, glfw::ShowAliases(btn), action, mods),
        //glfw::CursorPosEvent(xpos, ypos)    => window.set_title(format!("Time: {}, Cursor position: ({}, {})", time, xpos, ypos).as_slice()),
        glfw::CursorEnterEvent(true)        => println!("Time: {}, Cursor entered window.", time),
        glfw::CursorEnterEvent(false)       => println!("Time: {}, Cursor left window.", time),
        glfw::ScrollEvent(x, y)             => window.set_title(format!("Time: {}, Scroll offset: ({}, {})", time, x, y).as_slice()),
        glfw::KeyEvent(key, scancode, action, mods) => {
            //println!("Time: {}, Key: {}, ScanCode: {}, Action: {}, Modifiers: [{}]", time, key, scancode, action, mods);
            match (key, action) {
                (glfw::KeyEscape, glfw::Press) => window.set_should_close(true),
                (glfw::KeyR, glfw::Press) => {
                    // Resize should cause the window to "refresh"
                    let (window_width, window_height) = window.get_size();
                    window.set_size(window_width + 1, window_height);
                    window.set_size(window_width, window_height);
                }
                _ => {}
            }
        }
    }
}
