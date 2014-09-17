use util::{min,max};


#[deprecated] //"use geom::Point instead of draw::geom::Point"
#[deriving(PartialEq, Clone, Show)]
pub struct Point {
	pub x: f32,
	pub y: f32
}

#[deprecated] //"use geom::Rect instead of draw::geom::Rect"
#[deriving(PartialEq, Clone, Show)]
pub struct Rect {
	x: f32,
	y: f32,
	w: f32,
	h: f32
}
impl Rect {
	pub fn zero() -> Rect { Rect { x:0.0, y:0.0, w:0.0, h:0.0 } }

	pub fn x(&self) -> f32 { self.x }
	pub fn y(&self) -> f32 { self.y }
	pub fn w(&self) -> f32 { self.w }
	pub fn h(&self) -> f32 { self.h }

	pub fn origin(&self) -> Point { Point { x: self.x(), y: self.y() } }
	pub fn corner(&self) -> Point { Point { x: self.x()+self.w(), y: self.y()+self.h() } }


	pub fn move_to(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
	}
	pub fn union(&self, other: &Rect) -> Rect {
		let x1 = min(self.x, other.x);
		let y1 = min(self.y, other.y);
		let x2 = max(self.x+self.w, other.x+other.w);
		let y2 = max(self.y+self.h, other.y+other.h);
		if x2 <= x1 || y2 <= y1 { return Rect::zero(); }
		Rect { x:x1, y:y1, w:x2-x1, h:y2-y1 }
	}
	pub fn intersection(&self, other: &Rect) -> Rect {
		let x1 = max(self.x, other.x);
		let y1 = max(self.y, other.y);
		let x2 = min(self.x+self.w, other.x+other.w);
		let y2 = min(self.y+self.h, other.y+other.h);
		if x2 < x1 || y2 < y1 { return Rect::zero(); }
		Rect { x:x1, y:y1, w:x2-x1, h:y2-y1 }
	}
    pub fn collides(&self, bbox: &Rect) -> bool {
	  // true iff self and bbox collide (intersect)
	  return  (bbox.x > self.x - bbox.w) &&
	          (bbox.y > self.y - bbox.h) &&
	          (bbox.x < self.x + self.w) &&
	          (bbox.y < self.y + self.h)
    }
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
	  // true iff abox contains point(x,y)
	  return  (x > self.x) &&
	          (y > self.y) &&
	          (x < self.x + self.w) &&
	          (y < self.y + self.h)
    }
    pub fn contains(&self, bbox: &Rect) -> bool {
	  // true if self contains bbox: that is, bbox is fully inside self
	  return  bbox.x >= self.x &&
	          bbox.y >= self.y &&
	          (bbox.x + bbox.w) <= (self.x + self.w) &&
	          (bbox.y + bbox.h) <= (self.y + self.h)
    }
}
