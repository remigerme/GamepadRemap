use std::collections::HashMap;
use std::io::Error;

use gilrs::ev::{Axis, Button};
use enigo::Key;
use serde::{Serialize, Deserialize};

use crate::axis::AxisState;
use crate::utils::{read_from_path, save_to_path};


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    name: String,
    mapping_buttons: HashMap<Button, Key>,
    states_buttons: HashMap<Button, bool>,

    mapping_axis: HashMap<Axis, (Option<Key>, Option<Key>)>,
    states_axis: HashMap<Axis, AxisState>,
    deadzones: HashMap<Axis, f64>
}


impl Config {
    pub fn new(
        name: String,
        mapping_buttons: HashMap<Button, Key>,
        states_buttons: HashMap<Button, bool>,
        mapping_axis: HashMap<Axis, (Option<Key>, Option<Key>)>,
        states_axis: HashMap<Axis, AxisState>,
        deadzones: HashMap<Axis, f64>
    ) -> Self {
        Config {
            name,
            mapping_buttons,
            states_buttons,
            mapping_axis,
            states_axis,
            deadzones
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        let data =  serde_json::to_string(&self)?;
        save_to_path(&self.name, &data)
    }
}


pub fn load(name: &String) -> Result<Config, Error> {
    let str_data = read_from_path(name)?;
    let config: Config = serde_json::from_str(&str_data)?;
    Ok(config)
}