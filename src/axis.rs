use enigo::{Enigo, KeyboardControllable};
use gilrs::Axis;
use serde::{Serialize, Deserialize};

use crate::config::Config;


const DEFAULT_DEADZONE: f32 = 0.1;


#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AxisState {
    Min,
    Mid,
    Max
}


fn get_axis_state(axis: Axis, value: f32, config: &Config) -> AxisState {
    let safe_deadzone = match config.get_deadzones().get(&axis) {
        Some(&d) => d,
        None => DEFAULT_DEADZONE
    };
    if value < -safe_deadzone {
        AxisState::Min
    } else if value > safe_deadzone {
        AxisState::Max
    } else {
        AxisState::Mid
    }
}


fn axis_state_changed(
    axis: Axis,
    current_state: AxisState,
    config: &Config
) -> bool {
    return config.get_axis_state(axis) != current_state
}


fn update_key_axis(
    enigo: &mut Enigo,
    axis: Axis,
    current_state: AxisState,
    config: &Config
) {
    match config.get_mapping_axis().get(&axis) {
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
    axis: Axis,
    value: f32,
    config: &mut Config
) {
    let current_state = get_axis_state(axis, value, config);
    if axis_state_changed(axis, current_state, config) {
        update_key_axis(enigo, axis, current_state, config);
        config.set_axis_state(axis, current_state);
    }
}