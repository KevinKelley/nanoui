use nanovg;
use nanovg::*;
use util::*;
use draw::draw::Draw;
use draw::icons::*;
use draw::geom::{Rect};


pub struct Button {
	pub icon: char,  // unicode char to stringify and draw from font-glyph
	pub text: String,
	pub color: nanovg::Color,
	pub bounds: Rect
}
impl Draw for Button {
	fn draw(&self, vg: &Ctx)
	{
		let text = self.text.as_slice();
		let x = self.bounds.x();
		let y = self.bounds.y();
		let w = self.bounds.w();
		let h = self.bounds.h();

		let stricon = String::from_char(1, self.icon);
		let preicon = stricon.as_slice();
		let col = self.color;

		let cornerRadius = 4.0;

		let bg = vg.linear_gradient(x,y,x,y+h, rgba(255,255,255,if is_black(col){16}else{32}), rgba(0,0,0,if is_black(col){16}else{32}));
		vg.begin_path();
		vg.rounded_rect(x+1.0,y+1.0, w-2.0,h-2.0, cornerRadius-1.0);
		if !is_black(col) {
			vg.fill_color(col);
			vg.fill();
		}
		vg.fill_paint(bg);
		vg.fill();

		vg.begin_path();
		vg.rounded_rect(x+0.5,y+0.5, w-1.0,h-1.0, cornerRadius-0.5);
		vg.stroke_color(rgba(0,0,0,48));
		vg.stroke();

		vg.font_size(20.0);
		vg.font_face("sans-bold");
		let mut bounds = [0.0f32, ..4];
		let tw = vg.text_bounds(0.0,0.0, text, &mut bounds);
		let mut iw = 0.0;
		if preicon != NO_ICON {
			vg.font_size(h*1.3);
			vg.font_face("icons");
			iw = vg.text_bounds(0.0,0.0, preicon.as_slice(), &mut bounds);
			iw += h*0.15;
		}

		if preicon != NO_ICON {
			vg.font_size(h*1.3);
			vg.font_face("icons");
			vg.fill_color(rgba(255,255,255,96));
			vg.text_align(LEFT|MIDDLE);
			vg.text(x+w*0.5-tw*0.5-iw*0.75, y+h*0.5, preicon.as_slice());
		}

		vg.font_size(20.0);
		vg.font_face("sans-bold");
		vg.text_align(LEFT|MIDDLE);
		vg.fill_color(rgba(0,0,0,160));
		vg.text(x+w*0.5-tw*0.5+iw*0.25,y+h*0.5-1.0,text);
		vg.fill_color(rgba(255,255,255,160));
		vg.text(x+w*0.5-tw*0.5+iw*0.25,y+h*0.5,text);
	}
}
