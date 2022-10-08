#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Direction::Top => write!(f, "top"),
            Direction::Right => write!(f, "right"),
            Direction::Bottom => write!(f, "bottom"),
            Direction::Left => write!(f, "left"),
        }
    }
}
