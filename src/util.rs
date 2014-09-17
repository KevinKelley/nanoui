
extern crate nanovg;

use nanovg::{Color};

pub static PI: f32 = 3.14159;

//pub fn min(a: f32, b: f32) -> f32 { if a < b { a } else { b } }
//pub fn max(a: f32, b: f32) -> f32 { if a > b { a } else { b } }
pub fn abs(a: f32) -> f32 { if a >= 0.0 { a } else { -a } }
//pub fn clamp(a: f32, mn: f32, mx: f32) -> f32 { if a < mn { mn } else { if a > mx { mx } else { a } } }
pub fn floor(x: f32) -> f32 { x.floor() }
pub fn sqrt(x: f32) -> f32 { x.sqrt() }
pub fn cos(x: f32) -> f32 { x.cos() }
pub fn sin(x: f32) -> f32 { x.sin() }
pub fn is_black(c: Color) -> bool { c.r()==0.0 && c.g()==0.0 && c.b()==0.0 && c.a()==0.0 }
pub fn rgba(r:u8,g:u8,b:u8,a:u8) -> nanovg::Color { Color::rgba(r,g,b,a) }
pub fn hsla(h:f32,s:f32,l:f32,a:u8) -> nanovg::Color { Color::hsla(h,s,l,a) }


pub fn min<T: PartialOrd>(a: T, b: T) -> T { if a<b { a } else { b } }
pub fn max<T: PartialOrd>(a: T, b: T) -> T { if a>b { a } else { b } }

pub fn clamp(v: f32, mn: f32, mx: f32) -> f32 { max(mn, min(mx, v) ) }

pub fn rgba_f(r:f32, g:f32, b:f32, a:f32) -> Color { Color::rgba_f(r, g, b, a) }
pub fn black() -> Color { Color::rgba(0,0,0,1) }

// offset a color by a given integer delta in the range -100 to 100
pub fn offset_color(color: Color, delta: i32) -> Color
{
    if delta != 0 {
	    let offset = (delta as f32) / 255.0;
        return rgba_f(
            clamp(color.r()+offset, 0.0, 1.0),
            clamp(color.g()+offset, 0.0, 1.0),
            clamp(color.b()+offset, 0.0, 1.0),
            color.a())
    }
    return color;
}


/// alpha intensity of transparent items (0xa4)
pub static TRANSPARENT_ALPHA: f32 = 0.643;

// make color transparent using the default alpha value
pub fn transparent(color: Color) -> Color
{
    return rgba_f(
        color.r(),
        color.g(),
        color.b(),
        color.a() * TRANSPARENT_ALPHA
    );
}
