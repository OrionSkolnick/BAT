pub mod input;

fn main() { //TODO: fix dogshit temp main function lol
    use input::input_userid;
    let mut i: u8 = 0;
    loop {
        i+=1;
        println!("ADDING USER#{i}:\n");
        if let Ok((_id, stats)) = input_userid(){
            println!("{} is {} years old", stats.name, stats.age);
            println!("{}'s pronouns are {}", stats.name, stats.pronouns);
            if stats.breathalize {
                println!("{} must be breathalized", stats.name);
            }
            println!("Note: {}", match stats.notes {
                Some(note) => note,
                None => String::from("N/A"),
            });
        } else {println!("THERE WAS AN ERROR");i-=1;}
    }
}
