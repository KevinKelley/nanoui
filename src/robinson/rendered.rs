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

struct App<'a> {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    nvg: nanovg::Ctx,
    //window_size: (u32,u32),
    mouse:(i32,i32),
    button:bool,
    root_node: dom::Node,
    stylesheet: css::Stylesheet,
    //style_root: style::StyledNode<'a>,
    //layout_root: layout::LayoutBox<'a>
}
impl<'a> App<'a> {
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

        //glfw.set_swap_interval(0);


        //let initial_containing_block = layout::Dimensions::sized(800,600);
        let no_page = dom::text("no page loaded".to_string());
        let no_style = css::Stylesheet { rules: vec![] };
        //let style_root = style::style_tree(&no_page, &no_style);
        //let layout_root = layout::layout_tree(&style_root, initial_containing_block);

        App {
            glfw: glfw,
            window: window,
            events: events,
            nvg: nanovg::Ctx::create_gl3(ANTIALIAS|STENCIL_STROKES),
            mouse:(0,0),
            button:false,
            root_node: no_page,
            stylesheet: no_style,
            //style_root: style_root,
            //layout_root: layout_root
        }
    }

    fn main(html:String, css:String) {
//        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
//
//        glfw.window_hint(glfw::ContextVersion(3, 2));
//        glfw.window_hint(glfw::OpenglForwardCompat(true));
//        glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
//        glfw.window_hint(glfw::OpenglDebugContext(true));
//        let (window, events) = glfw.create_window(600,480, "Graduate - Robinson", glfw::Windowed)
//            .expect("Failed to create GLFW window.");
//
//        window.set_sticky_keys(true);
//        window.set_all_polling(true);
//        window.make_current();
//
//        // use glfw to load GL function pointers
//        glcheck!(gl::load_with(|name| glfw.get_proc_address(name)));
//
//        //glfw.set_swap_interval(0);

        let mut app = App::new();
        app.load(html, css);
        app.run();
    }
    fn run(&mut self) {
        //let (window, events) = (&self.window, &self.events);

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

            let (win_width, win_height) = self.window.get_size();
            let (fb_width, fb_height) = self.window.get_framebuffer_size();
            // Calculate pixel ration for hi-dpi devices.
            let px_ratio = fb_width as f32 / win_width as f32;

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
            self.render(win_width as uint, win_height as uint, px_ratio);

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

    fn render(&mut self, w:uint,h:uint, px_ratio:f32) {

        let containing_block = layout::Dimensions::sized(w,0);
        let style_root = style::style_tree(&self.root_node, &self.stylesheet);
        let layout_root = layout::layout_tree(&style_root, containing_block);

        self.nvg.begin_frame(w as i32, h as i32, px_ratio);

        layout_root.render(&mut self.nvg);

        dump(&layout_root);

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

fn dump<'a>(node: &layout::LayoutBox<'a>) {
    dump_rec(node, 0);
}
fn dump_rec<'a>(node: &layout::LayoutBox<'a>, level: uint) {
    let spaces = String::from_char(level, ' ');
    let d = node.dimensions;
    println!("{}{},{} - {},{} border: {}", spaces, d.x,d.y, d.width,d.height, d.border);
    for ch in node.children.iter() {
        dump_rec(ch, level + 1);
    }
}
