
pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod style;
pub mod render;

static mut last_uid: u32 = 0;

#[deriving(Show)]
pub struct Uid(u32);
impl Uid {
	fn new() -> Uid {
		let val = unsafe {
			let val = last_uid;
			last_uid = last_uid + 1;
			val
		};
		Uid(val)
	}
}