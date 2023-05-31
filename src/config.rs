use std::collections::HashMap;
use std::io::Error;

use gilrs::ev::{Axis, Button};
use enigo::Key;
use serde::{Serialize, Deserialize};

use crate::axis::AxisState;
use crate::utils::{read_from_path, save_to_path};


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,

    mapping_buttons: HashMap<Button, Key>,
    pressed_buttons: HashMap<Button, bool>,

    mapping_axis: HashMap<Axis, (Option<Key>, Option<Key>)>,
    states_axis: HashMap<Axis, AxisState>,
    deadzones: HashMap<Axis, f32>
}


impl Config {
    pub fn new(
        name: String,
        mapping_buttons: HashMap<Button, Key>,
        mapping_axis: HashMap<Axis, (Option<Key>, Option<Key>)>,
        deadzones: HashMap<Axis, f32>
    ) -> Self {
        let mut pressed_buttons = HashMap::new();
        for (button, _) in mapping_buttons {
            pressed_buttons.insert(button, false);
        }
        let mut states_axis = HashMap::new();
        for (axis, _) in mapping_axis {
            states_axis.insert(axis, AxisState::Mid);
        }
        Config {
            name,
            mapping_buttons,
            pressed_buttons,
            mapping_axis,
            states_axis,
            deadzones
        }
    }

    pub fn default() -> Self {
        Config {
            name: "default".to_string(),
            mapping_buttons: HashMap::new(),
            pressed_buttons: HashMap::new(),
            mapping_axis: HashMap::new(),
            states_axis: HashMap::new(),
            deadzones: HashMap::new()
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        let data =  serde_json::to_string(&self)?;
        save_to_path(&self.name, &data)
    }

    pub fn get_mapping_buttons(&self) -> &HashMap<Button, Key> {
        &self.mapping_buttons
    }

    pub fn maps_button_to(&self, button: Button, key: Key) {
        self.mapping_buttons.insert(button, key);
        self.pressed_buttons.insert(button, false);
    }

    pub fn is_button_pressed(&self, button: Button) -> bool {
        match self.pressed_buttons.get(&button) {
            Some(&b) => b,
            None => false
        }
    }

    pub fn set_button_pressed(&mut self, button: Button, is_pressed: bool) {
        self.pressed_buttons.insert(button, is_pressed);
    }

    pub fn get_mapping_axis(&self) -> &HashMap<Axis, (Option<Key>, Option<Key>)> {
        &self.mapping_axis
    }

    pub fn maps_axis_to(&self, axis: Axis, state: AxisState, key: Key) {
        let existing_bind = match self.mapping_axis.get(&axis) {
            Some(&b) => b,
            None => (None, None)
        };
        let new_bind = match state {
            AxisState::Min => (Some(key), existing_bind.1),
            AxisState::Max => (existing_bind.0, Some(key)),
            AxisState::Mid => existing_bind
        };
        self.mapping_axis.insert(axis, new_bind);
        self.states_axis.insert(axis, AxisState::Mid);
    }

    pub fn get_axis_state(&self, axis: Axis) -> AxisState {
        match self.states_axis.get(&axis) {
            Some(&ax) => ax,
            None => AxisState::Mid
        }
    }

    pub fn set_axis_state(&self, axis: Axis, axis_state: AxisState) {
        self.states_axis.insert(axis, axis_state);
    }

    pub fn get_deadzones(&self) -> &HashMap<Axis, f32> {
        &self.deadzones
    }

    pub fn set_deadzone(&self, axis: Axis, value: f32) {
        self.deadzones.insert(axis, value);
    }
}


pub fn load(name: &String) -> Result<Config, Error> {
    let str_data = read_from_path(name)?;
    let config: Config = serde_json::from_str(&str_data)?;
    Ok(config)
}