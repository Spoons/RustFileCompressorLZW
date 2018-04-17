use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use std::collections::HashMap;

fn read_file_to_memory(filename: &str, buffer: &mut Vec<u8>, bytes_read: &mut usize) {
    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Unable to open file {} : {}", display, why.description()),
        Ok(file) => file,
    };

    //create 2mb static buffer
    let mut buf = [0u8; 2097152];

    loop {
        let read_status = file.read(&mut buf);
        match read_status {
            Ok(read_status) => {
                //hit eof
                if read_status == 0 {
                    println!("breaking due to oef");
                    break;
                }
                *bytes_read = read_status;
                buffer.extend_from_slice(&buf[..*bytes_read]);
            },
            Err(_) => {
                println!("Error occured during file read.");
            }
        }
    }
    println!("bytes read: {}", bytes_read);
}

fn main() {
    {
        let mut big_buf: Vec<u8> = Vec::new();
        let mut bytes_read : usize = 0;
        read_file_to_memory("input", &mut big_buf, &mut bytes_read);

        //initialize dictionary with single character values
        let mut dictionary = HashMap::new();
        for x in 0..255 {
            dictionary.insert(vec![x],x as u16);
        }

        //represents 'current string'
        let mut current: Vec<u8> = Vec::new();
        //represents our output
        let mut output: Vec<u16> = Vec::new();
        for x in 0..bytes_read {
            let mut c = big_buf[x as usize];
            current.push(c);

            if dictionary.contains_key(&current) == false {
                //find next code number
                //add current substring to dictionary
                let n = dictionary.len() + 1;
                dictionary.insert(current.clone(), n as u16);
                current.pop();
                //push to output
                output.push(dictionary[&current]);
                current = Vec::new();
                current.push(c);
            }
        }
        output.push(dictionary[&current]);

        //println!("{:?}", output);
        println!("compressed output bytes: {}", output.len()*2);
        let compression_ratio: f32 = 2.0*output.len() as f32/bytes_read as f32 * 100.0;
        println!("Compression ratio: {}%", compression_ratio);
    }
}
