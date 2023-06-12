fn main() {
    let message = "Hello, world!".to_string();
    println!("{}", message);

    primitive_data_types();
    non_copyable_example();
    copy_clone_traits_example();
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

fn non_copyable_example() {
    let x = String::from("Owned Value");
    let y = x;

    // println!("{}", x); - not allowed because owernship was transfer to y
    // println!("{}", y.clone();  or set y = x.clone() - possible fix but means duplicating memory
    println!("{}", &y); // better fix, borrow the value of y, it'll be returned when println! returns
    println!("{}", y);
    
}

// here we add Copy/Clone traits without which the below b2 = b1 would move b1, now it's copying; 
// copy has been disabled due to the addition of a String, can only include Copy trait if all elements support it
// comment out clone due to the addition of manual implementation of Clone, otherwise it'd be generated
#[derive(Debug/* , Clone */ /*, Copy*/)] 
struct Book {
    id: u32,
    publish_year: u32,
    title: String
}

impl Clone for Book {
    fn clone(&self) -> Self {
        println!("Cloning {:?}", self);

        // still not overly fond of this return style implied by the lack of ;
        Book {
            id: self.id,
            publish_year: self.publish_year,
            title: self.title.clone(),
        }
    }
}

fn copy_clone_traits_example() {
    let b1 = Book {
        id: 1,
        publish_year: 1990,
        title: "Jurassic Park".to_string(),
    };

    let mut b2 = b1.clone();
    b2.id = 2;
    b2.publish_year = 1992;
    b2.title = "Rising Sun".to_string();

    println!("{:?}", b1); 
    println!("{:?}", b2);
}