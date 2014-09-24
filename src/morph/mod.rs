pub use nanovg::{Image, Font, Color};
pub use widget::Widget;
pub use Point;

pub enum Morph {
	ImageMorph(Image),
	TextMorph(String, Font),
	ShapeMorph(Shape),
	WidgetMorph(Widget),
	CompositeMorph(Vec<Box<Morph>>)
}

pub struct Shape {
	path: Vec<PathElement>,
	stroke_color: Color,
	stroke_width: u16,
	fill_color: Color
}
pub enum PathElement {
	MoveTo(Point),
	LineTo(Point),
	QuadTo(Point),
	BezierTo(Point),
	// etc...
}

pub trait Layout<'a> {
	fn layout(&self);
}
pub struct Composite<'a> {
	layout: Box<Layout<'a> + 'a>,
	children: Vec<Box<Morph>>,
}