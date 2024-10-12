pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

pub struct Rectangle {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rectangle {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        assert!(left <= right, "left must be less than or equal to right");
        assert!(top <= bottom, "top must be less than or equal to bottom");
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
    #[inline(always)]
    pub fn width(&self) -> u32 {
        (self.right - self.left) as u32
    }
    #[inline(always)]
    pub fn height(&self) -> u32 {
        (self.bottom - self.top) as u32
    }
    pub fn dimension(&self) -> Dimensions {
        Dimensions { width: self.width(), height: self.height() }
    }
}


pub(crate) static mut FULLSCREEN: Rectangle = Rectangle {
    left: 0,
    top: 0,
    right: 0,
    bottom: 0,
};

