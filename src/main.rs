use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::str;
use std::io::{Read, Write};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::env;

extern crate time;
use time::PreciseTime;

fn read_file_to_memory(filename: &str, buffer: &mut Vec<u8>, bytes_read: &mut usize) {
    //escape characters from filename
    let path = Path::new(&filename);
    let display = path.display();

    //try to open file
    let mut file = match File::open(&path) {
        Err(why) => panic!("Unable to open file {} : {}", display, why.description()),
        Ok(file) => file,
    };

    //create 2mb static buffer
    let mut buf = [0u8; 2097152];

    //set bytes_read pointer to zero
    *bytes_read = 0;
    loop {
        //this call reads from file and stores contents in buf
        let read_status:Result<_,_> = file.read(&mut buf);
        //here we check if the read was successful by matching
        //read_status enum with it's states Ok or Err
        match read_status {
            //If read_status is Ok then value contains the number
            //of bytes read
            Ok(value) => {
                //stream is empty so break
                if value == 0 {
                    println!("Breaking due to stream being empty.");
                    break;
                }
                //add num bytes of this read to bytes_read
                *bytes_read = value;
                //copy our 2mb buffer into our vector and try reading
                //from the strem again.
                buffer.extend_from_slice(&buf[..*bytes_read]);
            },
            //if there is an error print it
            Err(e) => {
                println!("Error occured during file read.\n{}",e);
            }
        }
    }
    println!("bytes read: {}", bytes_read);
}

fn write_vector_to_file(filename: &str, input: Vec<u8>) -> usize {
    let path = Path::new(filename);
    let display = path.display();
    let mut file = match OpenOptions::new().write(true).create(true).open(filename) {
        Err(e) => panic!("couldn't create {}: {}", display, e),
        Ok(file) => file,
    };
    match file.write_all(&input) {
        Err(e) => {
            panic!("couldn't write to {}: {}", display, e)
        },
        Ok(_) => println!("write successful")
    };
    return 0 as usize;
}

fn main() {

    let args: Vec<String> = env::args().collect();

    //parse args
    let mut filename = "input";
    if args.len() > 1 {
        filename = &args[1];
    }
    let print_decoded = false;
    //here we read the whole file into memory
    let mut file_input_buffer: Vec<u8> = Vec::new();
    //this is how many bytes were read during file read
    let mut bytes_read : usize = 0;
    //read file into a vector. store the size in bytes read.
    read_file_to_memory(filename, &mut file_input_buffer, &mut bytes_read);

    //this is where we are going to write our encoded output
    let mut compressed_data_buffer: Vec<u16> = Vec::new();

    //LZW Encoder
    //this is where encoding begins
    {
        let start = PreciseTime::now();
        println!("beginning compression");

        //initialize dictionary with single character values
        //we are initializing the first 255 'values' 0-255 to
        //to a matching 'code' with the same value
        let mut dictionary: HashMap<Vec<u8>, u16> = HashMap::new();
        for x in 0..256 {
            dictionary.insert(vec![x as u8],x);
        }

        //this is the current series of bytes we are trying to
        //convert into codes.
        let mut bytes_to_encode: Vec<u8> = Vec::new();
        //loop over each byte in the file
        for x in 0..bytes_read {
            //this is our iterator variable for the current value.
            let mut current_byte = file_input_buffer[x as usize];
            //add current character to string
            bytes_to_encode.push(current_byte);

            //if our "bytes_to_encode" string does not have a code in the dictionary
            //then we are going to  create a one
            if dictionary.contains_key(&bytes_to_encode) == false {
                //here we find the next code value
                let next_available_code = dictionary.len() + 1;
                //add our string to dictionary with new code
                dictionary.insert(bytes_to_encode.clone(), next_available_code as u16);
                //delete the last character from the current string
                bytes_to_encode.pop();
                //write our code to output
                compressed_data_buffer.push(dictionary[&bytes_to_encode]);
                //reset bytes to encode
                bytes_to_encode = Vec::new();
                bytes_to_encode.push(current_byte);
            }
        }
        //if we already have a code, use it.
        compressed_data_buffer.push(dictionary[&bytes_to_encode]);
        let end = PreciseTime::now();
        let duration = start.to(end);

        //println!("{:?}", output);
        println!("compression complete\ndictionary size: {}", dictionary.len());
        println!("compressed output bytes: {}", compressed_data_buffer.len()*2);
        let compression_ratio: f32 = 2.0*compressed_data_buffer.len() as f32/bytes_read as f32 * 100.0;
        println!("Compression ratio: {}%", compression_ratio);
        println!("Compression speed: {} kbps", (compressed_data_buffer.len() as f32*2.0)/(duration.num_milliseconds()as f32*0.001));
    }

    //LZW Decoder
    //begin decoding. we are going to use the compressed_data_buffer
    //from the encoding step.
    {
        //this is a new dictionary due to the scope change!!
        let mut dictionary: HashMap<u16, Vec<u8>> = HashMap::new();
        //insert the default values into dictionary
        for x in 0..256 {
            dictionary.insert(x,vec![x as u8]);
        }

        //buffer for decoded output
        let mut decoded_output: Vec<u8> = Vec::new();
        let mut previous: Vec<u8> = Vec::new();
        let mut code: u16;
        let mut next_code: u16 = dictionary.len() as u16 + 1;

        //println!("dictionary length: {}", dictionary.len());

        for x in 0..compressed_data_buffer.len() {
            code = compressed_data_buffer[x].clone();

            if dictionary.contains_key(&code) == false {
                let mut temp = previous.clone();
                temp.push(previous[0]);
                dictionary.insert(code, temp);
            }
            //println!("index: {}, code: {}, next_code: {}",x,code,next_code);
            decoded_output.append(&mut dictionary[&(code)].clone());

            if &previous.len() > &(0 as usize) {
                let mut combined_string: Vec<u8> = previous.clone();
                combined_string.push(dictionary[&(code)][0]);
                dictionary.insert(next_code, combined_string);
                //println!("{}",next_code);

                next_code+=1;
            }
            previous = dictionary[&code].clone();
        }

        //println!("{:?}",decoded_output);
        if (print_decoded) {
            println!("{:?}", str::from_utf8(decoded_output.as_slice()).unwrap());
        }
        println!("writing file");
        write_vector_to_file("output", decoded_output);
    }
}
