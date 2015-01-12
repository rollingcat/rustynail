
extern crate time;

use std::io::{File,BufferedReader, IoResult};

static THREAD_NUM: uint = 4u;

// pub fn find_max_occurance(list: &[int], count : int) -> int {
pub fn find_max_occurance(list: &[int], count : int) -> int {
    let count_buf: Vec<int> = Vec::from_elem(count as uint / 10u, 0);

    let (a, b):(int, int) = unsafe { std::mem::transmute(count_buf.as_slice()) };
    let (i, j):(int, int) = unsafe { std::mem::transmute(list.as_slice()) };

    let mut begin_idx = 0u;
    let mut input_num = count as uint / THREAD_NUM;

    let counters = Vec::from_fn(THREAD_NUM, |_| {
        let (tx, rx) = channel();

        spawn(proc() {
            let counter : &mut [int] = unsafe { std::mem::transmute((a, b)) };
            let input : &[int] = unsafe { std::mem::transmute((i, j)) };

            tx.send(count_occurrence(input, counter, begin_idx, input_num));
        });
        begin_idx += input_num;
        rx
    });

    for rx in counters.iter() {
        rx.recv();
    }

    // let mut max_num = 0;
    // let mut max_count = 0;
    // for i in range(0, count_buf.len()) {
    //     if count_buf[i] > max_count {
    //         max_num = i;
    //         max_count = count_buf[i]
    //     }
    // }

    let (c, d):(int, int) = unsafe { std::mem::transmute(count_buf.as_slice()) };

    begin_idx = 0u;
    input_num = count_buf.len() / THREAD_NUM;

    let finders = Vec::from_fn(THREAD_NUM, |_| {
        let (tx, rx) = channel();
        spawn(proc() {
            let buf : &mut [int] = unsafe { std::mem::transmute((c, d)) };

            tx.send(scan_max_occurrence(buf, begin_idx, input_num));
        });
        begin_idx += input_num;
        rx
    });

    let mut max_num = 0;
    let mut max_count = 0;

    for rx in finders.iter() {
        let (num, count) = rx.recv();
        if count < max_count {
            continue;
        }

        if count > max_count {
            max_num = num;
            max_count = count;
            continue;
        }

        if num < max_num {
            max_num = num;
        }
    }

    max_num as int
}

fn count_occurrence(input: &[int], counter: &mut [int], begin: uint, size: uint) {
    for i in range(begin, begin + size) {
        counter[input[i] as uint] += 1;
    }
}

fn scan_max_occurrence(buf: &[int], begin: uint, size: uint) -> (int, int) {
    let mut max_num = 0;
    let mut max_count = 0;

    for i in range(begin, begin + size) {
        if buf[i] > max_count {
            max_num = i;
            max_count = buf[i]
        }
    }

    (max_num as int, max_count)
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

    println!("Maximum Occurance Value is {}", find_max_occurance(list.as_slice(), i));

    let end_time = time::precise_time_ns();
    let delta = (end_time - start_time) as f32;

    println!("Elapsed Time {} nsec", delta);
    println!("Elapsed Time {} usec", delta / 1000.0f32);
    println!("Elapsed Time {} msec", delta / 1000000.0f32);
    println!("Elapsed Time {} sec", delta / 1000000000.0f32);
}

