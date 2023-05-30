use std::collections::HashMap;

use enigo::{Enigo, Key, KeyboardControllable};
use gilrs::Axis;
use serde::{Serialize, Deserialize};


#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub enum AxisState {
    Min,
    Mid,
    Max
}


fn get_axis_state(axis: &Axis, value: f32, deadzones: &HashMap<Axis, f32>) -> Option<AxisState> {
    match deadzones.get(axis) {
        Some(&deadzone) =>
            if value < - deadzone {
                Some(AxisState::Min)
            } else if value > deadzone {
                Some(AxisState::Max)
            } else {
                Some(AxisState::Mid)
            },
        None => None
    }
}


fn axis_state_changed(
    axis: &Axis,
    current_state: &AxisState,
    axis_states: &HashMap<Axis, AxisState> 
) -> bool {
    return axis_states.get(axis) != Some(current_state)
}


fn update_key_axis(
    enigo: &mut Enigo,
    axis: &Axis,
    current_state: &AxisState,
    mapping_axis: &HashMap<Axis, (Option<Key>, Option<Key>)>
) {
    match mapping_axis.get(axis) {
        Some(&(key_min, key_max)) => {
            match current_state {
                AxisState::Min => {
                    match key_min {
                        Some(k) => enigo.key_down(k),
                        None => println!("No key min associated with {:?}", axis)
                    };
                    match key_max {
                        Some(k) => enigo.key_up(k),
                        None => println!("No key max associated with {:?}", axis)
                    }
                },
                AxisState::Mid => {
                    match key_min {
                        Some(k) => enigo.key_up(k),
                        None => println!("No key min associated with {:?}", axis)
                    };
                    match key_max {
                        Some(k) => enigo.key_up(k),
                        None => println!("No key max associated with {:?}", axis)
                    }
                },
                AxisState::Max => {
                    match key_min {
                        Some(k) => enigo.key_up(k),
                        None => println!("No key min associated with {:?}", axis)
                    };
                    match key_max {
                        Some(k) => enigo.key_down(k),
                        None => println!("No key max associated with {:?}", axis)
                    }
                }
            }
        },
        None => println!("No key is associated with {:?}", axis)
    };
}


pub fn update_axis(
    enigo: &mut Enigo,
    axis: &Axis,
    value: f32,
    mapping_axis: &HashMap<Axis, (Option<Key>, Option<Key>)>,
    axis_states: &mut HashMap<Axis, AxisState>,
    deadzones: &HashMap<Axis, f32>
) {
    let current_state = get_axis_state(axis, value, deadzones);
    match current_state {
        Some(current_state) => {
            if axis_state_changed(axis, &current_state, axis_states) {
                update_key_axis(enigo, axis, &current_state, mapping_axis);
                axis_states.insert(*axis, current_state);
            }
        },
        None => ()
    }
}