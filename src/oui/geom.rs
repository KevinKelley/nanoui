use std::mem::transmute;

#[deriving(Eq,PartialEq, Show)]
#[repr(C)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}
impl Vec2 {
    pub fn zero() -> Vec2 { Vec2 { x: 0, y: 0 } }
    pub fn as_mut_slice(&mut self) -> &mut [i32, ..2u] { unsafe { transmute(self) } }
}

impl<'a> Index<uint, i32> for Vec2 {
    fn index<'a>(&'a self, index: &uint) -> &'a i32 {
        match *index {
            0u => { &self.x },
            1u => { &self.y },
            _  => { fail!("bad index: {}!", *index) }
        }
    }
}
impl<'a> IndexMut<uint, i32> for Vec2 {
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut i32 {
        match *index {
            0u => { &mut self.x },
            1u => { &mut self.y },
            _  => { fail!("bad index: {}!", *index) }
        }
    }
}


#[deprecated] //"use geom::Point instead of draw::geom::Point"
#[deriving(Eq,PartialEq, Show)]
#[repr(C)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
impl Rect {
    pub fn zero() -> Rect { Rect { x:0, y:0, w:0, h:0 } }
    pub fn as_mut_slice(&mut self) -> &mut [i32, ..4u] { unsafe { transmute(self) } }
}

impl<'a> Index<uint, i32> for Rect {
    fn index<'a>(&'a self, index: &uint) -> &'a i32 {
        match *index {
            0u => { &self.x },
            1u => { &self.y },
            2u => { &self.w },
            3u => { &self.h },
            _  => { fail!("bad index: {}!", *index) }
        }
    }
}
impl<'a> IndexMut<uint, i32> for Rect {
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut i32 {
        match *index {
            0u => { &mut self.x },
            1u => { &mut self.y },
            2u => { &mut self.w },
            3u => { &mut self.h },
            _  => { fail!("bad index: {}!", *index) }
        }
    }
}
