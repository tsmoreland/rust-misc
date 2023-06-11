fn main() {
    let message = "Hello, world!".to_string();
    println!("{}", message);

    primitive_data_types()
}

// primitive_data_types lists the various primitives and via comments what the implicit types are, overly simple reference material
fn primitive_data_types() {

    // scalar integers: i8, i16, i32, i64, i128 and u8 ... u128 where u is unsigned integer and i is integer
    let x:i32 = 5;
    let y = 5; // implicit type is i32
    println!("{} {}", x, y);

    // scalar floats: f64
    let a:f64 = 5.0;
    let b = 5.0; // implicit type is f64
    println!("{} {}", a, b);

    // scalar bool and char
    let c:bool = true;
    let d:char = 'd';
    println!("{} {}", c, d);

    // Compound tuple
    let t: (i32, f64, bool) = (5, 5.0, true);

    // array of i32 with 5 items (static size, can't grow)
    let array:[i32; 5]  = [1, 2, 3, 4, 5];
    println!("{:?} {:?}, tuple access: {:?}", t, array, t.0);


}