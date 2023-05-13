use crate::examples::{hashmap_example, primitive_examples, string_example, vector_mut_example};

mod examples;

fn main() {
    vector_mut_example();
    string_example();
    hashmap_example();
    primitive_examples();
}
