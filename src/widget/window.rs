use nanovg::*;
use util::*;
use draw::draw::Draw;
use draw::geom::{Rect};

pub struct Window {
	pub title: String,
	pub bounds: Rect
}
impl Draw for Window {
	fn draw(&self, vg: &Ctx)
	{
		let title = self.title.as_slice();
		let x = self.bounds.x();
		let y = self.bounds.y();
		let w = self.bounds.w();
		let h = self.bounds.h();

		let cornerRadius = 3.0;

		vg.save();
	//	vg.clear_state();

		// Window
		vg.begin_path();
		vg.rounded_rect(x,y, w,h, cornerRadius);
		vg.fill_color(rgba(28,30,34,192));
	//	vg.fill_color(rgba(0,0,0,128));
		vg.fill();

		// Drop shadow
		let shadowPaint = vg.box_gradient(x,y+2.0, w,h, cornerRadius*2.0, 10.0, rgba(0,0,0,128), rgba(0,0,0,0));
		vg.begin_path();
		vg.rect(x-10.0,y-10.0, w+20.0,h+30.0);
		vg.rounded_rect(x,y, w,h, cornerRadius);
		vg.path_winding(HOLE);
		vg.fill_paint(shadowPaint);
		vg.fill();

		// Header
		let headerPaint = vg.linear_gradient(x,y,x,y+15.0, rgba(255,255,255,8), rgba(0,0,0,16));
		vg.begin_path();
		vg.rounded_rect(x+1.0,y+1.0, w-2.0,30.0, cornerRadius-1.0);
		vg.fill_paint(headerPaint);
		vg.fill();
		vg.begin_path();
		vg.move_to(x+0.5, y+0.5+30.0);
		vg.line_to(x+0.5+w-1.0, y+0.5+30.0);
		vg.stroke_color(rgba(0,0,0,32));
		vg.stroke();

		vg.font_size(18.0);
		vg.font_face("sans-bold");
		vg.text_align(CENTER|MIDDLE);

		vg.font_blur(2.0);
		vg.fill_color(rgba(0,0,0,128));
		vg.text(x+w/2.0,y+16.0+1.0, title);

		vg.font_blur(0.0);
		vg.fill_color(rgba(220,220,220,160));
		vg.text(x+w/2.0,y+16.0, title);

		vg.restore();
	}
}
