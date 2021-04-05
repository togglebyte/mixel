use std::collections::HashMap;
use std::fs::read_to_string;

use anyhow::Result;
use nightmaregl::events::{Key, KeyState};
use serde::Deserialize;

use crate::Mode;

// -----------------------------------------------------------------------------
//     - Actions -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub enum Action {
    Left,
    Right,
    Up,
    Down,
    Draw,
    CommandInput,
    CloseCommandInput,
    Noop,
}

impl Action {
    fn from_str(s: &str) -> Action {
        match s.to_ascii_lowercase().as_ref() {
            "left" => Action::Left,
            "right" => Action::Right,
            "up" => Action::Up,
            "down" => Action::Down,
            _ => Action::Noop,
        }
    }
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
pub struct InputHandler {
    pub key: Option<char>,
    pub ctrl: bool,
    input_map: InputMap,
    state: KeyState,
}

impl InputHandler {
    pub fn new() -> Result<Self> {
        let inst = Self {
            key: None,
            input_map: InputMap::new()?,
            state: KeyState::Released,
            ctrl: false,
        };

        Ok(inst)
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

    pub fn to_action(&self, c: char, mode: Mode) -> Option<Action> {
        self.input_map.map_input(c, mode, self.ctrl)
    }

    pub fn consume(&mut self) {
        self.key = None;
    }
}

// -----------------------------------------------------------------------------
//     - Input map -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct InputMap {
    normal: KeyMap,
    insert: KeyMap,
    visual: KeyMap,
}

impl InputMap {
    pub fn new() -> Result<Self> {
        let config = read_to_string("config.toml")?;
        let cfg: toml::Value = toml::from_str(&config)?;

        let inst = InputMap {
            normal: KeyMap::from_val(cfg.get("normal").map(toml::Value::to_owned)),
            insert: KeyMap::from_val(cfg.get("insert").map(toml::Value::to_owned)),
            visual: KeyMap::from_val(cfg.get("visual").map(toml::Value::to_owned)),
        };

        Ok(inst)
    }

    fn map_input(&self, c: char, mode: Mode, ctrl: bool) -> Option<Action> {
        match mode {
            Mode::Insert => self.insert.map_input(c, ctrl),
            Mode::Normal => self.normal.map_input(c, ctrl),
            Mode::Visual => self.visual.map_input(c, ctrl),
            Mode::Command => return None,
        }
    }
}

#[derive(Debug)]
struct KeyMap(HashMap<char, Action>);

impl KeyMap {
    fn map_input(&self, c: char, ctrl: bool) -> Option<Action> {
        self.0.get(&c).map(|a| *a)
    }

    fn from_val(mut val: Option<toml::Value>) -> KeyMap {
        let mut key_values = HashMap::new();
        let mut val = match val.take() {
            Some(v) => v,
            None => return KeyMap(key_values),
        };

        let mut table = match val.as_table_mut() {
            Some(t) => t,
            None => return KeyMap(key_values),
        };

        // -----------------------------------------------------------------------------
        //     - This is a hot mess -
        //     This should be fixed:
        //     * keys should not be chars, but rather virtual keycodes
        // -----------------------------------------------------------------------------
        for (k, v) in table
            .iter_mut()
            .filter(|(_, v)| v.is_str())
            .map(|(k, v)| (k, v.as_str().unwrap().to_owned()))
        {
            let action = Action::from_str(&k);
            let c = v.chars().next().unwrap();
            key_values.insert(c, action);
        }

        KeyMap(key_values)
    }
}

// -----------------------------------------------------------------------------
//     - Input handler -
// -----------------------------------------------------------------------------
pub trait Input {
    fn input(&mut self, c: char, mode: Mode, input: &InputHandler);
}
