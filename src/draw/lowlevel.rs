
use nanovg::{
    Ctx,
    Color,
    BUTT,MITER,
};
use util::{min,max,rgba_f,offset_color,transparent,};


/// shade intensity of beveled panels (expressed in percentage, -100..100)
pub static BEVEL_SHADE: i32 = 30;			// percent, -100..100
/// shade intensity of beveled insets
pub static INSET_BEVEL_SHADE: i32 = 30;		// percent, -100..100


//////////////////////////////////////////////////////
// NanoVG context extenders (self must be nanovg::Ctx)

pub trait LowLevel
{
    // misc utility

    //fn transparent(color: Color) -> Color;
    //fn offset_color(color: Color, delta: i32) -> Color;
    //fn select_corners(radiuses: [f32, ..4], r: f32, flags: CornerFlags);
    //fn inner_colors(shade_top: &Color, shade_down: &Color, theme: &WidgetTheme, state: WidgetState, flipActive: bool);
    //fn text_color(theme: &WidgetTheme, state: WidgetState) -> Color;
    //fn scroll_handle_rect(x: *const f32, y: *const f32, w: *const f32, h: *const f32, offset: f32, size: f32);

    // context related

    fn draw_rounded_box  (&mut self, x:f32, y:f32, w:f32,h:f32, cr0:f32,cr1:f32,cr2:f32,cr3:f32);
    fn draw_background   (&mut self, x:f32, y:f32, w:f32,h:f32, bg: Color);
    fn draw_bevel        (&mut self, x:f32, y:f32, w:f32,h:f32, bg: Color);
    fn draw_bevel_inset  (&mut self, x:f32, y:f32, w:f32,h:f32, cr2:f32,cr3:f32, bg: Color);
    fn draw_drop_shadow  (&mut self, x:f32, y:f32, w:f32,h:f32, r: f32, feather: f32, alpha: f32);
    fn draw_inner_box    (&mut self, x:f32, y:f32, w:f32,h:f32, cr0:f32,cr1:f32,cr2:f32,cr3:f32, shade_top: Color, shade_down: Color);
    fn draw_outline_box  (&mut self, x:f32, y:f32, w:f32,h:f32, cr0:f32,cr1:f32,cr2:f32,cr3:f32, color: Color);
    fn draw_check        (&mut self,ox:f32,oy:f32, color: Color);
    fn draw_arrow        (&mut self, x:f32, y:f32, s: f32, color: Color);
    fn draw_up_down_arrow(&mut self, x:f32, y:f32, s: f32, color: Color);
}
impl LowLevel for Ctx {

    /// Add a rounded box path at position (x, y) with size (w, h) and a separate
    /// radius for each corner listed in clockwise order, so that cr0 = top left,
    /// cr1 = top right, cr2 = bottom right, cr3 = bottom left;
    /// this is a low level drawing function: the path must be stroked or filled
    /// to become visible.
    fn draw_rounded_box(&mut self, x:f32,y:f32, w:f32,h:f32,
        cr0: f32, cr1: f32, cr2: f32, cr3: f32
    ) {
        let w = max(0.0, w);
        let h = max(0.0, h);
        let d = min(w, h);

        self.move_to(x, y+h*0.5);
        self.arc_to(x, y, x+w, y, min(cr0, d/2.0));
        self.arc_to(x+w, y, x+w, y+h, min(cr1, d/2.0));
        self.arc_to(x+w, y+h, x, y+h, min(cr2, d/2.0));
        self.arc_to(x, y+h, x, y, min(cr3, d/2.0));
        self.close_path();
    }

    /// Draw a flat panel without any decorations at position (x, y) with size (w, h)
    /// and fills it with bg color
    fn draw_background(&mut self, x:f32,y:f32, w:f32,h:f32, bg: Color)
    {
        self.begin_path();
        self.rect(x, y, w, h);
        self.fill_color(bg);
        self.fill();
    }

    /// Draw a beveled border at position (x, y) with size (w, h) shaded with
    /// lighter and darker versions of backgroundColor
    fn draw_bevel(&mut self, x:f32,y:f32, w:f32,h:f32, bg: Color)
    {
        self.stroke_width(1.0);

        let x = x + 0.5;
        let y = y + 0.5;
        let w = w - 1.0;
        let h = h - 1.0;

        self.begin_path();
        self.move_to(x, y+h);
        self.line_to(x+w, y+h);
        self.line_to(x+w, y);
        self.stroke_color(transparent(
            offset_color(bg, -BEVEL_SHADE)));
        self.stroke();

        self.begin_path();
        self.move_to(x, y+h);
        self.line_to(x, y);
        self.line_to(x+w, y);
        self.stroke_color(transparent(
            offset_color(bg, BEVEL_SHADE)));
        self.stroke();
    }

    /// Draw a lower inset for a rounded box at position (x, y) with size (w, h)
    /// that gives the impression the surface has been pushed in.
    /// cr2 and cr3 contain the radiuses of the bottom right and bottom left
    /// corners of the rounded box.
    fn draw_bevel_inset(&mut self, x:f32,y:f32, w:f32,h:f32,
        cr2: f32, cr3: f32,
        bg: Color
    ) {
        let y = y - 0.5;
        let d = min(w, h);
        let cr2 = min(cr2, d/2.0);
        let cr3 = min(cr3, d/2.0);

        self.begin_path();
        self.move_to(x+w, y+h-cr2);
        self.arc_to(x+w, y+h, x, y+h, cr2);
        self.arc_to(x, y+h, x, y, cr3);

        let bevelColor = offset_color(bg,
            INSET_BEVEL_SHADE);

        self.stroke_width(1.0);
        self.stroke_paint(
            self.linear_gradient(
                x, y+h-max(cr2, cr3)-1.0,
                x, y+h-1.0,
                rgba_f(bevelColor.r(), bevelColor.g(), bevelColor.b(), 0.0),
                bevelColor));
        self.stroke();
    }

    /// Draw a drop shadow around the rounded box at (x, y) with size (w, h) and
    /// radius r, with feather as its maximum range in pixels.
    /// No shadow will be painted inside the rounded box.
    fn draw_drop_shadow(&mut self, x:f32,y:f32, w:f32,h:f32,
        r: f32, feather: f32, alpha: f32
    ) {
        self.begin_path();

        let mut y = y;
        let mut h = h;
        y += feather;
        h -= feather;

        self.move_to(x-feather, y-feather);
        self.line_to(x, y-feather);
        self.line_to(x, y+h-feather);
        self.arc_to(x, y+h, x+r, y+h, r);
        self.arc_to(x+w, y+h, x+w, y+h-r, r);
        self.line_to(x+w, y-feather);
        self.line_to(x+w+feather, y-feather);
        self.line_to(x+w+feather, y+h+feather);
        self.line_to(x-feather, y+h+feather);
        self.close_path();

        self.fill_paint(self.box_gradient(
            x - feather*0.5, y - feather*0.5,
            w + feather, h+feather,
            r+feather*0.5,
            feather,
            rgba_f(0.0, 0.0, 0.0, alpha*alpha),
            rgba_f(0.0, 0.0, 0.0, 0.0)));
        self.fill();
    }

    /// Draw the inner part of a widget box, with a gradient from shade_top to
    /// shade_down. If h>w, the gradient will be horizontal instead of
    /// vertical.
    fn draw_inner_box(&mut self, x:f32,y:f32, w:f32,h:f32,
        cr0: f32, cr1: f32, cr2: f32, cr3: f32,
        shade_top: Color, shade_down: Color
    ) {
        self.begin_path();
        self.draw_rounded_box(x+1.0, y+1.0, w-2.0, h-3.0,
            max(0.0, cr0-1.0), max(0.0, cr1-1.0), max(0.0, cr2-1.0), max(0.0, cr3-1.0));
        self.fill_paint(
            if (h-2.0)>w  {self.linear_gradient(x, y, x+w, y, shade_top, shade_down)}
            else        {self.linear_gradient(x, y, x, y+h, shade_top, shade_down)});
        self.fill();
    }

    /// Draw the outline part of a widget box with the given color
    fn draw_outline_box(&mut self, x:f32,y:f32, w:f32,h:f32,
        cr0: f32, cr1: f32, cr2: f32, cr3: f32, color: Color
    ) {
        self.begin_path();
        self.draw_rounded_box(x+0.5, y+0.5, w-1.0, h-2.0, cr0, cr1, cr2, cr3);
        self.stroke_color(color);
        self.stroke_width(1.0);
        self.stroke();
    }

    /// Draw a checkmark for an option box with the given upper left coordinates
    /// (ox, oy) with the specified color.
    fn draw_check(&mut self, ox: f32, oy: f32, color: Color)
    {
        self.begin_path();
        self.stroke_width(2.0);
        self.stroke_color(color);
        self.line_cap(BUTT);
        self.line_join(MITER);
        self.move_to(ox+4.0, oy+5.0);
        self.line_to(ox+7.0, oy+8.0);
        self.line_to(ox+14.0, oy+1.0);
        self.stroke();
    }


    /// Draw a horizontal arrow for a number field with its center at (x, y) and
    /// size s; if s is negative, the arrow points to the left.
    fn draw_arrow(&mut self, x: f32, y: f32, s: f32, color: Color)
    {
        self.begin_path();
        self.move_to(x, y);
        self.line_to(x-s, y+s);
        self.line_to(x-s, y-s);
        self.close_path();
        self.fill_color(color);
        self.fill();
    }

    /// Draw an up/down arrow for a choice box with its center at (x, y) and size s
    fn draw_up_down_arrow(&mut self, x: f32, y: f32, s: f32, color: Color)
    {
        self.begin_path();
        let w = 1.1*s;
        self.move_to(x, y-1.0);
        self.line_to(x+0.5*w, y-s-1.0);
        self.line_to(x+w, y-1.0);
        self.close_path();
        self.move_to(x, y-1.0);
        self.line_to(x+0.5*w, y+s+1.0);
        self.line_to(x+w, y-1.0);
        self.close_path();
        self.fill_color(color);
        self.fill();
    }
}
