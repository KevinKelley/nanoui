#![feature(globs)]
#![feature(macro_rules)]

extern crate native;
extern crate glfw;
extern crate gl;

extern crate nanovg;
extern crate nanoui;

use glfw::Context as GLFWContext;
use std::gc::{Gc,GC};
use std::cell::Cell;

use nanovg::{Ctx, Image, Font, ANTIALIAS,STENCIL_STROKES};
use nanoui::blendish::theme::ThemedContext;
use nanoui::blendish::themed_draw::ThemedDraw;
use nanoui::blendish::widget::*;
use nanoui::oui::Context as OUIContext;
use nanoui::oui::{LEFT,TOP,HFILL};

use nanoui::draw::iconsheet::{icon_id, no_icon};

mod macros;


///////////////////////////////////////////////////////////////////////
// resource loading, for demo
pub struct Resources {
    pub font_normal: Font,
    pub iconsheet: Image
}

/// load and hold resources used in demo
impl Resources {
    pub fn load(vg: &Ctx, res_path: &str) -> Resources
    {
        let filename = format!("{}/blender_icons16.png", res_path);
        let icons = vg.create_image(filename.as_slice())
            .expect(format!("Couldn't load icons image from '{}'", filename).as_slice());

        let filename = format!("{}/DejaVuSans.ttf", res_path);
        let font = vg.create_font("sans", filename.as_slice())
            .expect(format!("Could not load font from '{}'", filename).as_slice());

        Resources {
            font_normal: font,
            iconsheet:  icons
        }
    }
}

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
//#[unsafe_destructor]
//impl Drop for AppData {
//    fn drop(&mut self) {
//        // fake save-to-storage
//        println!("drop appdata {}", self);
//    }
//}
///////////////////////////////////////////////////////////////////////

pub struct App<'a> {
    mouse: (i32,i32),           // current mouse pos
    button: bool,               // is mousebutton pressed
    elapsed_time: f64,          // seconds since app start
    data: AppData,
    themed: ThemedContext<'a>,  // wrap nvg ctx w/ themed-draw fns
    ui: OUIContext<Widget>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let themed = {
            let nvg = Ctx::create_gl3(ANTIALIAS|STENCIL_STROKES);
            let resources = Resources::load(&nvg, "./res");
            let font = resources.font_normal;
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
            ui: create(),
        }
    }
    fn nvg(&mut self) -> &mut Ctx { self.themed.nvg() }

    // life cycle methods

    fn load(&mut self) {
        init(self);
    }

    fn update(&mut self, dt: f64) {
        self.elapsed_time += dt;
        update(&mut self.ui, self.mouse, self.button, self.elapsed_time as f32);
    }

    fn render(&mut self, w:i32, h:i32, px_ratio: f32) {
        let (w,  h) = (w as f32, h as f32);

        self.nvg().begin_frame(w as i32, h as i32, px_ratio);

        draw(&mut self.ui, &mut self.themed, w,h);

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
    glcheck!(gl::load_with(|name| window.get_proc_address(name)));

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

        let (win_width, win_height) = window.get_size();  // (i32,i32)
        let (fb_width, fb_height) = window.get_framebuffer_size();
        // Calculate pixel ration for hi-dpi devices.
        let px_ratio = fb_width as f32 / win_width as f32;

        // clear framebuffer
        glcheck!(gl::Viewport(0, 0, fb_width, fb_height));
        glcheck!(gl::ClearColor(0.0, 0.0, 0.0, 0.0));
        glcheck!(gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT|gl::STENCIL_BUFFER_BIT));

        // shrug
        glcheck!(gl::Enable(gl::BLEND));
        glcheck!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
        glcheck!(gl::Enable(gl::CULL_FACE));
        glcheck!(gl::Disable(gl::DEPTH_TEST));

        // Update ui, and render to framebuffer
        app.update(dt);
        app.render(win_width, win_height, px_ratio);

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
        glfw::KeyEvent(key, _scancode, action, _mods) => {
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


///////////////////////////////////////////////////////////////////////
// ui

pub fn create() -> OUIContext<Widget> {
    OUIContext::create_context()
}

pub fn init(app: &mut App) {

    let ui = &mut app.ui;

    // setup the UI

    ui.clear(); // removes any previous items, currently will break
                // if multiple-re-init.

    // build the ui hierarchy: start at root,
    // compose elements into nested groups that flow

    let root = panel(ui);
    // position root element
    ui.set_layout(root, LEFT|TOP);
    ui.set_margins(root, 60, 10, 0, 0);
    ui.set_size(root, 450, 400);

    let col = column(ui, root);
    ui.set_margins(col, 10, 10, 10, 10);
    ui.set_layout(col, TOP|HFILL);

    button(ui, col, 1, icon_id(6, 3), "Item 1", Some(demohandler));
    button(ui, col, 2, icon_id(6, 3), "Item 2", Some(demohandler));

    {
        let h = hgroup(ui, col);
        radio(ui, h, 3, icon_id(6,  3), "Item 3.0", app.data.enum1);
        radio(ui, h, 4, icon_id(0, 10), "", app.data.enum1);
        radio(ui, h, 5, icon_id(1, 10), "", app.data.enum1);
        radio(ui, h, 6, icon_id(6,  3), "Item 3.3", app.data.enum1);
    }

    {
        let row = row(ui, col);
        let left = vgroup(ui, row);
        label(ui, left, no_icon(), "Items 4.0:");
        let left_body = vgroup(ui, left);
        button(ui, left_body, 7, icon_id(6, 3), "Item 4.0.0", Some(demohandler));
        button(ui, left_body, 8, icon_id(6, 3), "Item 4.0.1", Some(demohandler));
        let right = vgroup(ui, row);
        ui.set_frozen(right, app.data.option1.get()); // make initial gui state match data
        label(ui, right, no_icon(), "Items 4.1:");
        let right_body = vgroup(ui, right);
        slider(ui, right_body,  9, "Item 4.1.0", app.data.progress1);
        slider(ui, right_body, 10, "Item 4.1.1", app.data.progress2);
    }

    button(ui, col, 11, icon_id(6, 3), "Item 5", None);

    check(ui, col, 12, "Freeze section 4.1", app.data.option1, Some(freezehandler));
    check(ui, col, 13, "Item 7", app.data.option2, Some(checkhandler));
    check(ui, col, 14, "Item 8", app.data.option3, Some(checkhandler));

    // structure is built, append-handlers have run (so edge-grabbers are set);
    // now complete the layout

    ui.layout();
}

pub fn update(ui: &mut OUIContext<Widget>, (mx,my): (i32,i32), btn: bool, _t: f32) {
    // apply inputs: mouse and buttons, keys if needed

    ui.set_button(0/*left button*/, btn);
    ui.set_cursor(mx, my);

    // process input triggers to update item states
    ui.process();
}

pub fn draw(ui: &mut OUIContext<Widget>, ctx: &mut ThemedContext, w:f32, h:f32)
{
    // draw the ui
    ctx.draw_background(0.0, 0.0, w, h);

    let root = ui.root();
    draw_ui(ui, ctx, root, 0, 0);
}
