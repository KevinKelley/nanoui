use nanovg::*;
use util::*;
use draw::draw::Draw;
use draw::geom::{Rect};
use draw::icons::ICON_CHECK;


pub struct CheckBox {
	pub text: String,
	pub bounds: Rect
}
impl Draw for CheckBox {
	fn draw(&self, vg: &Ctx)
	{
		let text = self.text.as_slice();
		let x = self.bounds.x();
		let y = self.bounds.y();
		let h = self.bounds.h();

		vg.font_size(18.0);
		vg.font_face("sans");
		vg.fill_color(rgba(255,255,255,160));

		vg.text_align(LEFT|MIDDLE);
		vg.text(x+28.0, y+h*0.5, text);

		let bg = vg.box_gradient(x+1.0,y+floor(h*0.5)-9.0+1.0, 18.0,18.0, 3.0,3.0, rgba(0,0,0,32), rgba(0,0,0,92));
		vg.begin_path();
		vg.rounded_rect(x+1.0,y+floor(h*0.5)-9.0, 18.0,18.0, 3.0);
		vg.fill_paint(bg);
		vg.fill();

		vg.font_size(40.0);
		vg.font_face("icons");
		vg.fill_color(rgba(255,255,255,128));
		vg.text_align(CENTER|MIDDLE);
		vg.text(x+9.0+2.0, y+h*0.5, ICON_CHECK);
	}
}
