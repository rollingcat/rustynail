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

static GRAIN_SIZE: uint = 10000000u / 2;

fn main() {
    // CODE ---------------- 1
    // let i_vec = vec![1i, 2, 3, 4 ,5, 6];
    // let i_vec_p = i_vec.slice(1, 4); // partially copy
    // let (a, b):(int, int) = unsafe { std::mem::transmute(i_vec_p) };
    // let (tx, rx) = channel();
    // spawn(proc() {
    //     let borrowed: &mut [int] = unsafe { std::mem::transmute((a, b)) };
    //     println!("borrowed = {}", borrowed);
    //     borrowed[0] = 10;
    //     tx.send(());
    // });
    // rx.recv();
    // println!("i_vec = {}", i_vec);
    // println!("i_vec_p = {}", i_vec_p);
    // ------------------------

    // CODE ---------------- 2
    // let v = range(0i, 10).collect: Vec<_>>();
    // let v_arc = Arc::new(RWLock::new(v));

    // for idx in range(0i, 3) {
    //     let v_arc_clone = v_arc.clone();
    //     spawn(move || {
    //         {
    //             let mut w = v_arc_clone.deref().write();
    //             (*w)[idx as uint] = (*w)[idx as uint] + 1;
    //         }
    //         {
    //             let r = v_arc_clone.deref().read();
    //             println!("task share and modify the value : {}", *r);
    //         }
    //     });
    // }
    // ------------------------

    // http://doc.rust-lang.org/std/sync/atomic/struct.AtomicInt.html

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

        // if i % 100 == 0 {
        //     print!(".");
        // }

        result = reader.read_line();
    }

    // let mut list : Vec<int> = vec![];
    let mut list2 : Vec<int> = vec![];
    // let mut i = 5000000u;

    // list = range(0i, i as int).rev().collect();

    // for _ in range(0u, i) {
    //     list.push((rand::random::<uint>() % i) as int);
    // }

    list2 = list.clone();

    // i += i;

    // println!("");
    // println!("=========");
    // println!("Total Items {}", i);

    let mut start_time = time::precise_time_ns();

    // println!("Maximum Occurance Value is {}", use_rust_sort(list.as_mut_slice(), i));

    let mut end_time = time::precise_time_ns();
    let mut delta = (end_time - start_time) as f32;

    // println!("Elapsed Time {} nsec", delta);
    // println!("Elapsed Time {} usec", delta / 1000.0f32);
    // println!("Elapsed Time {} msec", delta / 1000000.0f32);
    // println!("Elapsed Time {} sec", delta / 1000000000.0f32);

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

fn parallel_sort(list : &mut [int], size : uint, id: uint) {
    println!("parallel #{} BEGIN", id);
    if size < GRAIN_SIZE {
        println!("parallel #{} ---> quick sort", id);
        list.sort();
    } else {
        // let thread_list = list.slice(0, size / 2);
        let (a, b):(int, int) = unsafe { std::mem::transmute(list.slice(0, size / 2)) };

        let mut future = Future::spawn(proc() {
            let borrowed: &mut [int] = unsafe { std::mem::transmute((a, b)) };
            parallel_sort(borrowed, size / 2, id + 10)
        });

        parallel_sort(list.slice_mut(size / 2, size), size - (size / 2), id + 1);

        future.get();

        merge(list, 0, size / 2, size - 1);
    }
    println!("parallel #{} END", id);
}

fn rust_sort(data: &mut [int]) -> bool {
    data.sort();
    true
}

fn swap(data: &mut [int], i: uint, j: uint) {
    let tmp = data[i];
    data[i] = data[j];
    data[j] = tmp;
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
            swap(data, l+i, k);
            i += 1u;
        } else {
            swap(data, m+1+j, k);
            j += 1u;
        }
        k += 1u;
    }
}

fn use_parallel_sort(list: &mut [int], count : uint) -> int {
    let mut start_time;
    let mut end_time;
    let mut delta;

    start_time = time::precise_time_ns();
    parallel_sort(list, count, 0);
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
