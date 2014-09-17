use nanovg::*;
use util::*;
use draw::draw::Draw;
use draw::geom::{Rect};


pub struct Label {
	pub text: String,
	pub bounds: Rect
}
impl Draw for Label {
	fn draw(&self, vg: &Ctx)
	{
		let text = self.text.as_slice();
		let x = self.bounds.x();
		let y = self.bounds.y();
		let h = self.bounds.h();

		vg.font_size(18.0);
		vg.font_face("sans");
		vg.fill_color(rgba(255,255,255,128));

		vg.text_align(LEFT|MIDDLE);
		vg.text(x,y+h*0.5,text);
	}
}

