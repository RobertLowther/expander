use winput::{Input, Vk};

//pub fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

pub const VK_CHARACTERS: [Vk; 45] = [
    Vk::A,
    Vk::B,
    Vk::C,
    Vk::D,
    Vk::E,
    Vk::F,
    Vk::G,
    Vk::H,
    Vk::I,
    Vk::J,
    Vk::K,
    Vk::L,
    Vk::M,
    Vk::N,
    Vk::O,
    Vk::P,
    Vk::Q,
    Vk::R,
    Vk::S,
    Vk::T,
    Vk::U,
    Vk::V,
    Vk::W,
    Vk::X,
    Vk::Y,
    Vk::Z, 
    Vk::_1,
    Vk::_2,
    Vk::_3,
    Vk::_4,
    Vk::_5,
    Vk::_6,
    Vk::_7,
    Vk::_8,
    Vk::_9,
    Vk::_0,
    Vk::Minus,
    Vk::Plus,
    Vk::Oem1,
    Vk::Oem2,
    Vk::Oem3,
    Vk::Oem4,
    Vk::Oem5,
    Vk::Oem6,
    Vk::Oem7,
];

pub const VK_WHITESPACE: [Vk; 3] = [
    Vk::Space,
    Vk::Tab,
    Vk::Enter,
];

pub const VK_CONTROL: [Vk; 18] = [
    Vk::Alt,
    Vk::Control,
    Vk::CapsLock,
    Vk::LeftWin,
    Vk::RightWin,
    Vk::Insert,
    Vk::F1,
    Vk::F2,
    Vk::F3,
    Vk::F4,
    Vk::F5,
    Vk::F6,
    Vk::F7,
    Vk::F8,
    Vk::F9,
    Vk::F10,
    Vk::F11,
    Vk::F12,
];

pub const VK_NAVIGATION: [Vk; 8] = [
    Vk::UpArrow,
    Vk::DownArrow,
    Vk::RightArrow,
    Vk::LeftArrow,
    Vk::PageUp,
    Vk::PageDown,
    Vk::Home,
    Vk::End,
];

pub const VK_DELETEION: [Vk; 2] = [
    Vk::Delete,
    Vk::Backspace,
];

pub fn vk_to_string(vk: &Vk, shift: bool) -> String{
    match vk {
        Vk::A => if shift { "A".to_string() } else { "a".to_string() },
        Vk::B => if shift { "B".to_string() } else { "b".to_string() },
        Vk::C => if shift { "C".to_string() } else { "c".to_string() },
        Vk::D => if shift { "D".to_string() } else { "d".to_string() },
        Vk::E => if shift { "E".to_string() } else { "e".to_string() },
        Vk::F => if shift { "F".to_string() } else { "f".to_string() },
        Vk::G => if shift { "G".to_string() } else { "g".to_string() },
        Vk::H => if shift { "H".to_string() } else { "h".to_string() },
        Vk::I => if shift { "I".to_string() } else { "i".to_string() },
        Vk::J => if shift { "J".to_string() } else { "j".to_string() },
        Vk::K => if shift { "K".to_string() } else { "k".to_string() },
        Vk::L => if shift { "L".to_string() } else { "l".to_string() },
        Vk::M => if shift { "M".to_string() } else { "m".to_string() },
        Vk::N => if shift { "N".to_string() } else { "n".to_string() },
        Vk::O => if shift { "O".to_string() } else { "o".to_string() },
        Vk::P => if shift { "P".to_string() } else { "p".to_string() },
        Vk::Q => if shift { "Q".to_string() } else { "q".to_string() },
        Vk::R => if shift { "R".to_string() } else { "r".to_string() },
        Vk::S => if shift { "S".to_string() } else { "s".to_string() },
        Vk::T => if shift { "T".to_string() } else { "t".to_string() },
        Vk::U => if shift { "U".to_string() } else { "u".to_string() },
        Vk::V => if shift { "V".to_string() } else { "v".to_string() },
        Vk::W => if shift { "W".to_string() } else { "w".to_string() },
        Vk::X => if shift { "X".to_string() } else { "x".to_string() },
        Vk::Y => if shift { "Y".to_string() } else { "y".to_string() },
        Vk::Z => if shift { "Z".to_string() } else { "z".to_string() },
        Vk::_1 => if shift { "!".to_string() } else {  "1".to_string() },
        Vk::_2 => if shift { "@".to_string() } else {  "2".to_string() },
        Vk::_3 => if shift { "#".to_string() } else {  "3".to_string() },
        Vk::_4 => if shift { "$".to_string() } else {  "4".to_string() },
        Vk::_5 => if shift { "%".to_string() } else {  "5".to_string() },
        Vk::_6 => if shift { "^".to_string() } else {  "6".to_string() },
        Vk::_7 => if shift { "&".to_string() } else {  "7".to_string() },
        Vk::_8 => if shift { "*".to_string() } else {  "8".to_string() },
        Vk::_9 => if shift { "(".to_string() } else {  "9".to_string() },
        Vk::_0 => if shift { ")".to_string() } else {  "0".to_string() },
        Vk::Oem1 => if shift { ":".to_string() } else {  ";".to_string() },
        Vk::Oem2 => if shift { "?".to_string() } else {  "/".to_string() },
        Vk::Oem3 => if shift { "~".to_string() } else {  "`".to_string() },
        Vk::Oem4 => if shift { "{".to_string() } else {  "[".to_string() },
        Vk::Oem5 => if shift { "|".to_string() } else {  "\\".to_string() },
        Vk::Oem6 => if shift { "}".to_string() } else {  "]".to_string() },
        Vk::Oem7 => if shift { "\"".to_string() } else {  "'".to_string() },
        Vk::Oem8 => if shift { "Oem8".to_string() } else {  "Oem8".to_string() },
        Vk::Minus => if shift { "_".to_string() } else {  "-".to_string() },
        Vk::Plus => if shift { "+".to_string() } else {  "=".to_string() },
        Vk::Shift => "Shift".to_string(),
        Vk::Comma => if shift { "<".to_string() } else {  ",".to_string() },
        Vk::Period => if shift { ">".to_string() } else {  ".".to_string() },
        Vk::Numpad0 => "0".to_string(),
        Vk::Numpad1 => "1".to_string(),
        Vk::Numpad2 => "2".to_string(),
        Vk::Numpad3 => "3".to_string(),
        Vk::Numpad4 => "4".to_string(),
        Vk::Numpad5 => "5".to_string(),
        Vk::Numpad6 => "6".to_string(),
        Vk::Numpad7 => "7".to_string(),
        Vk::Numpad8 => "8".to_string(),
        Vk::Numpad9 => "9".to_string(),
        _ => "!".to_string(),
    }
}

#[macro_export]
macro_rules! vk_to_string {
    ($a: expr) => {
        vk_to_string($a, false)
    };
    ($a: expr, $b: expr) => {
        vk_to_string($a, $b)
    }
}

pub fn send_input(output: &[Input]) -> u32 {
    winput::send_inputs(output)
}
