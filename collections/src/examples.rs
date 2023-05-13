use std::collections::HashMap;

pub fn vector_example() {

    let mut my_vec = vec![1, 2, 3];
    my_vec.push(4);

    print!("vector: [");
    for item in my_vec.iter() {
        print!("{}, ", item);
    }
    println!("]");

    let mut prime_numbers = vec![1, 3, 5, 7];
    let mut even_numbers = Vec::from([2, 4, 6, 8]);
    let odd_numbers:Vec<i32> = Vec::with_capacity(10);
    println!("odd numbers capacity: {:?}", odd_numbers.capacity());

    prime_numbers.push(11);
    println!("Prime Numbers: {:?}", prime_numbers);

    even_numbers.pop();
    println!("event numbers: {:?}", even_numbers)
}


pub fn string_example() {
    let hello_world:String = String::from("Hello World");
    println!("string: {}", hello_world);

    let mut pre_allocated_string = String::with_capacity(5);
    println!("pre_allocated_string capacity: {:?}", pre_allocated_string.capacity());

    for _ in 0..6 {
        pre_allocated_string.push('Z');
    }
    println!("String: {}", pre_allocated_string);
    println!("updated pre_allocated_string capacity: {:?}", pre_allocated_string.capacity());
}

pub fn hashmap_example() {
    let mut map = HashMap::new();
    map.insert("key", 10);

    for (key, value) in &map  {
        println!("{key} = {value}");
    }

    let mut string_map = HashMap::new();
    string_map.insert(1, "Hello");
    string_map.insert(2, "World");

    let coffee_map = HashMap::from([
        ("Espresso", 4.50),
        ("Original", 3.20),
        ("Dark Roast", 3.30),
    ]);
    println!("Coffee map {:?}", coffee_map);

    let pre_allocated_map:HashMap<i32, i32> = HashMap::with_capacity(10);
    println!("pre_allocated_map capacity: {:?}", pre_allocated_map.capacity());
}

pub fn primitive_examples() {
    let tuple = (1, 2, "three");
    println!("{} {} {}", tuple.0, tuple.1, tuple.2);

    let (one, two, three) = tuple;
    println!("{} {} {}", one, two, three);

    let mut array:[u8; 4] = [1, 2, 3, 4];
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

    let mut_slice = &mut array[2..3]; // like a view or span
    mut_slice[0] = 15;
    println!("Mut Slice: {:?}", mut_slice);
}