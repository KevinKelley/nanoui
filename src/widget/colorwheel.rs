use nanovg::*;
use util::*;
use draw::draw::Draw;
use draw::geom::{Rect};


pub struct ColorWheel {
	pub bounds: Rect,
	pub theta: f32
}
impl Draw for ColorWheel {
	fn draw(&self, vg: &Ctx)
	{
		let x = self.bounds.x();
		let y = self.bounds.y();
		let w = self.bounds.w();
		let h = self.bounds.h();
		let t = self.theta;

		//f32 r0, r1, ax,ay, bx,by, cx,cy, aeps, r;
		let hue = sin(t * 0.12);

		vg.save();

	  /*	vg.begin_path();
		vg.rect(x,y,w,h);
		vg.fill_color(rgba(255,0,0,128));
		vg.fill();*/

		let cx = x + w*0.5;
		let cy = y + h*0.5;
		let r1 = min(w,h) * 0.5 - 5.0;
		let r0 = r1 - 20.0;
		let aeps = 0.5 / r1;	// half a pixel arc length in radians (2pi cancels out).

		for i in range(0, 6u) {
			let a0 = (i as f32) / 6.0 * PI * 2.0 - aeps;
			let a1 = ((i as f32)+1.0) / 6.0 * PI * 2.0 + aeps;
			vg.begin_path();
			vg.arc(cx,cy, r0, a0, a1, CW);
			vg.arc(cx,cy, r1, a1, a0, CCW);
			vg.close_path();
			let ax = cx + cos(a0) * (r0+r1)*0.5;
			let ay = cy + sin(a0) * (r0+r1)*0.5;
			let bx = cx + cos(a1) * (r0+r1)*0.5;
			let by = cy + sin(a1) * (r0+r1)*0.5;
			let paint = vg.linear_gradient(ax,ay, bx,by, hsla(a0/(PI*2.0),1.0,0.55,255), hsla(a1/(PI*2.0),1.0,0.55,255));
			vg.fill_paint(paint);
			vg.fill();
		}

		vg.begin_path();
		vg.circle(cx,cy, r0-0.5);
		vg.circle(cx,cy, r1+0.5);
		vg.stroke_color(rgba(0,0,0,64));
		vg.stroke_width(1.0);
		vg.stroke();

		// Selector
		vg.save();
		vg.translate(cx,cy);
		vg.rotate(hue*PI*2.0);

		// Marker on
		vg.stroke_width(2.0);
		vg.begin_path();
		vg.rect(r0-1.0,-3.0,r1-r0+2.0,6.0);
		vg.stroke_color(rgba(255,255,255,192));
		vg.stroke();

		let mut paint = vg.box_gradient(r0-3.0,-5.0,r1-r0+6.0,10.0, 2.0,4.0, rgba(0,0,0,128), rgba(0,0,0,0));
		vg.begin_path();
		vg.rect(r0-2.0-10.0,-4.0-10.0,r1-r0+4.0+20.0,8.0+20.0);
		vg.rect(r0-2.0,-4.0,r1-r0+4.0,8.0);
		vg.path_winding(HOLE);
		vg.fill_paint(paint);
		vg.fill();

		// Center triangle
		let r = r0 - 6.0;
		let mut ax = cos(120.0/180.0*PI) * r;
		let mut ay = sin(120.0/180.0*PI) * r;
		let bx = cos(-120.0/180.0*PI) * r;
		let by = sin(-120.0/180.0*PI) * r;
		vg.begin_path();
		vg.move_to(r,0.0);
		vg.line_to(ax,ay);
		vg.line_to(bx,by);
		vg.close_path();
		paint = vg.linear_gradient(r,0.0, ax,ay, hsla(hue,1.0,0.5,255), rgba(255,255,255,255));
		vg.fill_paint(paint);
		vg.fill();
		paint = vg.linear_gradient((r+ax)*0.5,(0.0+ay)*0.5, bx,by, rgba(0,0,0,0), rgba(0,0,0,255));
		vg.fill_paint(paint);
		vg.fill();
		vg.stroke_color(rgba(0,0,0,64));
		vg.stroke();

		// Select circle on triangle
		ax = cos(120.0/180.0*PI) * r*0.3;
		ay = sin(120.0/180.0*PI) * r*0.4;
		vg.stroke_width(2.0);
		vg.begin_path();
		vg.circle(ax,ay,5.0);
		vg.stroke_color(rgba(255,255,255,192));
		vg.stroke();

		paint = vg.radial_gradient(ax,ay, 7.0,9.0, rgba(0,0,0,64), rgba(0,0,0,0));
		vg.begin_path();
		vg.rect(ax-20.0,ay-20.0,40.0,40.0);
		vg.circle(ax,ay,7.0);
		vg.path_winding(HOLE);
		vg.fill_paint(paint);
		vg.fill();

		vg.restore();

		vg.restore();
	}
}
