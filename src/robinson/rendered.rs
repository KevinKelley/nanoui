#![feature(phase)]
#![feature(macro_rules)]

extern crate native;
extern crate sync;
extern crate glfw;
extern crate gl;

extern crate nanovg;
extern crate nanoui;
extern crate getopts;

extern crate debug;
#[phase(plugin, link)]
extern crate inspect;


use getopts::{optopt,getopts};
use std::io::fs::File;
use std::os::args;

use glfw::Context as GLFWContext;
use sync::comm::Receiver;
use nanovg::{Ctx, ANTIALIAS,STENCIL_STROKES};
use nanoui::robinson::css;
use nanoui::robinson::dom;
use nanoui::robinson::html;
use nanoui::robinson::layout;
use nanoui::robinson::style;
use nanoui::robinson::render::Render;

///////////////////////////////////////////////////////////////////////
/// evaluate the expression, then check for GL error.
macro_rules! glcheck(
    ($e: expr) => (
        {
            $e;
            assert_eq!(gl::GetError(), 0);
        }
    )
)

/// main entry point.
///
/// read an html and a css file (default or as specified in cmd-line parameters),
/// then call App::main.
fn main() {
    // Parse command-line options:
    let opts = [
        optopt("h", "html", "HTML document", "FILENAME"),
        optopt("c", "css", "CSS stylesheet", "FILENAME"),
    ];
    let matches = match getopts(args().tail(), opts) {
        Ok(m) => m,
        Err(f) => fail!(f.to_string())
    };

    // Read input files:
    let read_source = |arg_filename: Option<String>, default_filename: &str| {
        let path = match arg_filename {
            Some(ref filename) => filename.as_slice(),
            None => default_filename,
        };
        File::open(&Path::new(path)).read_to_string().unwrap()
    };
    let html = read_source(matches.opt_str("h"), "examples/test.html");
    let css  = read_source(matches.opt_str("c"), "examples/test.css");

    App::main(html, css);
}

/// App holds runtime state, like window and graphics-lib contexts.
struct App<'a> {

    /// use GLFW for window-creation; this is the initialized handle to GLFW library.
    glfw: glfw::Glfw,

    /// application's main window
    window: glfw::Window,
    /// a channel for receiving events directed to 'window'.
    events: Receiver<(f64, glfw::WindowEvent)>,

    /// NanoVG drawing-context; uses GL to draw vector-graphics primitives to window.
    nvg: nanovg::Ctx,

    /// track mouse-position; redundant with glfw queries
    mouse:(i32,i32),
    /// track primary-button-press state; redundant
    button:bool,

    /// here's the DOM root...
    root_node: dom::Node,
    /// ...and the stylesheet to use when rendering it.
    stylesheet: css::Stylesheet,
}

impl<'a> App<'a> {

    /// create an App, give it the html and css, and run it.
    fn main(html:String, css:String) {
        let mut app = App::new();
        app.load(html, css);
        app.run();
    }

    /// initialize libraries, and create a main window.
    fn new<'a>() -> App<'a> {
        // initialize glfw, and create a main-window
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::ContextVersion(3, 2));
        glfw.window_hint(glfw::OpenglForwardCompat(true));
        glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
        glfw.window_hint(glfw::OpenglDebugContext(true));
        let (window, events) = glfw.create_window(600,480, "Graduate - Robinson", glfw::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_sticky_keys(true);
        window.set_all_polling(true);
        window.make_current();

        // use glfw to load GL function pointers
        glcheck!(gl::load_with(|name| glfw.get_proc_address(name)));

        App {
            glfw: glfw,
            window: window,
            events: events,
            nvg: nanovg::Ctx::create_gl3(ANTIALIAS|STENCIL_STROKES),
            mouse:(0,0),
            button:false,
            root_node: dom::text("no page loaded".to_string()),
            stylesheet: css::Stylesheet { rules: vec![] },
        }
    }

    /// process window events and render the display.
    fn run(&mut self) {

        while !self.window.should_close()
        {
            // process outstanding window events
            self.glfw.wait_events(); // or poll_events for continuous updating

            // 'borrow' dance!  can't borrow self as mut (to handle events)
            // while 'self.window' is borrowed as immut (to receive them).
            // so, first grab events from window into a vec; then process them from the vec.iter
            let mut evts: Vec<glfw::WindowEvent> = vec![];
            for (_timestamp, event) in glfw::flush_messages(&self.events) {
                evts.push(event);
            }
            for event in evts.iter() {
                handle_window_event(self, *event);
            }

            //let (win_width, win_height) = self.window.get_size();
            let (fb_width, fb_height) = self.window.get_framebuffer_size();
            // Calculate pixel ration for hi-dpi devices.
            //let px_ratio = fb_width as f32 / win_width as f32;

            // clear framebuffer
            glcheck!(gl::Viewport(0, 0, fb_width, fb_height));
            glcheck!(gl::ClearColor(0.0, 0.0, 0.0, 0.0));
            glcheck!(gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT|gl::STENCIL_BUFFER_BIT));

            // configuration for GL context
            glcheck!(gl::Enable(gl::BLEND));
            glcheck!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
            glcheck!(gl::Enable(gl::CULL_FACE));
            glcheck!(gl::Disable(gl::DEPTH_TEST));

            // Update ui, and render to framebuffer
            self.update();
            self.render();

            glcheck!(gl::Enable(gl::DEPTH_TEST));

            // swap in the freshened buffer
            self.window.swap_buffers();
        }
    }

    fn load(&'a mut self, html:String, css:String) {
        // Since we don't have an actual window, hard-code the "viewport" size.
        //let initial_containing_block = layout::Dimensions::sized(800,600);

        // Parsing and rendering:
        self.root_node = html::parse(html);
        self.stylesheet = css::parse(css);
        //self.style_root = style::style_tree(&self.root_node, &self.stylesheet);
        //self.layout_root = layout::layout_tree(&self.style_root, initial_containing_block);

        // Debug output:
        //println!("{}", layout_root.dimensions);
        //inspect!(root_node); println!("");
        //inspect!(stylesheet); println!("");
        //inspect!(style_root); println!("");
        //inspect!(layout_root); println!("");
    }

    fn update(&self) {/*pass in mouse/key/window state too*/}

    fn render(&mut self) {

            let (win_width, win_height) = self.window.get_size();
            let (fb_width, _fb_height) = self.window.get_framebuffer_size();
            // Calculate pixel ration for hi-dpi devices.
            let px_ratio = fb_width as f32 / win_width as f32;

        // TODO - have to pass '0' height to root layout,
        // or it'll position stuff below existing window
        let containing_block = layout::Dimensions::sized(win_width as uint, 0);

        let style_root = style::style_tree(&self.root_node, &self.stylesheet);
        let layout_root = layout::layout_tree(&style_root, containing_block);

        // TODO figure out where/how to manage resources... layout and render need access
        let filename = format!("{}/DejaVuSans.ttf", "./res");
        let font = self.nvg.create_font("sans", filename.as_slice())
            .expect(format!("Could not load font from '{}'", filename).as_slice());

        self.nvg.begin_frame(win_width as i32, win_height as i32, px_ratio);

        layout_root.render(&mut self.nvg, &font);

        println!(""); dump_bounds(&layout_root, 0);
        //println!(""); inspect!(layout_root);

        self.nvg.end_frame();
    }
}
fn handle_window_event(
    //window: &glfw::Window,
    app: &mut App,
    event: glfw::WindowEvent
) {
    let window = &app.window;
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => window.set_should_close(true),
        glfw::MouseButtonEvent(_, glfw::Press, _) => app.button = true,
        glfw::MouseButtonEvent(_, glfw::Release, _) => app.button = false,
        glfw::CursorPosEvent(xpos, ypos) => app.mouse = (xpos as i32, ypos as i32),
        glfw::RefreshEvent => {}
        _ => {}
    }
}

fn dump_bounds<'a>(node: &layout::LayoutBox<'a>, level: uint) {
    let spaces = String::from_char(level*2, ' ');
    let d = node.dimensions;
    println!("{}{},{} - {},{} ", spaces, d.x,d.y, d.width,d.height);
    for ch in node.children.iter() {
        dump_bounds(ch, level+1);
    }
}
