use std::collections::HashMap;

use enigo::{Enigo, Key, KeyboardControllable};
use gilrs::Button;

use crate::config::Config;


fn button_state_changed(
    button: Button,
    is_pressed: bool,
    config: &Config
) -> bool {
    return config.is_button_pressed(button) != is_pressed
}


fn update_key_button(
    enigo: &mut Enigo,
    button: Button,
    is_pressed: bool,
    mapping_buttons: &HashMap<Button, Key>
) {
    match mapping_buttons.get(&button) {
        Some(&key) => {
            if is_pressed {
                enigo.key_down(key);
            } else {
                enigo.key_up(key);
            }
        },
        None => println!("No key is associated with {:?}", button)
    };
}


pub fn update_button(
    enigo: &mut Enigo,
    button: Button,
    is_pressed: bool,
    config: &mut Config
) {
    if button_state_changed(button, is_pressed, config) {
        update_key_button(enigo, button, is_pressed, config.get_mapping_buttons());
        config.set_button_pressed(button, is_pressed);
    }
}