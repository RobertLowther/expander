mod keymanager;
use keymanager::{send_input, vk_to_string, VK_WHITESPACE};
use winput::{Action, Input, Vk};
use winput::message_loop;
use winput::message_loop::EventReceiver;
fn main() {
    let receiver: EventReceiver = message_loop::start().unwrap();
    let mut input_string = String::new();
    let mut input_keys: Vec<Vk> = vec![];
    let mut is_shift: bool = false;
    let mut is_caps: bool = false;

    loop {
        let mut key: Option<Vk> = None;

        match receiver.next_event() {
            // Key Down
            message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                    // is shift
                    if vk == Vk::Shift {
                        if is_caps {
                            is_shift = false;
                        } else {
                            is_shift = true;
                        }
                    }
                    // is capslock
                    else if vk == Vk::CapsLock {
                        is_caps = !is_caps;
                        is_shift = !is_shift;
                    }
                    else if VK_WHITESPACE.contains(&vk) {
                        input_string = String::new();
                        input_keys = vec![];
                    }
                    // is F2
                    else if vk == Vk::F2 {
                        send_input(&[
                            Input::from_vk(Vk::Shift, Action::Press),
                            Input::from_vk(Vk::B, Action::Press),
                            Input::from_vk(Vk::C, Action::Press),
                            Input::from_vk(Vk::Shift, Action::Release),
                            Input::from_vk(Vk::D, Action::Press),
                        ]);
                    }
                    // is character
                    else {
                        key = Some(vk);
                    }
                },

            // Key Up
            message_loop::Event::Keyboard {
                vk,
                action: Action::Release,
                ..
            } => {
                    // is shift
                    if vk == Vk::Shift {
                        if is_caps {
                            is_shift = true;
                        } else {
                            is_shift = false;
                        }
                    }
                }, 
            _ => (),
        }

        match key {
            Some(vk) => {
                input_string.push_str(&vk_to_string!(&vk, is_shift));
                input_keys.push(vk);
                println!("\"{}\"", input_string);
                println!("{:?}", input_keys);

                // check if input_string matches a pattern and if so
                // then replace text and reset input_string
            },
            None => ()
        }
    }
}
