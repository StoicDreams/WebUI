#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Direction {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LeftOrRight {
    #[default]
    Right,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LeftCenterRight {
    #[default]
    Left,
    Center,
    Right,
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

impl std::fmt::Display for LeftOrRight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LeftOrRight::Right => write!(f, "right"),
            LeftOrRight::Left => write!(f, "left"),
        }
    }
}

impl std::fmt::Display for LeftCenterRight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LeftCenterRight::Right => write!(f, "right"),
            LeftCenterRight::Center => write!(f, "center"),
            LeftCenterRight::Left => write!(f, "left"),
        }
    }
}
