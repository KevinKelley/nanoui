
//#![warn(missing_doc)]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(unsafe_destructor)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variable)]
#![allow(non_camel_case_types)]  // temporarily

extern crate libc;
extern crate nanovg;


//#[deriving(Clone, Eq, PartialEq, Show)]
//#[repr(u32)]
//pub enum WidgetState {
//    /// not interacting
//    DEFAULT  = 0,
//    /// the mouse is hovering over the control
//    HOVER    = 1,
//    /// the widget is activated (pressed) or in an active state (toggled)
//    ACTIVE   = 2,
//}
#[deriving(Eq,PartialEq, Show)]
#[repr(u32)]
pub enum ItemState {
    /// the item is inactive
    COLD   = 0,
    /// the item is inactive, but the cursor is hovering over this item
    HOT    = 1,
    /// the item is toggled or activated (depends on item kind)
    ACTIVE = 2,
    /// the item is unresponsive
    FROZEN = 3,
}

/// flags indicating which corners are sharp (for grouping widgets)
bitflags!(
    flags CornerFlags: u32 {
        // all corners are round
        static CORNER_NONE         = 0,
        // sharp top left corner
        static CORNER_TOP_LEFT     = 1,
        // sharp top right corner
        static CORNER_TOP_RIGHT    = 2,
        // sharp bottom right corner
        static CORNER_DOWN_RIGHT   = 4,
        // sharp bottom left corner
        static CORNER_DOWN_LEFT    = 8,
        // all corners are sharp;
        // you can invert a set of flags using ^= BND_CORNER_ALL
        static CORNER_ALL          = 0xF,
        // top border is sharp
        static CORNER_TOP          = 3,
        // bottom border is sharp
        static CORNER_DOWN         = 0xC,
        // left border is sharp
        static CORNER_LEFT         = 9,
        // right border is sharp
        static CORNER_RIGHT        = 6
    }
)

pub mod blendish;
pub mod oui;
//mod ui;
//mod resources;

