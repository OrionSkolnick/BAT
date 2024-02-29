use std::num::ParseIntError;
use std::fmt::Display;
use std::convert::From;
use std::io;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum InputError {
    ParseError(ParseIntError),
    IOError(io::Error),
    InvalidInput(String),
    EmptyInput(String),
}

impl From<ParseIntError> for InputError {
    fn from(e: ParseIntError) -> InputError {
        InputError::ParseError(e)
    }
}

impl From<io::Error> for InputError {
    fn from(e: io::Error) -> InputError {
        InputError::IOError(e)
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::ParseError(parse_int_error) => write!(f, "{}", parse_int_error),
            InputError::IOError(io_error) => write!(f, "{}", io_error),
            InputError::InvalidInput(invalid_err) => write!(f, "{}", invalid_err),
            InputError::EmptyInput(name) => write!(f, "{} was empty", name),
        }
    }
}

pub struct GuestInfo { //TODO: add record of where they stayed previous nights
    pub name: String,
    pub pronouns: String,
    pub breathalize: bool,
    pub age: u8, //TODO: change age int to be a DOB from which you can calculate age
    pub preferred_bunks: Option<VecDeque<u8>>,
    pub notes: Option<String>,
    pub preferred_room: String
}

pub fn input_userid () -> Result<(u32, GuestInfo), InputError> {

    fn get_bool_input(prompt: String) -> Result<bool, InputError> {

        let mut str = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut str)?;

        if let Some(yn) = match str.trim().chars().nth(0) {
            Some(char) => {
                match char {
                    'Y' | 'y' => Some(true),
                    'N' | 'n' => Some(false),
                    _ => None,
                }
            }
            _ => None
        }{
            Ok(yn)
        } else {
            return Err(InputError::InvalidInput(String::from("input was not y or n")));
        }
    }

    let mut id_stdin = String::with_capacity(6);
    println!("Please put in a User ID: ");
    io::stdin().read_line(&mut id_stdin)?;

    let trimmed_id_stdin = id_stdin.trim();
    if trimmed_id_stdin.len() != 5 { //throws error if string length is wrong
        return Err(InputError::InvalidInput(String::from("user id was not length 5 (ex ji210)")));
    }
    let fullid = u32::from_str_radix(&trimmed_id_stdin, 36)?; //TODO: make this use bit math
    let numid: u32 = (fullid>>16).try_into().unwrap();
    let _iv1: u32 = fullid-(numid<<16);

    let mut name_stdin = String::new();
    println!("Please enter the full name of the guest:");
    io::stdin().read_line(&mut name_stdin)?;
    let name=String::from(name_stdin.trim());
    if name.is_empty() {
        return Err(InputError::EmptyInput(String::from("name")));
    }

    let mut pronouns_stdin = String::new();
    println!("Please enter the pronouns of the guest (ex: he/him/his):");
    io::stdin().read_line(&mut pronouns_stdin)?;
    let pronouns=String::from(pronouns_stdin.trim().to_lowercase());
    if pronouns.split('/').count() != 3 {
        return Err(InputError::InvalidInput(String::from("pronouns weren't formatted correctly! you MUST use forward slash and have 3 pronouns! (ex he/him/his)")));
    }

    let breath = get_bool_input(String::from("Do they need to be breathalized? (y/n): "))?;

    let mut age_stdin = String::with_capacity(4);
    println!("How many years old is the guest:");
    io::stdin().read_line(&mut age_stdin)?;
    let age = age_stdin.trim().parse()?;

    let bunks: Option<VecDeque<u8>>;
    if get_bool_input(String::from("Do they have a preferred bunk(s)? (y/n): "))? {

        let mut bunk_amount_stdin = String::with_capacity(4);
        println!("How many prefered bunks does {name} have: ");
        io::stdin().read_line(&mut bunk_amount_stdin)?;
        let bunk_amount: u8 = bunk_amount_stdin.trim().parse()?;
        let mut bunks_vecdeque: VecDeque<u8> = VecDeque::with_capacity(bunk_amount.into());
        
        for index in 0..bunk_amount {
            let mut bunk_stdin = String::with_capacity(4);
            println!("Favored bunk #{} (ex 239): ", index+1);
            io::stdin().read_line(&mut bunk_stdin)?;
            bunks_vecdeque.push_front(bunk_stdin.trim().parse()?);
        }
        bunks = Some(bunks_vecdeque);
    } else {
        bunks = None;
    }

    let mut note_stdin = String::new();
    println!("Enter any notes for the user to be stored (press ENTER to skip): ");
    io::stdin().read_line(&mut note_stdin)?;
    let note=String::from(note_stdin.trim());

    let mut room_stdin = String::new();
    println!("What room will the name be staying in (ex MALE or room 2): ");
    io::stdin().read_line(&mut room_stdin)?;
    let room=String::from(room_stdin.trim());
    if room.is_empty() {
        return Err(InputError::EmptyInput(String::from("room")));
    }

    return Ok((
            fullid,
            GuestInfo{
                name: name,
                pronouns: pronouns,
                breathalize: breath,
                age: age,
                preferred_bunks:bunks,
                notes: match note.is_empty() {
                    true => None,
                    false => Some(note),
                },
                preferred_room: room}
        )
    );
}
