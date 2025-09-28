#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Axis(pub usize);

impl Axis {
    pub fn lowercase_str(&self) -> &'static str {
        match self.0 {
            0 => "x",
            1 => "y",
            2 => "z",
            3 => "w",
            n => panic!("invalid axis: {n}"),
        }
    }

    pub fn uppercase_str(&self) -> &'static str {
        match self.0 {
            0 => "X",
            1 => "Y",
            2 => "Z",
            3 => "W",
            n => panic!("invalid axis: {n}"),
        }
    }

    pub fn ordinal_str(&self) -> &'static str {
        match self.0 {
            0 => "1st",
            1 => "2nd",
            2 => "3rd",
            3 => "4th",
            n => panic!("invalid axis: {n}"),
        }
    }
}
