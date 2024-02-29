pub mod input;

fn main() { //TODO: fix dogshit temp main function lol
    use input::input_userid;

    let mut i: u8 = 0;

    loop {
        i+=1;
        println!("ADDING USER#{i}:\n");
        let result = input_userid();
        match result {
            Ok(((id, _iv), stats)) => {
                println!("{} (id {id}) is {} years old", stats.name, stats.age);
                println!("{}'s pronouns are {}", stats.name, stats.pronouns);
                if stats.breathalize {
                    println!("{} must be breathalized", stats.name);
                }
                if let Some(mut bunks) = stats.preferred_bunks {
                    println!("Most preferred bunk is: Bunk {}", bunks.pop_front().unwrap());
                }
                println!("Note: {}", match stats.notes {
                    Some(note) => note,
                    None => String::from("N/A"),
                });
            }
            Err(e) => {
                i-=1;
                println!("ERROR: {}", e);
            }
        }
    }
}
