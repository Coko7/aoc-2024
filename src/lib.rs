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

    pub fn dist(&self, other: &Pos2D) -> f64 {
        let xd = (other.x - self.x).pow(2) as f64;
        let yd = (other.y - self.y).pow(2) as f64;
        (xd + yd).sqrt()
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

impl PartialEq for Pos2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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

#[derive(Debug)]
pub struct Map2D<T> {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<T>,
}

impl<T> Map2D<T> {
    pub fn pos2idx(&self, position: &Pos2D) -> Option<usize> {
        pos2idx(position, self.width, self.height)
    }

    pub fn idx2pos(&self, idx: usize) -> Pos2D {
        idx2pos(idx, self.width, self.height)
    }

    pub fn is_valid_pos(&self, position: &Pos2D) -> bool {
        position.x >= 0
            && position.x < (self.width as i32)
            && position.y >= 0
            && position.y < (self.height as i32)
    }

    pub fn get(&self, position: &Pos2D) -> Option<&T> {
        if let Some(idx) = self.pos2idx(position) {
            return Some(&self.tiles[idx]);
        }

        None
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

pub fn pos2idx(position: &Pos2D, width: usize, height: usize) -> Option<usize> {
    if position.x < 0 || position.x >= width as i32 {
        return None;
    }

    if position.y < 0 || position.y >= height as i32 {
        return None;
    }

    Some((position.y as usize * height) + position.x as usize)
}

pub fn idx2pos(idx: usize, width: usize, height: usize) -> Pos2D {
    let x = (idx % width) as i32;
    let y = (idx / height) as i32;
    Pos2D::new(x, y)
}
