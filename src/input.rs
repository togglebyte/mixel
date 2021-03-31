use nightmaregl::events::Key as WinitKey;

// -----------------------------------------------------------------------------
//     - Actions -
// -----------------------------------------------------------------------------
pub enum Action {
    Left,
    Right,
    Up,
    Down,
    Draw,
    CommandInput,
    CloseCommandInput,
}

// -----------------------------------------------------------------------------
//     - Input mode -
//     If it's mapped then produce actions
// -----------------------------------------------------------------------------
pub enum InputMode {
    Mapped,
    Raw,
}

// -----------------------------------------------------------------------------
//     - Key input -
//     So we can overwrite the `Key` in case of a char
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub enum Key {
    Key(WinitKey),
    Char(char),
    Empty,
}

// -----------------------------------------------------------------------------
//     - Input -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Input {
    key: Key,
    input_map: InputMap,
}

impl Input {
    pub fn new() -> Self {
        Self {
            key: Key::Empty,
            input_map: InputMap::new(),
        }
    }

    pub fn update(&mut self, value: Key) {
        self.key = value;
    }

    pub fn take(&mut self) -> Key {
        let mut key = Key::Empty;
        std::mem::swap(&mut self.key, &mut key);
        key
    }

    pub fn action(&self) -> Option<Action> {
        self.input_map.map(self.key)
    }

    pub fn consume(&mut self) {
        self.key = Key::Empty;
    }
}

// -----------------------------------------------------------------------------
//     - Input map -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct InputMap {
}

impl InputMap {
    pub fn new() -> Self {
        Self {
            
        }
    }

    fn map(&self, key: Key) -> Option<Action> {
        match key {
            Key::Char('h') => Some(Action::Left),
            Key::Char('j') => Some(Action::Down),
            Key::Char('k') => Some(Action::Up),
            Key::Char('l') => Some(Action::Right),
            Key::Key(WinitKey::Space) => Some(Action::Draw),
            Key::Key(WinitKey::Colon) => Some(Action::CommandInput),
            _ => None,
        }
    }
}
