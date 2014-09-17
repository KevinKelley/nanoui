
use nanovg::Ctx;


pub trait Draw {
	fn draw(&self, vg: &Ctx);
}




