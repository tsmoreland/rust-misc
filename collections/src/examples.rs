use std::collections::HashMap;

pub fn vector_mut_example() {

    let mut my_vec = vec![1, 2, 3];
    my_vec.push(4);

    print!("vector: [");
    for item in my_vec.iter() {
        print!("{}, ", item);
    }
    println!("]");
}

pub fn string_example() {
    let hello_world:String = String::from("Hello World");
    println!("string: {}", hello_world)
}

pub fn hashmap_example() {
    let mut map = HashMap::new();
    map.insert("key", 10);

    for (key, value) in &map  {
        println!("{key} = {value}");
    }
}

pub fn primitive_examples() {
    let tuple = (1, 2, "three");
    println!("{} {} {}", tuple.0, tuple.1, tuple.2);

    let array:[u8; 4] = [1, 2, 3, 4];
    print!("[");
    for item in array {
       print!("{},", item);
    }
    println!("]");

    let slice = &array[1..3]; // like a view or span
    print!("[");
    for item in slice {
        print!("{},", item);
    }
    println!("]");
}