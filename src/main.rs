use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::str;
use std::io::{Read, Write};
use std::collections::HashMap;



fn main() {
    let path = Path::new("input");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Unable to open file {} : {}", display, why.description()),
        Ok(file) => file,
    };

    let mut buf = [0u8; 2048];
    let bytes_read = file.read(&mut buf).unwrap();
    println!("b read: {}", bytes_read);
    println!("{}", str::from_utf8(&buf).unwrap());


}
