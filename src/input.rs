use std::fs::read_to_string;
use serde::Deserialize;
use nightmaregl::events::{Key, KeyState};

// -----------------------------------------------------------------------------
//     - Actions -
// -----------------------------------------------------------------------------
#[derive(Debug)]
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
//     - Input -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Input {
    key: Option<char>,
    input_map: InputMap,
    state: KeyState,
    ctrl: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            key: None,
            input_map: InputMap::new(),
            state: KeyState::Released,
            ctrl: false,
        }
    }

    pub fn update(&mut self, c: char) {
        self.key = Some(c);
    }
    
    pub fn update_modifier(&mut self, key: Key, state: KeyState) {
        match (key, state) {
            (Key::LControl, KeyState::Pressed) => self.ctrl = true,
            (Key::RControl, KeyState::Pressed) => self.ctrl = true,
            _ => {}
        }

        match (state, key) {
            (KeyState::Released, Key::LControl) => self.ctrl = false,
            (KeyState::Released, Key::RControl) => self.ctrl = false,
            _ => return,
        }
    }

    pub fn take(&mut self) -> Option<char> {
        let mut new_val = None;
        std::mem::swap(&mut self.key, &mut new_val);
        new_val
    }

    pub fn action(&self) -> Option<Action> {
        None
        // self.input_map.map_input(self.key)
    }

    pub fn consume(&mut self) {
        self.key = None;
    }
}

// -----------------------------------------------------------------------------
//     - Input map -
// -----------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct InputMap {
    keys: Keys,
}

impl InputMap {
    pub fn new() -> Self {
        let config = read_to_string("config.toml").unwrap();
        toml::from_str(&config).unwrap()
    }

    fn map_input(&self, c: char) -> Option<Action> {
        match c {
            'h' => Some(Action::Left),
            'j' => Some(Action::Down),
            'k' => Some(Action::Up),
            'l' => Some(Action::Right),
            ' ' => Some(Action::Draw),
            ':' => Some(Action::CommandInput),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Keys {
    left: String,
    up: String,
    down: String,
    right: String,
    ex: String,
    insert: String,
}
