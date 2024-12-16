pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug, Clone, Copy)]
pub struct Pos2D {
    pub x: i32,
    pub y: i32,
}

impl Pos2D {
    pub fn new(x: i32, y: i32) -> Pos2D {
        Pos2D { x, y }
    }

    pub fn right(&self, offset: i32) -> Pos2D {
        Pos2D::new(self.x + offset, self.y)
    }

    pub fn left(&self, offset: i32) -> Pos2D {
        Pos2D::new(self.x - offset, self.y)
    }

    pub fn top(&self, offset: i32) -> Pos2D {
        Pos2D::new(self.x, self.y - offset)
    }

    pub fn bot(&self, offset: i32) -> Pos2D {
        Pos2D::new(self.x, self.y + offset)
    }

    pub fn add<T>(&self, other: T) -> Pos2D
    where
        T: AsRef<Pos2D>,
    {
        let other = other.as_ref();
        Pos2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AsRef<Pos2D> for Pos2D {
    fn as_ref(&self) -> &Pos2D {
        self
    }
}

impl AsRef<Pos2D> for (i32, i32) {
    fn as_ref(&self) -> &Pos2D {
        unsafe { &*(self as *const (i32, i32) as *const Pos2D) }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate(direction: &Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
