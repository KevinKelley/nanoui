
pub use nanovg::{Image, Font, Color};
pub use Point;

pub mod button;
pub mod checkbox;
pub mod colorwheel;
pub mod dropdown;
pub mod editbox;
pub mod graph;
pub mod label;
pub mod paragraph;
pub mod slider;
pub mod spinner;
pub mod window;

pub mod unused;

pub enum Morph {
	ImageMorph(Image),
	TextMorph(String, Font),
	ShapeMorph(Shape),
	WidgetMorph(Widget),
	CompositeMorph(Vec<Box<Morph>>)
}

pub struct Shape {
	path: Path,
	stroke: Color,
	fill: Color
}
pub enum Path {
	MoveTo(Point),
	LineTo(Point),
	QuadTo(Point),
	BezierTo(Point),
}

pub enum Widget {
	Button,
	Checkbox,
	Radio,
	Colorwheel,
	Dropdown,
	Editbox,
	Graph,
	Label,
	Paragraph,
	Slider,
	Spinner,
	Window
}
