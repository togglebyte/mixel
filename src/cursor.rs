use nightmaregl::Position;

pub struct Cursor {
    pub position: Position<i32>,
    pub dirty: bool,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            position: Position::zero(),
            dirty: false,
        }
    }
}
