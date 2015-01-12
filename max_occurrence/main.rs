extern crate time;

use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};

use std::rand;
use std::rand::{Rng, SeedableRng, StdRng};

use std::sync::Arc;
use std::sync::Future;

use std::io::{File,BufferedReader, IoResult};

static COUNT_THREAD_NUM: uint = 4u;

pub fn find_max_occurance_hashmap(list : Vec<int>, count : uint) -> int {
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

static GRAIN_SIZE: uint = 10u / 2;

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
    //         }
    //         None => ()
    //     }

    //     // if i % 100 == 0 {
    //     //     print!(".");
    //     // }

    //     result = reader.read_line();
    // }

    // let mut list : Vec<int> = vec![3, 2, 8, 6, 1, 7, 4, 5];
    // let mut list : Vec<int> = vec![3, 2, 8, 5, 6, 5, 7, 1, 5, 2, 3, 1, 8, 5, 7, 2];
    let mut list : Vec<int> = vec![];
    let mut list2 : Vec<int> = vec![];
    let i = 100000000i;
    // let i = 16u;

    // list = range(0i, i as int).rev().collect();

    let div = i as uint / 10u;

    let seed: &[_] = &[12345];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    for _ in range(0u, i as uint) {
        // list.push((rand::random::<uint>() % i) as int);
        list.push((rng.gen::<uint>() % div) as int);
    }

    list2 = list.clone();

    // for i in list2.iter() {
    //     print!("{} ", i);
    // }
    // println!("");

    println!("");
    println!("=========");
    println!("Total Items {}", i);

    let mut start_time = time::precise_time_ns();

    println!("Maximum Occurance Value is {}", use_parallel_sort(list.as_mut_slice(), i));

    let mut end_time = time::precise_time_ns();
    let mut delta = (end_time - start_time) as f32;

    // println!("Elapsed Time {} nsec", delta);
    // println!("Elapsed Time {} usec", delta / 1000.0f32);
    // println!("Elapsed Time {} msec", delta / 1000000.0f32);
    println!("Elapsed Time {} sec", delta / 1000000000.0f32);

    println!("");
    println!("=========");
    println!("Total Items {}", i);

    start_time = time::precise_time_ns();

    println!("Maximum Occurance Value is {}", find_max_occurance(list2, i));

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;

    // println!("Elapsed Time {} nsec", delta);
    // println!("Elapsed Time {} usec", delta / 1000.0f32);
    // println!("Elapsed Time {} msec", delta / 1000000.0f32);
    println!("Elapsed Time {} sec", delta / 1000000000.0f32);


    // for i in list2.iter() {
    //     print!("{} ", i);
    // }
    // println!("");
}

fn quick_and_merge(list : &mut [int], count : int) -> int {
    let mut start_time = time::precise_time_ns();
    let mut end_time;
    let mut delta;

    let thread_num = 4i;
    let list_size = count / thread_num;
    let mut begin_idx = 0i;

    let mut merge_buf: Vec<int> = Vec::new();

    {
        let (a, b):(int, int) = unsafe { std::mem::transmute(list.as_slice()) };

        let threads = Vec::from_fn(thread_num as uint, |_| {
            let (tx, rx) = channel();
            let borrowed: &mut [int] = unsafe { std::mem::transmute((a, b)) };
            spawn(proc() {
                quick_sort(borrowed, begin_idx, begin_idx + list_size - 1i);
                tx.send(true)
            });
            begin_idx += list_size;
            rx
        });

        end_time = time::precise_time_ns();
        delta = (end_time - start_time) as f32;
        println!("Sort thread trigger {} sec", delta / 1000000000.0f32);

        start_time = time::precise_time_ns();

        merge_buf = Vec::from_elem(count as uint, 0);

        end_time = time::precise_time_ns();
        delta = (end_time - start_time) as f32;
        println!("Mem alloc time {} sec", delta / 1000000000.0f32);

        for rx in threads.iter() {
            rx.recv();
        }

        end_time = time::precise_time_ns();
        delta = (end_time - start_time) as f32;
        println!("Sort Recv waiting {} sec", delta / 1000000000.0f32);
    }

    //==================================================

    start_time = time::precise_time_ns();

    let (a, b):(int, int) = unsafe { std::mem::transmute(list.as_slice()) };
    let (i, j):(int, int) = unsafe { std::mem::transmute(merge_buf.as_slice()) };
    let mut future_left = Future::spawn(proc() {
            let borrowed_list: &mut [int] = unsafe { std::mem::transmute((a, b)) };
            let borrowed_buf: &mut [int] = unsafe { std::mem::transmute((i, j)) };
            merge_to_buf(borrowed_list, 0i, count / 2i - 1i, borrowed_buf, 0i);
        });

    let (c, d):(int, int) = unsafe { std::mem::transmute(list.as_slice()) };
    let (x, y):(int, int) = unsafe { std::mem::transmute(merge_buf.as_slice()) };
    let mut future_right = Future::spawn(proc() {
            let borrowed_list: &mut [int] = unsafe { std::mem::transmute((c, d)) };
            let borrowed_buf: &mut [int] = unsafe { std::mem::transmute((x, y)) };
            merge_to_buf(borrowed_list, count / 2i, count - 1i, borrowed_buf, count / 2i);
        });

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;
    println!("Merge thread trigger {} sec", delta / 1000000000.0f32);

    start_time = time::precise_time_ns();

    future_left.get();
    future_right.get();

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;
    println!("Merge Recv waiting {} sec", delta / 1000000000.0f32);

    //==================================================

    // for i in range(0u, list.len()) {
    //     print!("{} ", list[i]);
    // }
    // println!("");

    scan_max_occurrence_with_two_bufs(merge_buf.as_slice(), count)
}

fn merge_to_buf(list: &[int], p: int, r: int, buf: &mut [int], start_idx: int) {
    let mut left = p as uint;
    let mut right = p as uint + ((r as uint - p as uint + 1u) / 2u);
    let mut iter = start_idx as uint;

    let left_end = right;
    let right_end = r as uint + 1u;

    // println!("left: {} / right: {} / left_end: {} / right_end: {}",
    //     left, right, left_end, right_end);

    while left != left_end && right != right_end {
        // println!("list[left]: {} / list[right]: {}", list[left], list[right]);
        if list[left] < list[right] {
            buf[iter] = list[left];
            left += 1;
        } else if list[left] > list[right] {
            buf[iter] = list[right];
            right += 1;
        } else {
            buf[iter] = list[left];
            iter += 1u;
            buf[iter] = list[left];
            left += 1;
            right += 1;
        }
        iter += 1u;

        // for i in buf.iter() {
        //     print!("{} ", i);
        // }
        // println!("/ iter: {}", iter);
    }

    if left != left_end {
        for i in range(left, left_end) {
            buf[iter] = list[i];
            iter += 1u;
        }
    } else if right != right_end {
        for i in range(right, right_end) {
            buf[iter] = list[i];
            iter += 1u;
        }
    }

    // print!("result: ");
    // for i in buf.iter() {
    //     print!("{} ", i);
    // }
    // println!("");
}

fn scan_max_occurrence(list: &[int], count : int) -> int {
    let mut current_num = list[0];
    let mut current_count = 1u;
    let mut max_num = std::int::MAX;
    let mut max_count = 0u;
    for i in range(1u, count as uint) {
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

    max_num
}

fn scan_max_occurrence_with_two_bufs(list: &[int], count : int) -> int {
    let mut max_num = 0i;
    let mut max_count = 0u;

    let mut left_idx = 0u;
    let mut right_idx = count as uint / 2u;

    let mut left = list[left_idx];
    let mut right = list[right_idx];

    let left_end = count as uint / 2u;
    let right_end = count as uint;

    let mut current_num;
    let mut current_count;

    // println!("scan_max_occurrence_with_two_bufs START");


    while left_idx != left_end && right_idx != right_end {
        // println!("left: {} - {} / right: {} - {}", left_idx, left, right_idx, right);
        left = list[left_idx];
        right = list[right_idx];

        if left < right {
            current_num = left;
            current_count = 1u;
            loop {
                left_idx += 1u;
                if left_idx == left_end || current_num != list[left_idx] {
                    break;
                }
                current_count += 1u;
            }
        } else if left > right {
            current_num = right;
            current_count = 1u;
            loop {
                right_idx += 1u;
                if right_idx == right_end || current_num != list[right_idx] {
                    break;
                }
                current_count += 1u;
            }

        } else {
            current_num = left;
            current_count = 1u;
            loop {
                left_idx += 1u;
                if left_idx == left_end || current_num != list[left_idx] {
                    break;
                }
                current_count += 1u;
            }
            loop {
                right_idx += 1u;
                if right_idx == right_end || current_num != list[right_idx]  {
                    break;
                }
                current_count += 1u;
            }
        }
        if current_count > max_count {
            max_count = current_count;
            max_num = current_num;
        }
    }

    max_num
}

fn use_rust_sort(list: &mut [int], count : int) -> int {
    let mut start_time;
    let mut end_time;
    let mut delta;

    start_time = time::precise_time_ns();
    list.sort();
    end_time = time::precise_time_ns();

    delta = (end_time - start_time) as f32;
    println!("Sorting: {} sec", delta / 1000000000.0f32);

    start_time = time::precise_time_ns();
    let max_num = scan_max_occurrence(list, count);
    end_time = time::precise_time_ns();

    delta = (end_time - start_time) as f32;
    println!("Scan max occurrence: {} sec", delta / 1000000000.0f32);

    max_num
}

fn use_quick_sort(list: &mut [int], count : int) -> int {
    let mut start_time = time::precise_time_ns();

    quick_sort(list, 0, count - 1i);

    let mut end_time = time::precise_time_ns();
    let mut delta = (end_time - start_time) as f32;
    println!("Sort time: {} sec", delta / 1000000000.0f32);

    start_time = time::precise_time_ns();

    let max_num = scan_max_occurrence(list, count);

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;
    println!("Scan max occurrence: {} sec", delta / 1000000000.0f32);

    max_num
}

fn use_quick_sort_with_tasks(list: &mut [int], count : int) -> int {
    let mut start_time = time::precise_time_ns();

    quick_sort_with_tasks(list, 0, count as int - 1i, count / 10i);

    let mut end_time = time::precise_time_ns();
    let mut delta = (end_time - start_time) as f32;
    println!("Sort time: {} sec", delta / 1000000000.0f32);

    start_time = time::precise_time_ns();

    let max_num = scan_max_occurrence(list, count);

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;
    println!("Scan max occurrence: {} sec", delta / 1000000000.0f32);

    max_num
}

fn partition(list: &mut [int], p : int, r: int) -> int {

    let pivot_value = list[r as uint];
    let pivot = r as uint;
    // if (r - p as int) > 100 {
    //     quick_sort(list, p, p as int + 8i);
    //     pivot_value = list[p + 4u];
    //     pivot = p + 4u;
    // } else {
        // pivot_value = list[r];
        // pivot = r;
    // }

    let mut i = p as uint - 1u;

    for j in range(p as uint, r as uint) {
        if list[j] < pivot_value {
            i += 1u;
            swap(list, i, j);
        }
    }
    i += 1u;
    swap(list, i, pivot);

    i as int
}

fn quick_sort(list: &mut [int], p : int, r: int) {
    if p < r {
        let q = partition(list, p, r);
        quick_sort(list, p, q - 1i);
        quick_sort(list, q + 1i, r);
    }
}

fn quick_sort_with_tasks(list: &mut [int], p : int, r: int, grain_size: int) {

    let q = partition(list, p, r);

    let left = q - p;
    let right = r - q;

    let left_task = left > grain_size;
    let right_task = right > grain_size;

    // println!("{} sec >>> {} ~ {} / {} ---> left: {} {}/ right: {} {}", delta/ 1000000000.0f32, p, r, r - p as int + 1,
    //     left, left_task, right, right_task);

    let (chan1, port1) = channel();
    let (chan2, port2) = channel();

    if left_task {
        let (a, b):(int, int) = unsafe { std::mem::transmute(list.as_slice()) };

        spawn(proc() {
            let borrowed: &mut [int] = unsafe { std::mem::transmute((a, b)) };
            quick_sort_with_tasks(borrowed, p, q - 1i, grain_size);
            chan1.send(true);
        });
    }

    if right_task {
        let (a, b):(int, int) = unsafe { std::mem::transmute(list.as_slice()) };

        Future::spawn(proc() {
            let borrowed: &mut [int] = unsafe { std::mem::transmute((a, b)) };
            quick_sort_with_tasks(borrowed, q + 1i, r, grain_size);
            chan2.send(true);
        });
    }

    if !left_task {
        // start_time = time::precise_time_ns();
        quick_sort(list, p, q - 1i);
        // end_time = time::precise_time_ns();
        // delta = (end_time - start_time) as f32;
        // println!("{} sec >>> sort!: q - p", delta / 1000000000.0f32);
    }

    if !right_task {
        // start_time = time::precise_time_ns();
        quick_sort(list, q + 1i, r);
        // end_time = time::precise_time_ns();
        // delta = (end_time - start_time) as f32;
        // println!("{} sec >>> sort!: q - p", delta / 1000000000.0f32);
    }

    if left_task {
        port1.recv();
    }

    if right_task {
        port2.recv();
    }

    // println!("{} ~ {} / {} ---> left: {} {}/ right: {} {}", p, r, r - p as int + 1,
    //     left, left_task, right, right_task);
}








fn parallel_sort(list : &mut [int], size: int, grain_size: int, full_size: int) {
    // println!("parallel #{} BEGIN - {}", id, size);
    if size <= grain_size {
        // println!("parallel #{} ---> quick sort", id);
        // list.sort();
        quick_sort(list, 0, size as int - 1i);
    } else {
        let mut merge_buf: Vec<int> = Vec::new();
        // let thread_list = list.slice(0, size / 2);
        let (a, b):(int, int) = unsafe { std::mem::transmute(list.slice(0, size as uint / 2u)) };

        let mut future = Future::spawn(proc() {
            let borrowed: &mut [int] = unsafe { std::mem::transmute((a, b)) };
            parallel_sort(borrowed, size / 2, grain_size, full_size)
        });

        parallel_sort(list.slice_mut(size as uint / 2u, size as uint), size - (size / 2), grain_size, full_size);

        if size != full_size {
            merge_buf = Vec::with_capacity(size as uint);
        }

        future.get();

        if size != full_size {
            merge_buf = list.to_vec();
            merge_to_buf(merge_buf.as_slice(), 0i, size - 1i, list.as_mut_slice(), 0i);
        }

        // print!("<after> merge_to_buf: ");
        // for i in range(0u, list.len()) {
        //     print!("{} ", list[i]);
        // }
        // println!("");
    }
    // println!("parallel #{} END", id);
}

fn swap(data: &mut [int], i: uint, j: uint) {
    if i != j {
        let tmp = data[i];
        data[i] = data[j];
        data[j] = tmp;
    }
}

fn use_parallel_sort(list: &mut [int], count : int) -> int {
    let mut start_time;
    let mut end_time;
    let mut delta;

    start_time = time::precise_time_ns();
    parallel_sort(list, count, count / 8i, count);
    end_time = time::precise_time_ns();

    delta = (end_time - start_time) as f32;
    println!("Sorting: {} sec", delta / 1000000000.0f32);

    start_time = time::precise_time_ns();
    // let max_num = scan_max_occurrence(list, count);
    let max_num = scan_max_occurrence_with_two_bufs(list, count);
    end_time = time::precise_time_ns();

    delta = (end_time - start_time) as f32;
    println!("Scan max occurrence: {} sec", delta / 1000000000.0f32);

    max_num
}










static THREAD_NUM: uint = 4u;

pub fn find_max_occurance(list : Vec<int>, count : int) -> int {

    let mut start_time = time::precise_time_ns();

    let count_buf: Vec<int> = Vec::from_elem(count as uint / 10u, 0);

    let mut end_time = time::precise_time_ns();
    let mut delta = (end_time - start_time) as f32;
    println!("Alloc Time {} sec", delta / 1000000000.0f32);

    start_time = time::precise_time_ns();

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

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;
    println!("Count Time {} sec", delta / 1000000000.0f32);

    start_time = time::precise_time_ns();

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

            tx.send(scan_occurrence(buf, begin_idx, input_num));
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

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;
    println!("Scan Time {} sec", delta / 1000000000.0f32);

    max_num as int
}

fn count_occurrence(input: &[int], counter: &mut [int], begin: uint, size: uint) {
    for i in range(begin, begin + size) {
        counter[input[i] as uint] += 1;
    }
}

fn scan_occurrence(buf: &[int], begin: uint, size: uint) -> (int, int) {
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
