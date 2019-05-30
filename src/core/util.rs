#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn zero() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

impl Size {
    pub fn zero() -> Size {
        Size { w: 0.0, h: 0.0 }
    }

    pub fn smallest(&self, other: &Size) -> Size {
        Size {
            w: self.w.min(other.w),
            h: self.h.min(other.h),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    pub fn zero() -> Self {
        Rect {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
        }
    }

    pub fn with_position_size(top_left: &Point, size: &Size) -> Self {
        Rect {
            x: top_left.x,
            y: top_left.y,
            w: size.w,
            h: size.h,
        }
    }

    pub fn size(&self) -> Size {
        Size {
            w: self.w,
            h: self.h,
        }
    }

    pub fn top_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    /// Returns a rectangle of the given size that is centered on the current rectangle.
    pub fn center(&self, size: Size) -> Rect {
        Rect {
            x: self.x + (self.w - size.w) / 2.0,
            y: self.y + (self.h - size.h) / 2.0,
            w: size.w,
            h: size.h,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x && point.x < self.x + self.w
        && point.y >= self.y && point.y < self.y + self.h
    }
}