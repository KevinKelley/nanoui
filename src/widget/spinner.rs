use nanovg::*;
use util::*;
use draw::draw::Draw;
use draw::geom::Point;


pub struct Spinner {
	pub text: String,
	pub center: Point,
	pub rho: f32,
	pub theta: f32
}
impl Draw for Spinner {
	fn draw(&self, vg: &Ctx)
	{
		//let text = self.text.as_slice();
		let cx = self.center.x;
		let cy = self.center.y;
		let r = self.rho;
		let t = self.theta;

		let a0 = 0.0 + t*6.0;
		let a1 = PI + t*6.0;
		let r0 = r;
		let r1 = r * 0.75;

		vg.save();

		vg.begin_path();
		vg.arc(cx,cy, r0, a0, a1, CW);
		vg.arc(cx,cy, r1, a1, a0, CCW);
		vg.close_path();
		let ax = cx + cos(a0) * (r0+r1)*0.5;
		let ay = cy + sin(a0) * (r0+r1)*0.5;
		let bx = cx + cos(a1) * (r0+r1)*0.5;
		let by = cy + sin(a1) * (r0+r1)*0.5;
		let paint = vg.linear_gradient(ax,ay, bx,by, rgba(0,0,0,0), rgba(0,0,0,128));
		vg.fill_paint(paint);
		vg.fill();

		vg.restore();
	}
}
