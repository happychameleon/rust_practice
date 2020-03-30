// here i'll add some comments not much changed here

/* here is a block comment, wow this is crAZY fun
ha my comment just goes on and on 
and on and on */

/// okay im told this is a doc comment and will be used for the html docs, very cool
fn main() {
    println!("Hello, world!");
    println!("I am person of this world!");
    //formatted();
    //debug();
    //display();
    //formatting();
    //primitives();
    //lit_op();
    tuples();
}


/// Exercise 1.2 formatted print https://doc.rust-lang.org/stable/rust-by-example/hello/print.html
fn formatted(){
    // as i understand it the {} when using the println! macro is replaced by any argument that 
    // followes it and that argument is stringified (made to a string)
    // if i don't use a suffix for 12 it will become a i32,
    // by adding a suffix it will become whatever 
    // i say it is
    println!("so here is the argument, {}", 12);
    
    // another cool thing that can be done
    println!("{0}, this is {1}. {1}, this is {0}. Lets not talk out in the open, {2} might be listening.", "Alice", "Bob", "Eve" );
    
    // just dirrectly copying the example from `rust-by-example`,
    // apperently one can name arguments and 
    // have them print which is super cool!
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");
        
    // in a {} one can specify speciall formating after a `:`
    println!("{} of {:b} people know binary, the other half doesen't", 1, 2);
    
    // I can right-align text with a specified with. this will use spaces to right-align
    println!("{number:>width$}", number=1, width=21);
    
    // I can right-align text with a specified with. this will use 0s to right-align
    println!("{number:>0width$}", number=1, width=21);
    
    // fixed example
    println!("My name is {0}, {1} {0}", "Bond", "James");
    
    // printing the number of decimal places of pi by defining it in the println! macro
    let pi = 3.142592;
    println!("here is some pi with a precision of {prec}: {number:.prec$}", prec=5, number=pi);
}

/// Exercise 1.2.1 Debug
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person <'a> {
    name: &'a str,
    age: u8
}

fn debug(){
    // the {:?} is a speciall debug formater that works very much
    // like the normal print formater
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's");
    
    // thanks to the derive attribut this struct is printable
    println!("Now {:?} will print!", Structure(3));
    
    // drive however does not permitt you to control how the output looks.
    println!("Now {:?} will print!", Deep(Structure(7)));
    
    // rust does give one the option for "pretty printing"
    // the formater {:#?}
    let name = "peter";
    let age = 27;
    let peter = Person { name, age};
    
    // pretty print
    println!("{:#?}", peter);
    
    // this is what it would look like unpretty printed
    println!("{:?}", peter);
}

/// Exercise 1.2.2 Display and Exercise 1.2.2.1 Testcase: List

// the advantage of Display over Debug is that it can look cleaner
// and more compact. We do this by manually implementing fmt::Display

// here we are importing the fmt module
use std::fmt;

struct StructureTwo(i32);

// here we will implment the fmt trait for the Structure struct
impl fmt::Display for StructureTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y:{}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    r: f64,
    c: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.r, self.c)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;
        
        write!(f, "[")?;
        
        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for(count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 { write!(f, ", ")?;}
            write!(f, "{}: {}", count, v)?;
        }
        
        //close the opened bracket and return a fmt::Result value
        write!(f, "]")
    }
}

fn display(){
    println!("{}", StructureTwo(4));
    
    let minmax = MinMax(0, 14);
    
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Dispaly: {:?}", minmax);
    
    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    
    println!("The bigg range is {big} and the small is {small}",
        small = small_range,
        big = big_range);
    
    let point = Point2D {x: 3.3, y: 7.2};
    
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    
    let complex = Complex {r: 3.3, c: 7.2};
    println!("Compare complex");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
    
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

/// Exercise 1.2.3 Formatting

use std::fmt::{Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this memthod must write the formamted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        
        write!(f, "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue)
    }
}

fn formatting() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
    }
}

/// 2. Primitivies

fn primitives() {
    // Variables can be type annotated.
    let logical: bool = true;
    
    let a_float: f64 = 1.0; // Regualr annotation
    let an_integer = 5i32; // Suffix annotation
    
    // Or a default will be used.
    let default_float = 3.0; // `f64`
    let default_intager = 7; // `i32`
    
    // A type can also be inferred from context
    let mut inferred_type =  12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a varaible can't be changed.
    //mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;
}

