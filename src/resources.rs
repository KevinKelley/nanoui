
use nanovg::Ctx;
use nanovg::{Image,Font};

///// use unicode characters for icons
//static NO_ICON: 		   char = '\0';
//static ICON_SEARCH:        char = '\U0001F50D';
//static ICON_CIRCLED_CROSS: char = '\u2716';
//static ICON_CHEVRON_RIGHT: char = '\uE75E';
//static ICON_CHECK:         char = '\u2713';
//static ICON_LOGIN:         char = '\uE740';
//static ICON_TRASH:         char = '\uE729';


pub struct Resources {
	pub fontNormal: Font,
	pub iconsheet: Image
}

/// load and hold resources used in demo
impl Resources {
	pub fn load(vg: &Ctx, res_path: &str) -> Resources
	{
		let filename = format!("{}/blender_icons16.png", res_path);
		let icons = vg.create_image(filename.as_slice())
			.expect(format!("Couldn't load icons image from '{}'", filename).as_slice());

		let filename = format!("{}/DejaVuSans.ttf", res_path);
		let font = vg.create_font("sans", filename.as_slice())
			.expect(format!("Could not add font 'sans' from '{}'", filename).as_slice());

		Resources {
			fontNormal: font,
			iconsheet:  icons
		}
	}
}
