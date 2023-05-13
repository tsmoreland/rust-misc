use crate::examples::{hashmap_example, primitive_examples, string_example, vector_example};
use crate::vec_examples::{vec_general_example, vec_init_example};

mod examples;
mod vec_examples;

fn main() {
    vector_example();
    string_example();
    hashmap_example();
    primitive_examples();

    println!("--- vec examples ---");

    vec_init_example();
    vec_general_example();
}
