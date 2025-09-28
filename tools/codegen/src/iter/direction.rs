use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    Forwards,
    Backwards,
}

impl Direction {
    pub fn lowercase_str(&self) -> &'static str {
        match self {
            Direction::Right => "right",
            Direction::Left => "left",
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Forwards => "forwards",
            Direction::Backwards => "backwards",
        }
    }

    pub fn uppercase_str(&self) -> &'static str {
        match self {
            Direction::Right => "RIGHT",
            Direction::Left => "LEFT",
            Direction::Up => "UP",
            Direction::Down => "DOWN",
            Direction::Forwards => "FORWARDS",
            Direction::Backwards => "BACKWARDS",
        }
    }

    pub fn camelcase_str(&self) -> &'static str {
        match self {
            Direction::Right => "Right",
            Direction::Left => "Left",
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Forwards => "Forwards",
            Direction::Backwards => "Backwards",
        }
    }

    pub fn opposite_dir(&self) -> Self {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Forwards => Direction::Backwards,
            Direction::Backwards => Direction::Forwards,
        }
    }

    pub fn axis(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Left => 0,
            Direction::Up => 1,
            Direction::Down => 1,
            Direction::Forwards => 2,
            Direction::Backwards => 2,
        }
    }
}
