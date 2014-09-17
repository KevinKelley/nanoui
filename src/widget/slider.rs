use nanovg::*;
use util::*;
use draw::draw::Draw;
use draw::geom::{Rect};


pub struct Slider {
	pub pos: f32,		// hopefully this means 0..1 == relative position of slider
	pub bounds: Rect
}
impl Draw for Slider {
	fn draw(&self, vg: &Ctx)
	{
		let x = self.bounds.x();
		let y = self.bounds.y();
		let w = self.bounds.w();
		let h = self.bounds.h();
		let pos = self.pos;

		let cy: f32 = y+floor(h*0.5);
		let kr: f32 = floor(h*0.25);

		vg.save();
	//	vg.clear_state();

		// Slot
		let bg = vg.box_gradient(x,cy-2.0+1.0, w,4.0, 2.0,2.0, rgba(0,0,0,32), rgba(0,0,0,128));
		vg.begin_path();
		vg.rounded_rect(x,cy-2.0, w,4.0, 2.0);
		vg.fill_paint(bg);
		vg.fill();

		// Knob Shadow
		let shadow = vg.radial_gradient(x+floor(pos*w),cy+1.0, kr-3.0,kr+3.0, rgba(0,0,0,64), rgba(0,0,0,0));
		vg.begin_path();
		vg.rect(x+floor(pos*w)-kr-5.0,cy-kr-5.0,kr*2.0+5.0+5.0,kr*2.0+5.0+5.0+3.0);
		vg.circle(x+floor(pos*w),cy, kr);
		vg.path_winding(HOLE);
		vg.fill_paint(shadow);
		vg.fill();

		// Knob
		let knob = vg.linear_gradient(x,cy-kr,x,cy+kr, rgba(255,255,255,16), rgba(0,0,0,16));
		vg.begin_path();
		vg.circle(x+floor(pos*w),cy, kr-1.0);
		vg.fill_color(rgba(40,43,48,255));
		vg.fill();
		vg.fill_paint(knob);
		vg.fill();

		vg.begin_path();
		vg.circle(x+floor(pos*w),cy, kr-0.5);
		vg.stroke_color(rgba(0,0,0,92));
		vg.stroke();

		vg.restore();
	}
}

