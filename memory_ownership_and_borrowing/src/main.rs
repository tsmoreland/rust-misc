fn main() {
    let message = "Hello, world!".to_string();
    println!("{}", message);

    primitive_data_types();
    non_copyable_example();
    copy_clone_traits_example();

    immutable_borrowing();
    mutable_borrowing();

    lifetimes_in_fn_example();
    lifetimes_in_structs();
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

struct BookWithAuthor<'a> {
    id: u32,
    publish_year: u32,
    title: String,
    author: &'a str,
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

fn print_length(s: &String) {
    println!("{}, length = {}", s, s.len());
}

fn print_length_using_slice(s: &str) {
    println!("{}, length = {}", s, s.len());
}

fn immutable_borrowing() {
    let x = String::from("Hello, World!");
    let y: &String = &x;
    let z: &String = &x; // can have more than one immutable ref
    let w = &x[0..5];

    println!("{}", x);
    println!("{}", y);
    println!("{}", *y); // same as just y, it's not needed but is allowed
    print_length(z);
    print_length_using_slice(y);
    print_length_using_slice(w);
}

fn append(source: &mut String, value: &String) {
    source.push_str(value);
}

fn mutable_borrowing() {
    let mut x = "Hello ".to_string();
    let y = &mut x;
    let z = "World".to_string();
    append(y, &z);

    println!("{}", x);

    let mut a = 42;
    let b = &mut a;
    *b += 1; // need to dereference scalar / primivite type
    println!("{}", a);
}

fn lifetimes_in_fn_example() {
    /* -- demonstrates something the borrow checker would prevent
    let x:&String;    
    {
        let y = String::from("lifetime");
        // x = &y; -- error because lifetime of y ends with this scope, borrow checker safety checks during compile time and knows x would be an invalid ref
    }
    println!("{}", x);
    */

    let b1 = Book { id: 1, publish_year: 1990, title: "Jurassic Park".to_string() };
    let b2 = Book { id: 2, publish_year: 1992, title: "Rising Sun".to_string() };

    let b3 = get_oldest(&b1, &b2);

    //println!("{:?}", b1);
    //println!("{:?}", b2);
    println!("Oldest {:?}", b3);
}

// provide life time details via <'a> and then adding that to each reference
fn get_oldest<'a>(b1: &'a Book, b2: &'a Book) -> &'a Book {
    if b1.publish_year < b2.publish_year {
        b1
    } else {
        b2
    }
}

fn lifetimes_in_structs() {
    let authors = [ "Michael Crichton".to_string(), "Tom Clancy".to_string(), "Lee Child".to_string() ];

    let b1 = BookWithAuthor { id: 1, publish_year: 1990, title: "Jurassic Park".to_string(), author: &authors[0] }; 
    let b2 = BookWithAuthor { id: 2, publish_year: 1997, title: "Killing Floor".to_string(), author: &authors[2] }; 

    {
        let other_authors = [ "Lee Child".to_string() ];
        //b2.author = &other_authors[0]; // not allowed because life time of other_authors is shorter than b2, assuming b2 was mutable as well which it isn't any more
        _ = other_authors;
    }

    println!("{}: {} by {} published {}", b1.id, &b1.title, &b1.author, b1.publish_year);
    println!("{}: {} by {} published {}", b2.id, &b2.title, &b2.author, b2.publish_year);
}

