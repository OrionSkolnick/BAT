pub mod init;
use std::collections::HashMap;

fn main() {
    use crate::init::input_userid;
    let mut guests: HashMap<u8, String> = HashMap::with_capacity(u8::MAX.into());
    
    let input_max = 5;
    let mut input_index = 0;

    while input_index < input_max {
        input_index += 1;
        if let Ok((user_id, user_description)) = input_userid(input_index, input_max) {
            if !guests.contains_key(&user_id) {
                guests.insert(user_id, user_description.clone());
            }
            else {
                println!("\nERROR:");
                println!("you are overwriting the description of a previous entry");
                println!("Description \"{}\" not written", user_description);
            }
        }
    }
}
