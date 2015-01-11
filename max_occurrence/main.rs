extern crate time;

use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};

use std::rand;
use std::rand::{Rng, SeedableRng, StdRng};

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
    let mut list : Vec<int> = vec![];
    let mut list2 : Vec<int> = vec![];
    let i = 10000000u;
    // let i = 8u;

    // list = range(0i, i as int).rev().collect();

    let seed: &[_] = &[7];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    for _ in range(0u, i) {
        // list.push((rand::random::<uint>() % i) as int);
        list.push((rng.gen::<uint>() % i) as int);
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

    println!("Maximum Occurance Value is {}", use_quick_sort(list.as_mut_slice(), i));

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

    println!("Maximum Occurance Value is {}", use_quick_sort(list2.as_mut_slice(), i));

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;

    println!("Elapsed Time {} nsec", delta);
    println!("Elapsed Time {} usec", delta / 1000.0f32);
    println!("Elapsed Time {} msec", delta / 1000000.0f32);
    println!("Elapsed Time {} sec", delta / 1000000000.0f32);


    // for i in list2.iter() {
    //     print!("{} ", i);
    // }
    // println!("");
}

fn quick_and_merge(list : &mut [int], count : uint) -> int {
    let mut start_time = time::precise_time_ns();
    let mut end_time;
    let mut delta;
    
    let thread_num = 4u;
    let list_size = count / thread_num;
    let mut begin_idx = 0u;

    let mut merge_left: Vec<int> = Vec::new();
    let mut merge_right: Vec<int> = Vec::new();
    {
        let (a, b):(int, int) = unsafe { std::mem::transmute(list.as_slice()) };

        let threads = Vec::from_fn(thread_num, |_| {
            let (tx, rx) = channel();
            let borrowed: &mut [int] = unsafe { std::mem::transmute((a, b)) };
            spawn(proc() {
                quick_sort(borrowed, begin_idx, (begin_idx + list_size - 1u) as int);
                tx.send(true)
            });
            begin_idx += list_size;
            rx
        });

        end_time = time::precise_time_ns();
        delta = (end_time - start_time) as f32;
        println!("Sort thread trigger {} sec", delta / 1000000000.0f32);

        start_time = time::precise_time_ns();

        merge_left = Vec::from_elem(count / 2u, 0);
        merge_right = Vec::from_elem(count / 2u, 0);

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
    let (i, j):(int, int) = unsafe { std::mem::transmute(merge_left.as_slice()) };   
    let mut future_left = Future::spawn(proc() {
            let borrowed_list: &mut [int] = unsafe { std::mem::transmute((a, b)) };      
            let borrowed_buf: &mut [int] = unsafe { std::mem::transmute((i, j)) };      
            merge_to_buf(borrowed_list, borrowed_buf, 0u, count / 2u - 1u);
        });

    let (c, d):(int, int) = unsafe { std::mem::transmute(list.as_slice()) };
    let (x, y):(int, int) = unsafe { std::mem::transmute(merge_right.as_slice()) };
    let mut future_right = Future::spawn(proc() {
            let borrowed_list: &mut [int] = unsafe { std::mem::transmute((c, d)) };      
            let borrowed_buf: &mut [int] = unsafe { std::mem::transmute((x, y)) };      
            merge_to_buf(borrowed_list, borrowed_buf, count / 2u, count - 1u);
        });

    // merge_to_buf(list.as_slice(), &mut merge_left, 0, count - 1u);

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;
    println!("Merge thread trigger {} sec", delta / 1000000000.0f32);

    start_time = time::precise_time_ns();

    future_left.get();
    future_right.get();

    end_time = time::precise_time_ns();
    delta = (end_time - start_time) as f32;
    println!("Merge Recv waiting {} sec", delta / 1000000000.0f32);

    9999    
}

fn merge_to_buf(list: &[int], buf: &mut [int], p: uint, r: uint) {
    let mut left = p;
    let mut right = p + ((r - p + 1u) / 2u);
    let mut iter = 0u;

    let left_end = right;
    let right_end = r + 1u;

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
        // iter += 1u;
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

fn use_quick_sort(list: &mut [int], count : uint) -> int {
    let start_time = time::precise_time_ns();
    quick_sort(list, 0, count as int - 1i);
    let end_time = time::precise_time_ns();

    let delta = (end_time - start_time) as f32;
    println!("Sort time: {} sec", delta / 1000000000.0f32);

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

    max_num
}

fn use_quick_sort_with_tasks(list: &mut [int], count : uint) -> int {
    quick_sort_with_tasks(list, 0, count as int - 1i);

    // let r = count as int - 1i;
    // let q = partition(list, 0, r);

    // let (chan1, port1) = channel();
    // let (chan2, port2) = channel();

    // println!("Start>>> left: {} / right: {}", q, count - q as uint);
    
    // let (a, b):(int, int) = unsafe { std::mem::transmute(list.as_slice()) };
    // spawn(proc() {        
    //     let borrowed: &mut [int] = unsafe { std::mem::transmute((a, b)) };
    //     quick_sort_with_tasks(borrowed, 0, q - 1i);
    //     chan1.send(true);
    // });

    // let (c, d):(int, int) = unsafe { std::mem::transmute(list.as_slice()) };
    // spawn(proc() {
    //     let borrowed: &mut [int] = unsafe { std::mem::transmute((c, d)) };
    //     quick_sort_with_tasks(borrowed, q as uint + 1u, r);
    //     chan2.send(true);
    // });

    // port1.recv();
    // port2.recv();
    
    let mut start_time = time::precise_time_ns();

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
    
    let mut end_time = time::precise_time_ns();
    let mut delta = (end_time - start_time) as f32;

    println!("Occurrence Count Time {} sec", delta / 1000000000.0f32);

    max_num
}

fn partition(list: &mut [int], p : uint, r: int) -> int {

    let pivot_value;
    let pivot;
    // if (r - p as int) > 100 {
    //     quick_sort(list, p, p as int + 8i);
    //     pivot_value = list[p + 4u];
    //     pivot = p + 4u;
    // } else {
        pivot_value = list[r as uint];
        pivot = r as uint;
    // }

    let mut i = p - 1u;

    for j in range(p, r as uint) {
        if list[j] < pivot_value {
            i += 1u;
            swap(list, i, j);
        }
    }
    i += 1u;
    swap(list, i, pivot);

    i as int
}

fn quick_sort(list: &mut [int], p : uint, r: int) {
    if (p as int) < r {
        let q = partition(list, p, r);
        quick_sort(list, p, q - 1i);
        quick_sort(list, q as uint + 1u, r);
    }
}

fn quick_sort_with_tasks(list: &mut [int], p : uint, r: int) {

    let (chan1, port1) = channel();
    let (chan2, port2) = channel();

    let mut start_time = time::precise_time_ns();
    let q = partition(list, p, r);
    let mut end_time = time::precise_time_ns();

    let mut delta = (end_time - start_time) as f32;    

    let left = q - p as int;
    let right = r - q;

    let left_task = left > 1000000;
    let right_task = right > 1000000;

    // println!("{} sec >>> {} ~ {} / {} ---> left: {} {}/ right: {} {}", delta/ 1000000000.0f32, p, r, r - p as int + 1, 
    //     left, left_task, right, right_task);

    if left_task {
        let (a, b):(int, int) = unsafe { std::mem::transmute(list.as_slice()) };

        spawn(proc() {
            let borrowed: &mut [int] = unsafe { std::mem::transmute((a, b)) };
            quick_sort_with_tasks(borrowed, p, q - 1i);
            chan1.send(true);
        });                
    }

    if right_task {
        let (a, b):(int, int) = unsafe { std::mem::transmute(list.as_slice()) };

        Future::spawn(proc() {
            let borrowed: &mut [int] = unsafe { std::mem::transmute((a, b)) };
            quick_sort_with_tasks(borrowed, q as uint + 1u, r);
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
        quick_sort(list, q as uint + 1u, r);
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








fn parallel_sort(list : &mut [int], size : uint, id: uint) {
    println!("parallel #{} BEGIN - {}", id, size);
    if size <= GRAIN_SIZE {
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

        merge(list, 0, size / 2, size);
    }
    println!("parallel #{} END", id);
}

fn swap(data: &mut [int], i: uint, j: uint) {
    if i != j {} {
        let tmp = data[i];
        data[i] = data[j];
        data[j] = tmp;
    }
}

fn merge(data: &mut [int], l: uint, m: uint, r: uint) {
    let mut i = 0u;
    let mut j = 0u;
    let mut k = l;

    let n1 = m - l;
    let n2 = r - m;
    println!("n1: {} / n2: {}", n1, n2);

    loop {
        println!("----------------------");
        for i in range(l, r) {
            print!("{} ", data[i]);
        }
        println!("");
        println!("l: {} / m: {} / r: {}", l, m , r);
        

        if (i == n1) && (j == n2) {
            break;
        }

        if data[l+i] < data[m+j] {
            swap(data, l+i, k);
            i += 1u;
        } else {
            swap(data, m+j, k);
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
