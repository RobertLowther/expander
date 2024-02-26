mod keylogger;
use std::io::Write;

fn main() {
    let mut prev_ctrl_keys: Vec<String> = Vec::new();
    let mut prev_data_keys: Vec<String> = Vec::new();
    const INITIAL_BUFFER_VALUE: Option<String> = None;
    let mut buffer: [Option<String>; 32] = [INITIAL_BUFFER_VALUE; 32];
    let mut buffer_next: usize = 0;

    let delimiters: [String; 3]= [
        String::from("Space"),
        String::from("Enter"),
        String::from("Tab"),
    ];
    
    loop {
        // Get all keys that were depressed this frame
        // and sort them by type
        let all_keys = keylogger::run();
        let curr_ctrl_keys = all_keys.get("ctrl").unwrap().to_vec();
        let curr_data_keys = all_keys.get("data").unwrap().to_vec();

        // Determine which keys are newly depressed and store them in vectors
        let ctrl_equal = curr_ctrl_keys == prev_ctrl_keys;
        let data_equal = curr_data_keys == prev_data_keys;

        let mut new_ctrl_keys: Vec<String> = Vec::new();
        let mut new_data_keys: Vec<String> = Vec::new();
        
        if !ctrl_equal && curr_ctrl_keys.len() > 0 {
            // print new characters
            new_ctrl_keys = get_new_keys(&prev_ctrl_keys, &curr_ctrl_keys);
        }

        if !data_equal && curr_data_keys.len() > 0 {
            // print new characters
            new_data_keys = get_new_keys(&prev_data_keys, &curr_data_keys);
        }

        prev_ctrl_keys = all_keys.get("ctrl").unwrap().to_vec();
        prev_data_keys = all_keys.get("data").unwrap().to_vec();
        std::io::stdout().flush().unwrap();
        
        // Add new keys to a buffer
        for key in &new_data_keys {
            if delimiters.contains(key){
                for index in 0..buffer.len() {
                    if buffer[index] == None { break }
                    buffer[index] = None;
                }
                buffer_next = 0;
                break;
            }
            
            buffer[buffer_next] = Some(key.to_string());
            buffer_next += 1;
            if buffer_next >= buffer.len() { buffer_next = 0 };
        }

        if &new_ctrl_keys.len() > &0 || &new_data_keys.len() > &0 {
            for key in &buffer {
                if let Some(value) = key {
                    print!("{}", value);
                } else {
                    print!("{}", String::from("_"));
                }
            }
            println!("");
            std::io::stdout().flush().unwrap();
            // println!("{:?}", buffer);
        }
    }
    // if end of buffer matches a patter - replace
}

fn get_new_keys(prev_keys: &Vec<String>, curr_keys: &Vec<String>) -> Vec<String> {
    let mut new_keys: Vec<String> = Vec::new();
    for key in curr_keys {
        if !prev_keys.contains(&key) {
            new_keys.push(String::from(key));
        }
    }
    new_keys
}
