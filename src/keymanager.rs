use winput::{Input, Vk};

//pub fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

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
        Vk::F1 => "F1".to_string(),
        Vk::F2 => "F2".to_string(),
        Vk::F3 => "F3".to_string(),
        Vk::F4 => "F4".to_string(),
        Vk::F5 => "F5".to_string(),
        Vk::F6 => "F6".to_string(),
        Vk::F7 => "F7".to_string(),
        Vk::F8 => "F8".to_string(),
        Vk::F9 => "F9".to_string(),
        Vk::F10 => "F10".to_string(),
        Vk::F11 => "F11".to_string(),
        Vk::F12 => "F12".to_string(),
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
    return winput::send_inputs(output);
}
