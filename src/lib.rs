
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

#[deriving(Eq,PartialEq, Show)]
#[repr(u32)]
pub enum ItemState {
    /// DEFAULT: the item is not interacting, quiescent but usable (a button not hovered or pushed)
    COLD   = 0,
    /// HOVER: the item is unactivated, but the cursor is hovering over this item
    HOT    = 1,
    /// ACTIVE: the item is toggled or activated (depends on item kind)
    ACTIVE = 2,
    /// DISABLED: the item is unresponsive
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

pub mod util;
pub mod draw;
pub mod blendish;
pub mod oui;
pub mod widget;

//mod ui;
//mod resources;
