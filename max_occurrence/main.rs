extern crate time;

use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};

use std::sync::Arc;

use std::io::{File,BufferedReader, IoResult};

static COUNT_THREAD_NUM: uint = 4u;

pub fn find_max_occurance(list : Vec<int>, count : uint) -> int {

    let mut start_time;
    let mut end_time;
    let mut delta;

    start_time = time::precise_time_ns();

    let list_arc = Arc::new(list);

    let input_num = count / COUNT_THREAD_NUM;
    let mut begin_idx = 0u;

    let threads = Vec::from_fn(COUNT_THREAD_NUM, |_| {
            let (tx, rx) = channel();
            let thread_list = list_arc.clone();

            spawn(proc() {
                // tx.send(use_hash_map(thread_list.as_slice(), count));
                tx.send(use_hash_map(thread_list.slice(begin_idx, begin_idx + input_num), input_num));
            });
            begin_idx += input_num;
            rx
        });

    let mut hash_maps: Vec<HashMap<int, uint>> = Vec::new();

    let mut init = false;
    let mut unified_counter: HashMap<int, uint> = HashMap::new();
    for rx in threads.iter() {
        if !init {
            unified_counter = rx.recv();
            init = true;
            continue;
        }
        hash_maps.push(rx.recv());
    }

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;
    println!("Make maps: {} sec", delta / 1000000000.0f32);

    start_time = time::precise_time_ns();

    for i in range(0u, hash_maps.len()) {
        let map = hash_maps.get(i);

        for (key, value) in map.iter() {
            match unified_counter.entry(*key) {
                Vacant(entry) => entry.set(*value),
                Occupied(mut entry) => {
                    *entry.get_mut() += *value;
                    entry.into_mut()
                }
            };
        }
    }

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;
    println!("Unify maps: {} sec", delta / 1000000000.0f32);

    start_time = time::precise_time_ns();

    let mut number = std::int::MAX;
    let mut max_occurrence = 1u;
    for (num, occurrence) in unified_counter.iter() {
        if *occurrence < max_occurrence {
            continue;
        } else if *occurrence > max_occurrence {
            max_occurrence = *occurrence;
            number = *num;
        } else {
            if *num < number {
                number = *num;
            }
        }
    }

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;
    println!("Scan Final Map: {} sec", delta / 1000000000.0f32);

    number
}

fn use_hash_map(list : &[int], count : uint) -> HashMap<int, uint> {
    let mut counter: HashMap<int, uint> = HashMap::with_capacity(count / 2u);

    for num in list.iter() {
        match counter.entry(*num) {
            Vacant(entry) => entry.set(1u),
            Occupied(mut entry) => {
                *entry.get_mut() += 1;
                entry.into_mut()
            }
        };
    }
    counter
}


fn main() {
    let mut list : Vec<int> = vec![];
    let path = Path::new("GenRandomData");
    let mut result : IoResult<String>;
    let mut line : String;
    let mut i = 0u;

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

