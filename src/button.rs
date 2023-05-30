use std::collections::HashMap;

use enigo::{Enigo, Key, KeyboardControllable};
use gilrs::Button;


fn button_state_changed(
    button: &Button,
    is_pressed: bool,
    pressed_buttons: &mut HashMap<Button, bool> 
) -> bool {
    return pressed_buttons.get(button) != Some(&is_pressed)
}


fn update_key_button(
    enigo: &mut Enigo,
    button: &Button,
    is_pressed: bool,
    mapping_buttons: &HashMap<Button, Key>
) {
    match mapping_buttons.get(button) {
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
    button: &Button,
    is_pressed: bool,
    mapping_buttons: &HashMap<Button, Key>,
    pressed_buttons: &mut HashMap<Button, bool>,
) {
    if button_state_changed(button, is_pressed, pressed_buttons) {
        update_key_button(enigo, button, is_pressed, mapping_buttons);
        pressed_buttons.insert(*button, is_pressed);
    }
}