
//#![warn(missing_doc)]
#![feature(globs)]
#![feature(phase)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]  // temporarily
#![allow(deprecated)]

extern crate libc;
extern crate nanovg;

extern crate debug;
#[phase(plugin, link)]
extern crate inspect;

pub mod util;
pub mod geom;
pub mod draw;
pub mod blendish;
pub mod oui;
pub mod widget;
pub mod morph;
pub mod robinson;
pub mod ecs;

pub type Point = geom::Point<f32>;
pub type Rect = geom::Rect<f32>;


#[deriving(Eq,PartialEq, Show)]
#[repr(u32)]
pub enum ItemState {
    /// DEFAULT: the item is not interacting, quiescent but usable (a button not hovered or pushed)
    COLD,
    /// HOVER: the item is unactivated, but the cursor is hovering over this item
    HOT,
    /// ACTIVE: the item is toggled or activated. probably means 'has focus' (depends on item kind)
    ACTIVE,
    /// DISABLED: the item is unresponsive
    FROZEN,
}

