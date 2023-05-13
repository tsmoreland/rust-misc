use crate::examples::{hashmap_example, primitive_examples, string_example, vector_example};
use crate::linkedlist_examples::list_example;
use crate::vec_examples::{vec_general_example, vec_init_example, vecdeque_example};

mod examples;
mod vec_examples;
mod linkedlist_examples;

fn main() {
    vector_example();
    string_example();
    hashmap_example();
    primitive_examples();

    println!("--- vec examples ---");

    vec_init_example();
    vec_general_example();

    println!("--- vecdeque examples ---");
    vecdeque_example();

    println!("--- linked list ---");
    list_example();
}
