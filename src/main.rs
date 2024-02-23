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
                println!("you are overwriting the description of a previous entry on ID {}", user_id);
                println!("Description \"{}\" not written", user_description);
            }
        }
        else {
            println!("something went wrong teehee"); //FIXME: assumes input_userid does not return error FIX THIS
        }
    }
    loop { //FIXME: this is hacky placeholder code lol
        use std::io;
        let mut input = String::new();
        println!("\nPlease enter the user id you would like to view the description of:");
        let _ = io::stdin().read_line(&mut input);
        let user_id = input.trim().parse::<u8>().unwrap();
        if let Some(description) = guests.get(&user_id) {
            println!("{}\nDescription: {}", user_id, description);
        } else {
            println!("\nERROR: that user ID isn't in the database");
        }
    }
}
