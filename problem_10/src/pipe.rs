#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn set_next_pos(&self, pos: &mut Pos) {
        match self {
            Direction::Up => pos.y -= 1,
            Direction::Down => pos.y += 1,
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pipe {
    Ground,
    Vertical,
    Horizontal,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

impl Pipe {
    pub fn get_directions(&self) -> Vec<Direction> {
        match self {
            Pipe::Vertical => vec![Direction::Up, Direction::Down],
            Pipe::Horizontal => vec![Direction::Left, Direction::Right],
            Pipe::UpRight => vec![Direction::Up, Direction::Right],
            Pipe::UpLeft => vec![Direction::Up, Direction::Left],
            Pipe::DownLeft => vec![Direction::Down, Direction::Left],
            Pipe::DownRight => vec![Direction::Down, Direction::Right],
            Pipe::Ground => vec![],
        }
    }

    pub fn get_next_direction(&self, prev_direction: &Direction) -> Direction {
        match self {
            Pipe::Vertical => match prev_direction {
                Direction::Up => Direction::Up,
                Direction::Down => Direction::Down,
                _ => unreachable!("How the heck did we end up here? {:?}", prev_direction),
            },
            Pipe::Horizontal => match prev_direction {
                Direction::Left => Direction::Left,
                Direction::Right => Direction::Right,
                _ => unreachable!("How the heck did we end up here? {:?}", prev_direction),
            },
            Pipe::UpRight => match prev_direction {
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                _ => unreachable!("How the heck did we end up here? {:?}", prev_direction),
            },
            Pipe::UpLeft => match prev_direction {
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Up,
                _ => unreachable!("How the heck did we end up here? {:?}", prev_direction),
            },
            Pipe::DownLeft => match prev_direction {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                _ => unreachable!("How the heck did we end up here? {:?}", prev_direction),
            },
            Pipe::DownRight => match prev_direction {
                Direction::Up => Direction::Right,
                Direction::Left => Direction::Down,
                _ => unreachable!("How the heck did we end up here? {:?}", prev_direction),
            },
            Pipe::Ground => unreachable!("How the heck did we end up here?"),
        }
    }
}
