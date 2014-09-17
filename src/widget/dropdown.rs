use nanovg::*;
use util::*;
use draw::draw::Draw;
use draw::icons::*;
use draw::geom::{Rect};


pub struct DropDown {
	pub text: String,
	pub bounds: Rect
}
impl Draw for DropDown {
	fn draw(&self, vg: &Ctx)
	{
		let text = self.text.as_slice();
		let x = self.bounds.x();
		let y = self.bounds.y();
		let w = self.bounds.w();
		let h = self.bounds.h();

		let cornerRadius = 4.0;

		let bg = vg.linear_gradient(x,y,x,y+h, rgba(255,255,255,16), rgba(0,0,0,16));
		vg.begin_path();
		vg.rounded_rect(x+1.0,y+1.0, w-2.0,h-2.0, cornerRadius-1.0);
		vg.fill_paint(bg);
		vg.fill();

		vg.begin_path();
		vg.rounded_rect(x+0.5,y+0.5, w-1.0,h-1.0, cornerRadius-0.5);
		vg.stroke_color(rgba(0,0,0,48));
		vg.stroke();

		vg.font_size(20.0);
		vg.font_face("sans");
		vg.fill_color(rgba(255,255,255,160));
		vg.text_align(LEFT|MIDDLE);
		vg.text(x+h*0.3,y+h*0.5,text);

		vg.font_size(h*1.3);
		vg.font_face("icons");
		vg.fill_color(rgba(255,255,255,64));
		vg.text_align(CENTER|MIDDLE);
		vg.text(x+w-h*0.5, y+h*0.5, ICON_CHEVRON_RIGHT.as_slice());
	}
}
