use std::collections::VecDeque;

pub fn vec_init_example() {
    let alpha = vec![0, 1, 2, 3, 4]; // initialize with values
    let beta = vec![0; 6]; // initialize with 0 in 6 places
    let mut charlie = Vec::new();
    charlie.push(0);
    let delta:Vec<i32> = Vec::with_capacity(5);
    let echo = Vec::from([0, 1, 2]);

    println!("a: {:?} b: {:?} c: {:?} d: {:?} e: {:?}", alpha, beta, charlie, delta, echo);
}

pub fn vec_general_example() {
    let mut alpha = vec![0, 1, 2, 3, 4]; // initialize with values
    alpha[1] = 42; // panic if index isn't there
    let index0 = alpha[0];
    println!("alpha: {:?} index0 = {}", alpha, index0);

    // preferred means of indexing, safer
    let maybe_index0:Option<&i32> = alpha.get(0);
    match maybe_index0 {
        Some(x) => println!("(Option Example) index0 = {x}"),
        None => println!("no value at index0"),
    }

    let maybe_mut_index1 = alpha.get_mut(1);
    match maybe_mut_index1 {
        Some(x) => println!("(Option example) index1 = {x}"),
        None => println!("no value at index0"),
    }

    alpha.push(5);
    println!("alpha push: {:?}", alpha);
    let length = alpha.len();
    println!("alpha len: {:?} length: {}", alpha, length);
    alpha.append(&mut vec![6, 7]);
    println!("alpha append: {:?}", alpha);
    alpha.clear();
    println!("alpha clear: {:?}", alpha);

    alpha.append(&mut vec![0, 1, 2, 3, 4, 5, 6]);
    println!("alpha append: {:?}", alpha);
    alpha.drain(5..);
    println!("alpha drain: {:?}", alpha);

    alpha.insert(1, 10);
    println!("alpha insert: {:?}", alpha);
    alpha.remove(1);
    println!("alpha remove: {:?}", alpha);

    alpha.retain(|&n| n % 2 == 0);
    println!("alpha retain: {:?}", alpha);

    alpha.retain_mut(|n| if *n < 10 {
        *n += 2;
        true
    } else {
        false
    });
    println!("alpha retain_mut: {:?}", alpha);

    alpha.truncate(2);
    println!("alpha truncate: {:?}", alpha);
}

pub fn vecdeque_example() {
    let mut new_deque:VecDeque<u32> = VecDeque::new();
    let mut cap_deque:VecDeque<u32> = VecDeque::with_capacity(5);
    let from_deque = VecDeque::from([0, 1, 2]);

    new_deque.push_front(2);
    new_deque.push_front(1);
    new_deque.push_front(0);
    cap_deque.push_back(0);
    cap_deque.push_back(1);
    cap_deque.push_back(2);
    println!("new: {:?} cap: {:?} from: {:?}", new_deque, cap_deque, from_deque);

    new_deque.pop_front();
    cap_deque.pop_back();
    println!("new: {:?} cap: {:?} from: {:?}", new_deque, cap_deque, from_deque);

    let one = 1;
    if new_deque.contains(&one) {
        println!("new contains 1");
    }

    let new_front = new_deque.front();
    //let new_mut_front = new_deque.front_mut();
    match new_front {
        Some(x) => println!("new_front = {x}"),
        None => {},
    }

    let cap_back = new_deque.back();
    match cap_back {
        Some(x) => println!("cap_back = {x}"),
        None => {},
    }
    let cap_mut_back = new_deque.back_mut();
    match cap_mut_back {
        Some(x) => println!("cap_mut_back = {x}"),
        None => {},
    }

}