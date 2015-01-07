extern crate time;

use std::io::{File,BufferedReader, IoResult};

pub fn find_max_occurance(list : Vec<int>, count : int) -> int {
    // code here
    9999
}


fn main() {
    let mut list : Vec<int> = vec![];
    let path = Path::new("GenRandomData");
    let mut result : IoResult<String>;
    let mut line : String;
    let mut i = 0i;

    let file = File::open(&path);
    let mut reader = BufferedReader::new(file);
    result = reader.read_line();

    println!("Loding Data");
    loop {
        line = match result {
            Ok(s) => s,
                Err(_) => break,
        };

        match from_str(line.as_slice().trim()) {
            Some(x) => {
                list.push(x);
                i+=1;
        }
            None => ()
        }

        if i % 100 == 0 {
            print!(".");
        }

        result = reader.read_line();
    }
    println!("");
    println!("=========");
    println!("Total Items {}", i);

    let start_time = time::precise_time_ns();

    println!("Maximum Occurance Value is {}", find_max_occurance(list, i));

    let end_time = time::precise_time_ns();
    let delta = (end_time - start_time) as f32;

    println!("Elapsed Time {} nsec", delta);
    println!("Elapsed Time {} usec", delta / 1000.0f32);
    println!("Elapsed Time {} msec", delta / 1000000.0f32);
    println!("Elapsed Time {} sec", delta / 1000000000.0f32);
}

