use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::str;
use std::io::{Read, Write};
use std::collections::HashMap;

extern crate time;
use time::PreciseTime;

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

    //read file into memory
    let mut file_input_buffer: Vec<u8> = Vec::new();
    let mut bytes_read : usize = 0;
    read_file_to_memory("input", &mut file_input_buffer, &mut bytes_read);
    let mut compressed_data_buffer: Vec<u16> = Vec::new();

    //LZW Encoder
    {
        let start = PreciseTime::now();
        println!("beginning compression");

        //initialize dictionary with single character values
        let mut dictionary: HashMap<Vec<u8>, u16> = HashMap::new();
        for x in 0..256 {
            dictionary.insert(vec![x as u8],x);
        }

        //represents 'current string'
        let mut current: Vec<u8> = Vec::new();
        //represents our output
        for x in 0..bytes_read {
            let mut c = file_input_buffer[x as usize];
            current.push(c);

            if dictionary.contains_key(&current) == false {
                //find next code number
                //add current substring to dictionary
                let n = dictionary.len() + 1;
                dictionary.insert(current.clone(), n as u16);
                current.pop();
                //push to output
                compressed_data_buffer.push(dictionary[&current]);
                current = Vec::new();
                current.push(c);
            }
        }
        compressed_data_buffer.push(dictionary[&current]);
        let end = PreciseTime::now();
        let duration = start.to(end);

        //println!("{:?}", output);
        println!("compressed output bytes: {}", compressed_data_buffer.len()*2);
        let compression_ratio: f32 = 2.0*compressed_data_buffer.len() as f32/bytes_read as f32 * 100.0;
        println!("Compression ratio: {}%", compression_ratio);
        println!("Compression speed: {} kbps", (compressed_data_buffer.len() as f32*2.0)/(duration.num_milliseconds()as f32*0.001));
    }

    //LZW Decoder
    {
        let mut dictionary: HashMap<u16, Vec<u8>> = HashMap::new();
        for x in 0..256 {
            dictionary.insert(x,vec![x as u8]);
        }

        let mut decoded_output: Vec<u8> = Vec::new();
        let mut previous: Vec<u8> = Vec::new();
        let mut code: u16;
        let mut next_code: u16 = dictionary.len() as u16 + 1;

        println!("dictionary length: {}", dictionary.len());

        for x in 0..compressed_data_buffer.len() {
            code = compressed_data_buffer[x].clone();

            if (dictionary.contains_key(&code) == false) {
                let mut temp = previous.clone();
                temp.push(previous[0]);
                dictionary.insert(code, temp);
            }
            println!("index: {}, code: {}, next_code: {}",x,code,next_code);
            decoded_output.append(&mut dictionary[&(code)].clone());

            if &previous.len() > &(0 as usize) {
                let mut combined_string: Vec<u8> = previous.clone();
                combined_string.push(dictionary[&(code)][0]);
                dictionary.insert(next_code, combined_string);
                next_code+=1;
            }
            previous = dictionary[&code].clone();
        }

        println!("{:?}",decoded_output);
        println!("{:?}", str::from_utf8(decoded_output.as_slice()).unwrap());
    }
}
