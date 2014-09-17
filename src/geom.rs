use std::default::Default;
use std::mem::transmute;
use std::fmt;
use util::{min,max};

pub struct Point<T> {
	vals: [T, ..2]
}
impl<T: Num + Copy> Point<T> {
	pub fn new<T>(x:T, y:T) -> Point<T> { Point { vals: [x,y] } }
	pub fn x(&self) -> T { self.vals[0] }
	pub fn y(&self) -> T { self.vals[1] }
}
impl<T: Num + Copy> PartialEq for Point<T> {
	fn eq(&self, other: &Point<T>) -> bool {
		self.vals == other.vals
	}
}
impl<T: Num + Copy> Clone for Point<T> {
	fn clone(&self) -> Point<T> {
		Point { vals: self.vals }
	}
}
#[test]
fn testClone() {
	let mut a: Point<i32> = Point::<i32>::new(1,2);
	let b: Point<i32> = a.clone();
	a.vals[0] = 3;
	assert!(a.x() == 3);
	assert!(b.x() == 1);
}
impl<T: Num + Copy + fmt::Show> fmt::Show for Point<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
		write!(f, "Point(x:{},y:{})", self.vals[0], self.vals[1])
	}
}


pub struct Rect<T> {
	vals: [T, ..4]
}
impl<T: Num + Copy + PartialOrd + Default>
Rect<T> {
	pub fn zero() -> Rect<T> {
		let z: T = Default::default();
		Rect { vals: [z,z,z,z] }
	}

	pub fn x(&self) -> T { self.vals[0] }
	pub fn y(&self) -> T { self.vals[1] }
	pub fn w(&self) -> T { self.vals[2] }
	pub fn h(&self) -> T { self.vals[3] }

    pub fn as_slice(&self) -> &[T, ..4u] { unsafe { transmute(self) } }
    pub fn as_mut_slice(&mut self) -> &mut [T, ..4u] { unsafe { transmute(self) } }

	pub fn origin(&self) -> Point<T> { Point::<T>::new(self.x(), self.y()) }
	pub fn corner(&self) -> Point<T> { Point::<T>::new(self.x()+self.w(), self.y()+self.h()) }

	// inappropriate? prefer immutable
	pub fn move_to(&mut self, x: T, y: T) { self[0] = x; self[1] = y; }

	pub fn union(&self, other: &Rect<T>) -> Rect<T> {
		let x1 = min(self.x(), other.x());
		let y1 = min(self.y(), other.y());
		let x2 = max(self.x()+self.w(), other.x()+other.w());
		let y2 = max(self.y()+self.h(), other.y()+other.h());
		if x2 <= x1 || y2 <= y1 { return Rect::zero(); }
		Rect { vals: [x1, y1, x2-x1, y2-y1] }
	}
	pub fn intersection(&self, other: &Rect<T>) -> Rect<T> {
		let x1 = max(self.x(), other.x());
		let y1 = max(self.y(), other.y());
		let x2 = min(self.x()+self.w(), other.x()+other.w());
		let y2 = min(self.y()+self.h(), other.y()+other.h());
		if x2 < x1 || y2 < y1 { return Rect::zero(); }
		Rect { vals: [x1, y1, x2-x1, y2-y1] }
	}
    pub fn collides(&self, bbox: &Rect<T>) -> bool {
	  // true iff self and bbox collide (intersect)
	  return  (bbox.x() > self.x() - bbox.w()) &&
	          (bbox.y() > self.y() - bbox.h()) &&
	          (bbox.x() < self.x() + self.w()) &&
	          (bbox.y() < self.y() + self.h())
    }
    pub fn contains_point(&self, x: T, y: T) -> bool {
	  // true iff abox contains point(x,y)
	  return  (x > self.x()) &&
	          (y > self.y()) &&
	          (x < self.x() + self.w()) &&
	          (y < self.y() + self.h())
    }
    pub fn contains(&self, bbox: &Rect<T>) -> bool {
	  // true if self contains bbox: that is, bbox is fully inside self
	  return  bbox.x() >= self.x() &&
	          bbox.y() >= self.y() &&
	          (bbox.x() + bbox.w()) <= (self.x() + self.w()) &&
	          (bbox.y() + bbox.h()) <= (self.y() + self.h())
    }
}
impl<T: Num + Copy> PartialEq for Rect<T> {
	fn eq(&self, other: &Rect<T>) -> bool {
		self.vals == other.vals
	}
}
impl<T: Num + Copy> Clone for Rect<T> {
	fn clone(&self) -> Rect<T> {
		Rect { vals: self.vals } // is this shallow or deep copy?
	}
}
impl<T: Num + Copy + fmt::Show> fmt::Show for Rect<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
		write!(f, "Rect(x:{},y:{}, w:{},h:{})", self.vals[0], self.vals[1], self.vals[2], self.vals[3])
	}
}

impl<'a, T: Num + Copy + PartialOrd + Default> Index<uint, T> for Rect<T> {
    fn index<'a>(&'a self, index: &uint) -> &'a T {
    	&self.vals[*index]
    }
}
impl<'a, T: Num + Copy + PartialOrd + Default> IndexMut<uint, T> for Rect<T> {
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut T {
    	&mut self.vals[*index]
    }
}
