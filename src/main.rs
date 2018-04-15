use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use std::collections::HashMap;
use std::char;



fn main() {
    let path = Path::new("input");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Unable to open file {} : {}", display, why.description()),
        Ok(file) => file,
    };

    let mut buf = [0u8; 20];
    let bytes_read = file.read(&mut buf).unwrap();
    println!("b read: {}", bytes_read);
    //println!("{}", str::from_utf8(&buf).unwrap());

    //initialize dictionary with single character values
    let mut dictionary = HashMap::new();
    for x in 0..255 {
        dictionary.insert(vec![x],x as u16);
    }

    let mut current: Vec<u8> = Vec::new();
    for x in 0..bytes_read {
        let mut c = buf[x];
        current.push(c);

        if dictionary.contains_key(&current) == false {
            let n = dictionary.len() + 1;
            dictionary.insert(current.clone(), n as u16);
        }
    }
}
