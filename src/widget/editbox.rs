use nanovg::*;
use util::*;
use draw::draw::Draw;
use draw::geom::{Rect};


pub struct EditBox {
	pub text: String,
	pub bounds: Rect
}
impl Draw for EditBox {
	fn draw(&self, vg: &Ctx)
	{
		let text = self.text.as_slice();
		let x = self.bounds.x();
		let y = self.bounds.y();
		let w = self.bounds.w();
		let h = self.bounds.h();

		EditBox::draw_editbox_base(vg, x,y, w,h);

		vg.font_size(20.0);
		vg.font_face("sans");
		vg.fill_color(rgba(255,255,255,64));
		vg.text_align(LEFT|MIDDLE);
		vg.text(x+h*0.3,y+h*0.5,text);
	}
}
impl EditBox {
	fn draw_editbox_base(vg: &Ctx, x: f32, y: f32, w: f32, h: f32)
	{
		// Edit
		let bg = vg.box_gradient(x+1.0,y+1.0+1.5, w-2.0,h-2.0, 3.0,4.0, rgba(255,255,255,32), rgba(32,32,32,32));
		vg.begin_path();
		vg.rounded_rect(x+1.0,y+1.0, w-2.0,h-2.0, 4.0-1.0);
		vg.fill_paint(bg);
		vg.fill();

		vg.begin_path();
		vg.rounded_rect(x+0.5,y+0.5, w-1.0,h-1.0, 4.0-0.5);
		vg.stroke_color(rgba(0,0,0,48));
		vg.stroke();
	}

	fn draw_editbox_num(vg: &Ctx, text: &str, units: &str, x: f32, y: f32, w: f32, h: f32)
	{
		EditBox::draw_editbox_base(vg, x,y, w,h);

		let mut bounds = [0.0f32, ..4];
		let uw = vg.text_bounds(0.0,0.0, units, &mut bounds);

		vg.font_size(18.0);
		vg.font_face("sans");
		vg.fill_color(rgba(255,255,255,64));
		vg.text_align(RIGHT|MIDDLE);
		vg.text(x+w-h*0.3,y+h*0.5,units);

		vg.font_size(20.0);
		vg.font_face("sans");
		vg.fill_color(rgba(255,255,255,128));
		vg.text_align(RIGHT|MIDDLE);
		vg.text(x+w-uw-h*0.5,y+h*0.5,text);
	}
}
