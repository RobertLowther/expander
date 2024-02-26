use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashMap;

pub fn run() -> std::collections::HashMap<String, Vec<String>> {
    let device_state = DeviceState::new();
    
    let ctrl_keycodes = vec![
        Keycode::LShift,
        Keycode::RShift,
        Keycode::LControl,
        Keycode::RControl,
        Keycode::LAlt,
        Keycode::RAlt,
        Keycode::Escape,
    ];

    let shift_keycodes = vec![
        Keycode::LShift,
        Keycode::RShift,
    ];

    let mut letter_shift_map: HashMap<Keycode, String> = HashMap::new();
    letter_shift_map.insert(Keycode::A, "a".to_string());
    letter_shift_map.insert(Keycode::B, "b".to_string());
    letter_shift_map.insert(Keycode::C, "c".to_string());
    letter_shift_map.insert(Keycode::D, "d".to_string());
    letter_shift_map.insert(Keycode::E, "e".to_string());
    letter_shift_map.insert(Keycode::F, "f".to_string());
    letter_shift_map.insert(Keycode::G, "g".to_string());
    letter_shift_map.insert(Keycode::H, "h".to_string());
    letter_shift_map.insert(Keycode::I, "i".to_string());
    letter_shift_map.insert(Keycode::J, "j".to_string());
    letter_shift_map.insert(Keycode::K, "k".to_string());
    letter_shift_map.insert(Keycode::L, "l".to_string());
    letter_shift_map.insert(Keycode::M, "m".to_string());
    letter_shift_map.insert(Keycode::N, "n".to_string());
    letter_shift_map.insert(Keycode::O, "o".to_string());
    letter_shift_map.insert(Keycode::P, "p".to_string());
    letter_shift_map.insert(Keycode::Q, "q".to_string());
    letter_shift_map.insert(Keycode::R, "r".to_string());
    letter_shift_map.insert(Keycode::S, "s".to_string());
    letter_shift_map.insert(Keycode::T, "t".to_string());
    letter_shift_map.insert(Keycode::U, "u".to_string());
    letter_shift_map.insert(Keycode::V, "v".to_string());
    letter_shift_map.insert(Keycode::W, "w".to_string());
    letter_shift_map.insert(Keycode::X, "x".to_string());
    letter_shift_map.insert(Keycode::Y, "y".to_string());
    letter_shift_map.insert(Keycode::Z, "z".to_string());
    
    let mut char_shift_map: HashMap <Keycode, String> = HashMap::new();
    char_shift_map.insert(Keycode::Key1, "!".to_string());
    char_shift_map.insert(Keycode::Key2, "@".to_string());
    char_shift_map.insert(Keycode::Key3, "#".to_string());
    char_shift_map.insert(Keycode::Key4, "$".to_string());
    char_shift_map.insert(Keycode::Key5, "%".to_string());
    char_shift_map.insert(Keycode::Key6, "^".to_string());
    char_shift_map.insert(Keycode::Key7, "&".to_string());
    char_shift_map.insert(Keycode::Key8, "*".to_string());
    char_shift_map.insert(Keycode::Key9, "(".to_string());
    char_shift_map.insert(Keycode::Key0, ")".to_string());
    char_shift_map.insert(Keycode::Apostrophe, "\"".to_string());
    char_shift_map.insert(Keycode::Semicolon, ":".to_string());
    char_shift_map.insert(Keycode::LeftBracket, "{".to_string());
    char_shift_map.insert(Keycode::RightBracket, "}".to_string());
    char_shift_map.insert(Keycode::Minus, "_".to_string());
    char_shift_map.insert(Keycode::Equal, "+".to_string());
    char_shift_map.insert(Keycode::BackSlash, "|".to_string());
    char_shift_map.insert(Keycode::Comma, "<".to_string());
    char_shift_map.insert(Keycode::Dot, ">".to_string());
    char_shift_map.insert(Keycode::Slash, "?".to_string());
    char_shift_map.insert(Keycode::Grave, "~".to_string());

    let mut symbol_map: HashMap <Keycode, String> = HashMap::new();
    symbol_map.insert(Keycode::Key1, "1".to_string());
    symbol_map.insert(Keycode::Key2, "2".to_string());
    symbol_map.insert(Keycode::Key3, "3".to_string());
    symbol_map.insert(Keycode::Key4, "4".to_string());
    symbol_map.insert(Keycode::Key5, "5".to_string());
    symbol_map.insert(Keycode::Key6, "6".to_string());
    symbol_map.insert(Keycode::Key7, "7".to_string());
    symbol_map.insert(Keycode::Key8, "8".to_string());
    symbol_map.insert(Keycode::Key9, "9".to_string());
    symbol_map.insert(Keycode::Key0, "0".to_string());
    symbol_map.insert(Keycode::Apostrophe, "'".to_string());
    symbol_map.insert(Keycode::Semicolon, ";".to_string());
    symbol_map.insert(Keycode::LeftBracket, "[".to_string());
    symbol_map.insert(Keycode::RightBracket, "]".to_string());
    symbol_map.insert(Keycode::Minus, "-".to_string());
    symbol_map.insert(Keycode::Equal, "=".to_string());
    symbol_map.insert(Keycode::BackSlash, "\\".to_string());
    symbol_map.insert(Keycode::Comma, ",".to_string());
    symbol_map.insert(Keycode::Dot, ".".to_string());
    symbol_map.insert(Keycode::Slash, "/".to_string());
    symbol_map.insert(Keycode::Grave, "`".to_string());

    let keys = device_state.get_keys();
    // let mut ctrl_keys: Vec<Keycode> = Vec::new();
    // let mut data_keys: Vec<Keycode> = Vec::new();
    let mut ctrl_keys: Vec<String> = vec![];
    let mut data_keys: Vec<String> =  vec![];

    let mut shift_pressed = false;

    for key in &keys {
        if shift_keycodes.contains(key) {
            shift_pressed = true;
            break;
        }
    }

    for key in &keys {
        if shift_keycodes.contains(key) {
            continue;
        }
        if ctrl_keycodes.contains(key) {
            ctrl_keys.push(key.to_string());
        } else if shift_pressed && letter_shift_map.contains_key(key) {
            data_keys.push(key.to_string());
        } else if shift_pressed && char_shift_map.contains_key(key) {
            data_keys.push(String::from(&char_shift_map[key]));
        } else if letter_shift_map.contains_key(key) {
            data_keys.push(String::from(&letter_shift_map[key]));
        } else if symbol_map.contains_key(key) {
            data_keys.push(String::from(&symbol_map[key]));
        } else {
            data_keys.push(key.to_string());
        }
    }

    let mut key_map: std::collections::HashMap <String, Vec<String>> = std::collections::HashMap::new();
    key_map.insert("ctrl".to_string(), ctrl_keys);

    key_map.insert("data".to_string(), data_keys);

    key_map
}
