use crate::imports::*;

pub fn open_file(input: &str) -> File
{
    match File::open(input)
    {
        Err(e) => panic!("Could not open file: {}", e),
        Ok(val) => val
    }
}

pub fn read_string(input: &str) -> String
{
    let mut string = String::new();
    match open_file(input).read_to_string(&mut string)
    {
        Err(e) => println!("Error while reading: {}", e),
        _ => {}
    }
    string
}

pub fn read_bytes(input: &str) -> Vec<u8>
{
    let mut vec: Vec<u8> = vec![];
    match open_file(input).read_to_end(&mut vec)
    {
        Err(e) => println!("Error while reading: {}", e),
        _ => {}
    }
    vec
}
