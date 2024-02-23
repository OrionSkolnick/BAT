use std::num::ParseIntError;
use std::fmt::Display;

#[derive(Debug)]
pub enum InputError {
    ParseError(ParseIntError),
    IOError(std::io::Error),
}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::ParseError(parse_int_error) => 
                write!(f, "{}", parse_int_error),
            InputError::IOError(io_error) => 
                write!(f, "{}", io_error),
        }
    }
}

pub fn input_userid (index: u8, last: u8) -> Result<(u8, String), InputError> {
    use std::io;

    let mut user_id_stdin = String::new();
    println!("Please enter a user id between 0 and 255 ({}/{})", index, last);
    let user_id_result = io::stdin().read_line(&mut user_id_stdin);
    let userid: u8;        

    match user_id_result {
        Ok(_) => (),
        Err(e) => return Err(InputError::IOError(e)),
    }

    let parsed_user_id_stdin = user_id_stdin.trim().parse();
    match parsed_user_id_stdin {
        Ok(n) => userid=n,
        Err(e) => return Err(InputError::ParseError(e)),
    }

    let mut user_description_stdin = String::new();
    println!("Please enter a description for the guest:");
    let user_description_result = io::stdin().read_line(&mut user_description_stdin);
    let description: String;
    match user_description_result {
        Ok(_) => description=String::from(user_description_stdin.trim()),
        Err(e) => return Err(InputError::IOError(e)),
    }

    return Ok((userid, description));
}
