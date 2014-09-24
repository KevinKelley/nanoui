#![allow(unused_variable)]

use nanovg::{Ctx, Color};
use robinson::layout::LayoutBox;
use robinson::css;

pub trait Render {
	fn render(&self, &mut Ctx);
}

impl<'a> Render for LayoutBox<'a> {
	fn render(&self, nvg: &mut Ctx) {
        let style = self.get_style_node();
        let d = &self.dimensions;

        nvg.save();

        // bogus, erase entire area of this box
        let owned_bg = Color::rgba(224,192,224, 192);
        let white = css::Color(255,255,255,255);
        let background = style.lookup("background", "background", &white).to_color();

        let x1 = d.x             - d.margin.left - d.border.left - d.padding.left;
        let y1 = d.y             - d.margin.top  - d.border.top  - d.padding.top ;
        let x2 = d.x + d.width   + d.padding.right  + d.border.right  + d.margin.right;
        let y2 = d.y + d.height  + d.padding.bottom + d.border.bottom + d.margin.bottom;

        // paint semi-transparent bg, for debugging
        fill_rect(nvg, x1,y1, x2-x1, y2-y1, owned_bg);

        // paint client-area to background color
        fill_rect(nvg, d.x, d.y, d.width, d.height, background);

        // draw border
		{
			let black = css::Color(0,255,0, 255);
			{// top
		        let border_color = style.lookup("border-top-color", "border-color", &black).to_color();
		        let x1 = d.x - d.padding.left - d.border.left;
		        let y1 = d.y - d.padding.top - d.border.top/2.0;
		        let x2 = d.x + d.width + d.padding.right + d.border.right;
		        let y2 = y1;
		        draw_line(nvg, x1,y1, x2,y2, d.border.top, border_color);
			}
			{// bottom
		        let border_color = style.lookup("border-bottom-color", "border-color", &black).to_color();
		        let x1 = d.x - d.padding.left - d.border.left;
		        let y1 = d.y + d.height + d.padding.bottom - d.border.bottom/2.0;
		        let x2 = d.x + d.width + d.padding.right + d.border.right;
		        let y2 = y1;
		        draw_line(nvg, x1,y1, x2,y2, d.border.bottom, border_color);
			}
			{// left
		        let border_color = style.lookup("border-left-color", "border-color", &black).to_color();
		        let x1 = d.x - d.padding.left - d.border.left/2.0;
		        let y1 = d.y - d.padding.top;
		        let x2 = x1;
		        let y2 = d.y + d.height + d.padding.bottom;
		        draw_line(nvg, x1,y1, x2,y2, d.border.left, border_color);
			}
			{// right
		        let border_color = style.lookup("border-right-color", "border-color", &black).to_color();
		        let x1 = d.x + d.width + d.padding.right + d.border.right/2.0;
		        let y1 = d.y - d.padding.top;
		        let x2 = x1;
		        let y2 = d.y + d.height + d.padding.bottom;
		        draw_line(nvg, x1,y1, x2,y2, d.border.right, border_color);
			}

		}
        // clip to content-box
        //nvg.scissor(d.x, d.y, d.width, d.height);

        for child in self.children.iter() {
        	child.render(nvg);
        }

        nvg.restore();
	}
}

fn fill_rect(nvg: &mut Ctx, x:f32,y:f32, w:f32,h:f32, bg:Color) {
    nvg.begin_path();
    nvg.rect(x, y, w, h);
    nvg.fill_color(bg);
    nvg.fill();
}
fn draw_line(nvg: &mut Ctx, x1:f32,y1:f32, x2:f32,y2:f32, line_width:f32, color:Color) {
    nvg.begin_path();
    nvg.move_to(x1,y1);
    nvg.line_to(x2,y2);
    nvg.stroke_width(line_width);
    nvg.stroke_color(color);
    nvg.stroke();
}
