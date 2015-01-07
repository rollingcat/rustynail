extern crate time;

use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};

use std::rand;

use std::sync::Arc;
use std::sync::Future;

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
                tx.send(make_hash_map(thread_list.slice(begin_idx, begin_idx + input_num), input_num));
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

fn make_hash_map(list : &[int], count : uint) -> HashMap<int, uint> {
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
    // let mut list : Vec<int> = vec![];
    // let path = Path::new("GenRandomData");
    // let mut result : IoResult<String>;
    // let mut line : String;
    // let mut i = 0u;

    // let file = File::open(&path);
    // let mut reader = BufferedReader::new(file);
    // result = reader.read_line();

    // println!("Loding Data");
    // loop {
    //     line = match result {
    //         Ok(s) => s,
    //             Err(_) => break,
    //     };

    //     match from_str(line.as_slice().trim()) {
    //         Some(x) => {
    //             list.push(x);
    //             i+=1;
    //     }
    //         None => ()
    //     }

    //     if i % 100 == 0 {
    //         print!(".");
    //     }

    //     result = reader.read_line();
    // }

    let mut list : Vec<int> = vec![];
    let mut list2 : Vec<int> = vec![];
    let mut i = 5000000u;

    list = range(0i, i as int).rev().collect();

    for _ in range(0u, i) {
        list.push((rand::random::<uint>() % i) as int);
    }

    list2 = list.clone();

    i += i;

    println!("");
    println!("=========");
    println!("Total Items {}", i);

    let mut start_time = time::precise_time_ns();

    println!("Maximum Occurance Value is {}", use_rust_sort(list.as_mut_slice(), i));

    let mut end_time = time::precise_time_ns();
    let mut delta = (end_time - start_time) as f32;

    println!("Elapsed Time {} nsec", delta);
    println!("Elapsed Time {} usec", delta / 1000.0f32);
    println!("Elapsed Time {} msec", delta / 1000000.0f32);
    println!("Elapsed Time {} sec", delta / 1000000000.0f32);

    println!("");
    println!("=========");
    println!("Total Items {}", i);

    start_time = time::precise_time_ns();

    println!("Maximum Occurance Value is {}", use_parallel_sort(list2.as_mut_slice(), i));

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;

    println!("Elapsed Time {} nsec", delta);
    println!("Elapsed Time {} usec", delta / 1000.0f32);
    println!("Elapsed Time {} msec", delta / 1000000.0f32);
    println!("Elapsed Time {} sec", delta / 1000000000.0f32);
}

fn use_rust_sort(list : &mut [int], count : uint) -> int {
    let mut start_time;
    let mut end_time;
    let mut delta;

    start_time = time::precise_time_ns();
    list.sort();
    end_time = time::precise_time_ns();

    delta = (end_time - start_time) as f32;
    println!("Sorting: {} sec", delta / 1000000000.0f32);

    start_time = time::precise_time_ns();

    let mut current_num = list[0];
    let mut current_count = 1u;
    let mut max_num = std::int::MAX;
    let mut max_count = 0u;
    for i in range(1u, count) {
        if current_num == list[i] {
            current_count += 1u;
        } else {
            if current_count > max_count {
                max_num = current_num;
                max_count = current_count;
            }
            current_num = list[i];
            current_count = 1u;
        }
    }

    end_time = time::precise_time_ns();

    delta = (end_time - start_time) as f32;
    println!("Scan max occurrence: {} sec", delta / 1000000000.0f32);

    max_num
}

// template<class T>
// void parallel_sort(T* data, int len, int grainsize)
// {
//     if(len < grainsize) // Use grainsize instead of thread count so that we don't e.g. spawn 4 threads just to sort 8 elements.
//     {
//         std::sort(data, data + len, std::less<T>());
//     }
//     else
//     {
//         auto future = std::async(parallel_sort<T>, data, len/2, grainsize);

//         parallel_sort(data + len/2, len/2, grainsize); // No need to spawn another thread just to block the calling thread which would do nothing.

//         future.wait();

//         std::inplace_merge(data, data + len/2, data + len, std::less<T>());
//     }
// }

static GRAIN_SIZE: uint = 10000u;

fn parallel_sort(data : &mut [int], size : uint) {
    if size < GRAIN_SIZE {
        data.sort();
    } else {
        let mut future = Future::spawn(proc() {parallel_sort(data.slice(0, size / 2), size / 2)});

        parallel_sort(data.slice(size / 2, size), size - (size / 2));

        future.get();

        merge(data, 0, size / 2, size - 1);
    }
}

fn swap(a: &mut int, b: &mut int) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn merge(data: &mut [int], l: uint, m: uint, r: uint) {
    let mut i = 0u;
    let mut j = 0u;
    let mut k = l;

    let n1 = m - l + 1;
    let n2 = r - m;

    loop {
        if (i < n1) && (j < n2) {
            break;
        }

        if data[l+i] <= data[m+1+j] {
            swap(&mut data[l+i], &mut data[k]);
            i += 1u;     
        } else {
            swap(&mut data[m+1+j], &mut data[k]);
            j += 1u;
        }
        k += 1u;
    }
}

fn use_parallel_sort(list: &'static mut [int], count : uint) -> int {
    let mut start_time;
    let mut end_time;
    let mut delta;

    start_time = time::precise_time_ns();
    parallel_sort(list, count);
    end_time = time::precise_time_ns();

    delta = (end_time - start_time) as f32;
    println!("Sorting: {} sec", delta / 1000000000.0f32);


    start_time = time::precise_time_ns();

    let mut current_num = list[0];
    let mut current_count = 1u;
    let mut max_num = std::int::MAX;
    let mut max_count = 0u;
    for i in range(1u, count) {
        if current_num == list[i] {
            current_count += 1u;
        } else {
            if current_count > max_count {
                max_num = current_num;
                max_count = current_count;
            }
            current_num = list[i];
            current_count = 1u;
        }
    }

    end_time = time::precise_time_ns();

    delta = (end_time - start_time) as f32;
    println!("Scan max occurrence: {} sec", delta / 1000000000.0f32);

    max_num

}
