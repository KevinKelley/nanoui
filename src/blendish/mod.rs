
extern crate nanovg;

pub use nanovg::Color;
pub use nanovg::Winding;
pub use nanovg::CCW;
pub use nanovg::{Image, Font};
pub use self::constants::*;
pub use self::theme::ThemedContext;
pub use self::theme::*;
pub use util::{min,max,clamp,rgba_f,black,offset_color,};

pub use nanovg::Align as TextAlignment;

pub use draw::corners::{
    CornerFlags,
        CORNER_TOP_LEFT,CORNER_TOP_RIGHT,CORNER_DOWN_LEFT,CORNER_DOWN_RIGHT
};
pub use super::{
    ItemState,
    COLD,HOT,ACTIVE,FROZEN,
};

pub mod constants;
pub mod theme;

pub mod lowlevel_draw;
pub mod themed_draw;

pub mod widget;


////////////////////////////////////////////////////////////////////////////////


// Estimator Functions
// -------------------
// Use these functions to estimate sizes for widgets with your NVGcontext.

// pub fn label_width(ctx: &nanovg::Ctx, iconid: i32, label: &str, font: &Font) -> f32
// pub fn transparent(color: Color) -> Color
// pub fn offset_color(color: Color, delta: i32) -> Color
// pub fn select_corners(radiuses: &mut [f32, ..4], r: f32, flags: CornerFlags)
// pub fn inner_colors(shade_top: &mut Color, shade_down: &mut Color, theme: &WidgetTheme, state: ItemState, flipActive: bool
// pub fn text_color(theme: &WidgetTheme, state: ItemState) -> Color
// pub fn scroll_handle_rect(x: &mut f32, y: &mut f32, w: &mut f32, h: &mut f32, offset: f32, size: f32



// returns the ideal width for a label with given icon and text
pub fn label_width(ctx: &nanovg::Ctx, iconid: i32, label: &str, font: &Font) -> f32
{
    let mut w = (PAD_LEFT + PAD_RIGHT) as f32;
    if iconid >= 0 {
        w += ICON_SHEET_RES as f32;
    }
    if label.len() > 0 {
        ctx.font_face_id( font);
        ctx.font_size(LABEL_FONT_SIZE);
        w += ctx.text_advance(1.0, 1.0, label);
    }
    return w;
}


// Low Level Functions
// -------------------
// these are part of the implementation detail and can be used to theme
// new kinds of controls in a similar fashion.


// assigns radius r to the four entries of array radiuses depending on whether
// the corner is marked as sharp or not; see BNDcornerFlags for possible
// flag values.
pub fn select_corners(radiuses: &mut [f32, ..4], r: f32, flags: CornerFlags)
{
    radiuses[0] = if flags.contains(CORNER_TOP_LEFT  ) {0.0} else {r};
    radiuses[1] = if flags.contains(CORNER_TOP_RIGHT ) {0.0} else {r};
    radiuses[2] = if flags.contains(CORNER_DOWN_RIGHT) {0.0} else {r};
    radiuses[3] = if flags.contains(CORNER_DOWN_LEFT ) {0.0} else {r};
}

// computes the upper and lower gradient colors for the inner box from a widget
// theme and the widgets state. If flipActive is set and the state is
// ACTIVE, the upper and lower colors will be swapped.
pub fn inner_colors(shade_top: &mut Color, shade_down: &mut Color,
    theme: &WidgetTheme, state: ItemState, flipActive: bool)
{
    match state {
        FROZEN => {
            *shade_top = offset_color(theme.innerColor, theme.shadeTop);
            *shade_down = offset_color(theme.innerColor, theme.shadeDown);
        },
        COLD => {
            *shade_top = offset_color(theme.innerColor, theme.shadeTop);
            *shade_down = offset_color(theme.innerColor, theme.shadeDown);
        },
	    HOT => {
	        let color = offset_color(theme.innerColor, HOVER_SHADE);
	        *shade_top = offset_color(color, theme.shadeTop);
	        *shade_down = offset_color(color, theme.shadeDown);
	    },
	    ACTIVE => {
	        *shade_top = offset_color(theme.innerSelectedColor,
	            if flipActive {theme.shadeDown} else {theme.shadeTop});
	        *shade_down = offset_color(theme.innerSelectedColor,
	            if flipActive {theme.shadeTop} else {theme.shadeDown});
	    }
    }
}

// computes the text color for a widget label from a widget theme and the
// widgets state.
pub fn text_color(theme: &WidgetTheme, state: ItemState) -> Color
{
    return if state == ACTIVE {theme.textSelectedColor} else {theme.textColor};
}


// computes the bounds of the scrollbar handle from the scrollbar size
// and the handles offset and size.
// offset is in the range 0..1 and defines the position of the scroll handle
// size is in the range 0..1 and defines the size of the scroll handle
pub fn scroll_handle_rect(x: &mut f32, y: &mut f32, w: &mut f32, h: &mut f32,
    offset: f32, size: f32
) {
    let size = clamp(size, 0.0, 1.0);
    let offset = clamp(offset, 0.0, 1.0);
    if (*h) > (*w) {
        let hs = max(size*(*h), (*w)+1.0);
        *y = (*y) + ((*h)-hs)*offset;
        *h = hs;
    } else {
        let ws = max(size*(*w), (*h)-1.0);
        *x = (*x) + ((*w)-ws)*offset;
        *w = ws;
    }
}

