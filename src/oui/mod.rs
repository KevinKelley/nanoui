
pub use self::item::Item;
pub use self::context::Context;

pub mod geom;
pub mod item;
pub mod context;


// an OUI context holds a nested hierarchy of Items.
// there are 2 kinds of info that can be associated with an Item:
// Tag, and Widget.
// Tag is an opaque ID that isn't used internally; it's just a tag
// you can apply to a given item.
// Widget is whatever you parameterize the context with: each Item
// will then contain a Widget within it.  Handy for stateful UI.

pub type Tag = u64;


bitflags!(
    #[deriving(Show)]
    flags LayoutFlags: u32 {
        // anchor to left item or left side of parent
        static LEFT    = 1,
        // anchor to top item or top side of parent
        static TOP     = 2,
        // anchor to right item or right side of parent
        static RIGHT   = 4,
        // anchor to bottom item or bottom side of parent
        static DOWN    = 8,
        // anchor to both left and right item or parent borders
        static HFILL   = 5,
        // anchor to both top and bottom item or parent borders
        static VFILL   = 10,
        // center horizontally, with left margin as offset
        static HCENTER = 0,
        // center vertically, with top margin as offset
        static VCENTER = 0,
        // center in both directions, with left/top margin as offset
        static CENTER  = 0,
        // anchor to all four directions
        static FILL    = 15
    }
)

bitflags!(
    #[deriving(Show)]
    flags EventFlags: u32 {
        // on button 0 down
        static BUTTON0_DOWN     = 1,
        // on button 0 up
        // when this event has a handler, uiGetState() will return UI_ACTIVE as
        // long as button 0 is down.
        static BUTTON0_UP       = 2,
        // on button 0 up while item is hovered
        // when this event has a handler, uiGetState() will return UI_ACTIVE
        // when the cursor is hovering the items rectangle; this is the
        // behavior expected for buttons.
        static BUTTON0_HOT_UP   = 4,
        // item is being captured (button 0 constantly pressed);
        // when this event has a handler, uiGetState() will return UI_ACTIVE as
        // long as button 0 is down.
        static BUTTON0_CAPTURE  = 8,
        // item has received a new child
        // this can be used to allow container items to configure child items
        // as they appear.
        static APPEND           = 16
    }
)


//pub type Handler = Option<extern "C" fn(arg1: i32, arg2: EventFlags)>;
pub type Handler<Wgt> = Option<fn(ui: &mut Context<Wgt>, it: Item, evt: EventFlags)>;
