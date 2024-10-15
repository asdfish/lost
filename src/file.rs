use std::{
    fs::File,
    io::{
        BufReader,
        Read,
    }
};

pub fn file_contents(file_path: &str) -> Option<String> {
    let file: File = File::open(file_path).ok()?;
    let mut buffer_reader = BufReader::new(file);
    let mut contents: String = String::new();

    buffer_reader.read_to_string(&mut contents).ok()?;

    return Some(contents);
}
