#![allow(overflowing_literals)]

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
    //tuples();
    //arsli();
    //structures();
    //enums();
    //enumUse();
    //clike();
    //enum_testcase();
    //constants();
    //varbind();
    //mutability();
    //scope_shadow();
    //declare_first();
    //casting();
    //literals();
    //inference();
    //aliasing();
    //from_and_into();
    //tryfrom_tryinto();
    //to_from_strings();
    //flow_control();
    //functioned_fizzbuzz();
    //methods();
    //closures();
    //hof();
    diverg_func();
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

/// 2.1 Literals and operators

fn lit_op() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);
    
    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    
    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    
    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    
    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}

/// Exercise 2.2 Tuples

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to veriables
    let (integer, boolean) = pair;
    
    (boolean, integer)
}

// transpose swaps to elements of a 2x2 matrix
fn transpose(matrix: Matrix) -> Matrix {
    let a = matrix.0;
    let b = matrix.1;
    let c = matrix.2;
    let d = matrix.3;
    let matrix = Matrix(a, c, b, d);
    return matrix;
}

// The following struct is for the activiity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn tuples() {
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    
    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);
    
    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    
    // Tuples are printalbe
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    
    let pair = (1, true);
    println!("pair is {:?}", reverse(pair));
    
    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));
    
    // tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);
    
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    
    println!("Transpose:\n{}", transpose(matrix));
}

/// 2.3 Arrays and Slices

use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn arsli() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    
    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];
    
    // Indexing starts at 0
    println!("fist element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);
    
    // `len` returns the size of the array
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    
    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    
    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);
    
    // Out of bound indexing causes compile error
    //println!("{}", xs[5]);
}

/// Custom Types can be created with the keywords struct (define a structure)
/// and enum (define an enumeration),
/// Constants can be created with the keywords const and static.

/// 3.1 Structures

#[derive(Debug)]
struct PersonOne<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct NilOne;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and botoom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area (rect: &Rectangle) -> f32 {
    let Rectangle {top_left, bottom_right} = rect;
    let a = bottom_right.x - top_left.x;
    let b = top_left.y - bottom_right.y;
    let area = a * b;
    return area;
}

fn square(point: &Point, l: f32) -> Rectangle {
    let _square = Rectangle {
        top_left: Point {x: point.x, y: point.y + l},
        bottom_right: Point {x: point.x + l, y: point.y},
    };
    return _square;
}

fn structures() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = PersonOne { name, age };
    
    // Print debug struct
    println!("{:?}", peter);
    
    // Instantiate a `Point`
    let point: Point = Point {x: 10.3, y: 0.4};
    
    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);
    
    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };
    
    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    
    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;
    
    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };
    
    println!("{:?}", _rectangle);
    
    // Instantiate a unit struct
    let _nil = NilOne;
    
    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    
    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    
    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    
    println!("pair contains {:?} and {:?}", integer, decimal);
    
    // Calculate area of Rectangle
    println!("the size of the Rectangle is: {}", rect_area(&_rectangle));
    
    let point_two = Point {x: -1.0, y: -1.0};
    // Generate a square
    println!("{:?}", square(&point_two, 2.0));
}

/// 3.2 Enums

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click {x: i64, y: i64},
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn enums() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;
    
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
}


/// 3.2.1 use

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn enumUse() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use crate::Work::*;
    
    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;
    
    match status {
        // Note the lack of scoping because fo the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }
    
    match work {
        // Note the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}

/// 3.2.2 C-like

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum ColorTwo {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn clike() {
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    
    println!("roses are #{:06x}", ColorTwo::Red as i32);
    println!("violets are #{:06x}", ColorTwo::Blue as i32);
}

/// 3.2.3 Testcase: linked-list

use crate::ListNew::*;

enum ListNew {
    // Cons: Tuple struct that wraps an element and apointer to the next node
    Cons(u32, Box<ListNew>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl ListNew {
    // Create an empty list
    fn new() -> ListNew {
        // `Nil` has type `ListNew`
        Nil
    }
    
    // Consume a list, and return the same list with a new element at tis front
    fn prepend(self, elem: u32) -> ListNew {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }
    
    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, maching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero lenght
            Nil => 0
        }
    }
    
    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn enum_testcase() {
    // Create an empty linked list
    let mut list = ListNew::new();
    
    // Prepend some elemenets
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(4);
    
    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

/// 3.3 constants

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn constants() {
    let n = 16;
    
    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});
    
    // Error! Cannof modify ka `const`.
    //THRESHOLD = 5;
}

/// 4. Veraible Bindings

fn varbind() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();
    
    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;
    
    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);
    
    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_veriable = 3u32;
    
    let _noisy_unused_variable = 2u32;
}

/// 4.1 Mutablility

fn mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    
    println!("Before mutation: {}", mutable_binding);
    
    // Ok
    mutable_binding += 1;
    
    println!("After mutation: {}", mutable_binding);
    
    // Error
    //_immutable_binding += 1;
    
}

/// 4.2 scope and shadowing

fn scope_shadow() {
    // This binding lives in the main function
    let long_lived_binding = 1;
    
    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;
        
        println!("inner short: {}", short_lived_binding);
        
        // This binding *shadows* the outer one
        let long_lived_binding = 5_f32;
        
        println!("inner long: {}", long_lived_binding);
    }
    // End of the block
    
    // Error! `short_lived_binding` doesn't exist in this scope
    //println!("outer short: {}", short_lived_binding);
    
    println!("outer long: {}", long_lived_binding);
    
    // This binding also *shadows* the previous binding
    let long_lived_binding = 'a';
    
    println!("outer long: {}", long_lived_binding);
}

/// 4.3 Declare first

fn declare_first() {
    let a_binding;
    
    {
        let x = 2;
        
        // Initialize the binding
        a_binding = x * x;
    }
    
    println!("a binding: {}", a_binding);
    
    let another_binding;
    
    // Error! Use of uninitializedd binding
    //println!("another binding: {}", another_binding);
    
    another_binding = 1;
    
    println!("another_binding: {}", another_binding);
}



/// 5. Types, 5.1 Casting

// Suppress all wanings from casts which overflow.


fn casting() {
    let decimal = 65.4321_f32;
    
    // Error! No implicit conversion
    //let integer: u8 = decimal;
    
    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;
    
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    
    // when casting any value to an unsigned type, T,
    // std::T::Max + 1 is addedd or subtracted until the value
    // fits into the new type
    
    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);
    
    // 1000 - 256 - 256 -256 = 232
    // under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is: {}", 1000 as u8);
    // -1 + 256 = 255
    println!(" -1 as a u8 is : {}", (-1i8) as u8);
    
    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);
    
    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.
    
    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is: {}", 128 as i8);
    
    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the two's complement of 232 is -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}

/// 5.2 Literals

fn literals() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    
    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;
    
    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

/// 5.3 Inference

fn inference() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;
    
    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).
    
    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    println!("{:?}", vec);
}



// `NanoSecond` is a new name for u`u64`.
type NanoSecond = u64;
type Inch = u64;

/// 5.4 Aliasing

fn aliasing() {
    let nanoseconds: NanoSecond = 5;
    let inches: Inch = 2;
    
    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches);
}

/// 6. Conversion, 6.1 From and Into

use std::convert::From;

#[derive(Debug)]
struct NumberTwo {
     value: i32,
}

impl From<i32> for NumberTwo {
    fn from(item: i32) -> Self {
        NumberTwo { value: item }
    }
}

fn from_and_into() {
    let num = NumberTwo::from(30);
    println!("My number is {:?}", num);
    
    let int = 5;
    
    let num: NumberTwo = int.into();
    println!("My number is {:?}", num);
}

/// 6.2 TryFrom and TryInto

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn tryfrom_tryinto() {
    // TryFrom
    
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
    
    // TryInto
    
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    println!("{:?}", result);
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
    println!("{:?}", result);
}

/// 6.3 to and from strings

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn to_from_strings() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

/// 8. Flow of Control, 8.1 if/else, 8.2 loop, 8.3 while, 8.4 for and range,
/// 8.5 match, 8.6 if let, while let

enum Color3 {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

// A function `age` which returns a `u32`.
fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

enum Foo_two {
    Bar,
    Baz,
    Qux(u32)
}

fn flow_control() {
    let n = 5;
    
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }
    
    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            
            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            
            // This expresssion must return an `i32` as well.
            n / 2
        };
        
    println!("{} -> {}", n, big_n);
    
    let mut count = 0u32;
    
    println!("Let's count until infinity!");
    
    // Infinite loop
    loop {
        count += 1;
        
        if count == 3 {
            println!("three");
            
            // Skip the rest of this iteration
            continue;
        }
        
        println!("{}", count);
        
        if count == 5 {
            println!("OK, that's enough");
            
            // Exit this loop
            break;
        }
    }
    
    // Nested loops
    count = 0;
    'outer: loop {
        println!("Entered the outer loop");
        
        'inner: loop {
            println!("Entered the inner loop");
            count += 1;
            // This would break only the inner loop
            if count < 5 {
                break;
            } else {
                println!("this is the {}th loop, breaking out of the outer loop", count);
                // This breaks the outer loop
                break 'outer;
            }
        }
        println!("Just finished with the inner loop, count: {}", count)
    }
    
    println!("Exited the outer loop");
    
    // Returning from loops, if you want to return a value form a loop,
    // by putting it after the break the loop function will return the value
    
    let mut counter = 0;
    
    let result = loop {
         counter += 1;
         
         if counter == 10 {
             break counter * 2;
         }
    };
    
    println!("result is: {}", result);
    
    // with while one can run a loop as long as a condition is true
    
    // A counter varialbe
    let mut n = 1;
    
    // Loop while `n` is less than 101
    
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        
        // Increment counter
        n += 1;
    }
    
    // for loops
    
    // `m` will take the values: 1, 2, ..., 100 in each iteration
    for m in 1..=100 {
        if m % 15 == 0 {
            println!("fizzbuz");
        } else if m % 3 == 0 {
            println!("fizz");
        } else if m % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", m);
        }
    }
    
    // for and iterators
    
    let names = vec!["Bob", "Frank", "Ferris"];
    
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello there {}", name),
        }
    }
    
    let mut names = vec!["Bob", "Frank", "Ferris"];
    
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello there",
        }
    }
    
    println!("names: {:?}", names);
    
    // 8.5 match
    let number = 13;
    
    println!("Tell me about {}", number);
    
    match number {
        // Match a single value
        1 => println!("One!"),
        // Mach several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime") ,
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("I don't yet see whats special about this one"),
    }
    
    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
    };
    
    println!("{} -> {}", boolean, binary);
    
    // tuple destructur with match
    let pair = (3, 19);
    
    println!("Tell me about {:?}", pair);
    // Match can be used to destructure a tuple
    match pair {
        // Destructure the second
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }
    
    let color = Color3::RGB(122, 17, 40);
    
    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color3::Red => println!("The color is Red!"),
        Color3::Blue => println!("The color is Blue!"),
        Color3::Green => println!("The color is Green!"),
        Color3::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color3::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color3::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color3::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow:{}!", c, m, y),
        Color3::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }
    
    // 8.5.1.3 pointers/ref
    
    // Assign a reference of type `i32`. The `&` signifies there
    // is a referene being assigned.
    let reference = &4;
    
    match reference {
        // If `reference` is pattern matched against `&val`, it resttults
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val)
    }
    
    // To avoid the `&`, you dereference before matching.
    
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    
    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;
    
    // Rust provides `ref` for exaclty this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;
    
    // Accordingly, by difining 2 values without references, references
    // can be retrived via `ref` and `ref mut`.
    
    let value = 5;
    let mut mut_value = 6;
    
    // Use `ref` keyword to crate a reference.
    match value {
        ref r => println!("Got a reference to a vlaue: {:?}", r),
    }
    
    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it befor we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
    
    // 8.5.1.4 Structs
    
    struct Foo {
         x: (u32, u32),
         y: u32,
    }
    
    let foo = Foo { x: (1, 2), y: 3};
    
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        
        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        
        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        // Foo { y } => println!("y = {}", y)
    }
    
    // 8.5.2 Guards
    
    let pair_two = (3, 4);
    
    println!("Tell me about {:?}", pair_two);
    
    match pair_two {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a gurad
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
    
    // 8.5.3 Binding
    
    println!("Tell me what type of person you are");
    
    match age() {
        0 => println!("I'm not born yet I guess"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1 ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }
    
    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _ => (),
    }
    
    // 8.6 if let
    
    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;
    
    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`)."
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
    
    // If you need to specify a failure, sue an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i)
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }
    
    // Provide an altered failing condition.
    let i_like_letters = false;
    
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
    
    // Create example variables
    let a = Foo_two::Bar;
    let b = Foo_two::Baz;
    let c = Foo_two::Qux(100);
    
    // Variable a matches Foo::Bar
    if let Foo_two::Bar = a {
        println!("a is foobar");
    }
    
    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo_two::Bar = b {
        println!("b is foobar");
    }
    
    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo_two::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
    
    let d = Foo_two::Bar;
    
    if let Foo_two::Bar = d {
        println!("d is foobar");
    }
    
    // 8.7 while let
    
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);
    
    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`."
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ less rightward drift match and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional option `else`/`else if`
    // clauses. `while let` does not have these.
}


/// 9. functions

fn functioned_fizzbuzz() {
    // We can use this function here, and define it somewhere later
    fizzbuzz_to(100);
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }
    
    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

// Function that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
      for n in 1..=n {
          fizzbuzz(n);
      }
}

/// 9.1 Methods
/// Methods are the functions of an Oject. They have access to all the data of
/// their object by using the self keyword and are defined in an impl block.

struct Point_new {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` methods go in here
impl Point_new {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are gnerally used as constructors
    fn origin() -> Point_new {
        Point_new { x: 0.0, y: 0.0 }
    }
    
    // Another static method, taking two arguments:
    fn new( x: f64, y: f64 ) -> Point_new {
        Point_new { x: x, y: y }
    }
}

struct Rectangle_new {
    p1: Point_new,
    p2: Point_new,
}

impl Rectangle_new {
    // This is an instance method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point_new { x: x1, y: y1} = self.p1;
        let Point_new { x: x2, y: y2} = self.p2;
        
        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        ((x1 - x2).abs() + (y1 - y2).abs())
    }
    
    fn perimeter(&self) -> f64 {
        let Point_new { x: x1, y: y1 } = self.p1;
        let Point_new { x: x2, y: y2 } = self.p2;
        
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
    
    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        
        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair_new` owns resources: two heap allocated integers
struct Pair_new(Box<i32>, Box<i32>);

impl Pair_new {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // Destrucure `self`
        let Pair_new(first, second) = self;
        
        println!("Destroying Pair({}, {})", first, second);
        
        // `first` and `second` go out of scope and get freed
    }
}

fn methods() {
    let rectangle = Rectangle_new {
        // Static methods are called using double colons
        p1: Point_new::origin(),
        p2: Point_new::new(3.0, 4.0),
    };
    
    // Instance methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, ie.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());
    
    let mut square = Rectangle_new {
        p1: Point_new::origin(),
        p2: Point_new::new(1.0, 1.0),
    };
    
    square.translate(1.0, 1.0);
    
    let pair = Pair_new(Box::new(1), Box::new(1));
    
    pair.destroy();
}

/// 9.2 Closures

// a function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce() {
    
    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
// The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {
     f(3)
}

// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply_fn<F>(f: F) where
    F: Fn() {
    f();
}

// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn example_function() {
    println!("I'm a function!");
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    
    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    
    move || println!("This is a: {}", text)
}

fn closures() {
    // Increment via closures and functions.
    fn function (i: i32) -> i32 { i + 1  }
    
    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation bu is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i | i + 1;
    
    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    
    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    
    println!("closure returning one: {}", one());
    
    let mut j: i32 = 0;
    while j < 10 {
        j = closure_inferred(j);
        println!("current it: {}", j);
    }
    
    // 9.2.1 Capturing
    use std::mem;
    
    let color = "green";
    
    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);
    
    // Call the closure using the borrow.
    print();
    
    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`
    let _reborrow = &color;
    print();
    
    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = &color;
    
    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closre which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    
    // Call the closure using a mutable borrow.
    inc();
    
    // The clousure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    inc();
    
    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;
    
    // A non-copy type.
    let movable = Box::new(3);
    
    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    
    // `consume` consumes the variable so this can only be called once.
    consume();
    
    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];
    
    // move before vertical pipes forces closure to take ownership of captured
    // variables
    let contains = move |needle| haystack.contains(needle);
    
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    
    // println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.
    
    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
    
    // 9.2.2 as input parameters
    
    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();
    
    // Capture 2 variables: `greeting` by reference and 
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}", greeting);
        
        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");
        
        // Manually calling drop forces `farewell` to
        // be capured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };
    
    // Call the function which applies the closure.
    apply(diary);
    
    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;
    
    println!("3 doubled: {}", apply_to_3(double));
    
    // 9.2.3 type anonymity
    
    let x = 7;
    
    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`
    let print = || println!("{}", x);
    
    apply_fn(print);
    
    // 9.2.4 input functions
    
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");
    
    call_me(closure);
    call_me(example_function);
    
    // 9.2.5 As output parameters
    
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    
    fn_plain();
    fn_mut();
    fn_once();
    
    // 9.2.6.1 iterator::any
    
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    
    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
    
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    
    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`.
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
    
    // 9.2.6.2 searching through iterators
    
    let vec3 = vec![1, 2, 3];
    let vec4 = vec![4, 5, 6];
    
    // `iter()` for vecs yields `&i32`.
    let mut iter = vec3.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec4.into_iter();
    
    // `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    println!("Find 2 in vec3: {:?}", iter.find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec4: {:?}", into_iter.find(|&x| x == 2));
    
    let array3 = [1, 2, 3];
    let array4 = [4, 5, 6];
    
    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array3: {:?}", array3.iter().find(|&&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`
    println!("Find 2 in array4: {:?}", array4.into_iter().find(|&&x| x == 2));
    
    // `Iterator::position` gives you the index of an item
    let vec = vec![1, 9, 3, 3, 13, 2];
    
    let index_of_first_even_number = vec.iter().position(|x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));
    
    let index_of_first_negative_number = vec.iter().position(|x| x < &0);
    assert_eq!(index_of_first_negative_number, None);
}

/// 9.3 higher order functions

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn hof() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;
    
    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;
        
        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
             // Accumulate value, if it's odd
             acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);
    
    // Functional approach
    let sum_of_squared_odd_numbers: u32 = 
        (0..).map(|n| n * n)				 // All natural numbers squared
            .take_while(|&n_squared| n_squared < upper) // Below upper limit
            .filter(|&n_squared| is_odd(n_squared))	 // That are odd
            .fold(0, |acc, n_squared| acc + n_squared); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

/// 9.4 Diverging functions

fn diverg_func() {
     fn sum_odd_numbers(up_to: u32) -> u32 {
         let mut acc = 0;
         for i in 0..up_to {
              // Notice that the return type of this match expression must be u32
              // becasue of the type of the "addition" variable.
              let addition: u32 = match i%2 == 1 {
                  // The "i" variable is of type u32, which is perfectly fine.
                  true => i,
                  // On the otherr hand, the "continue" expression does not return
                  // u32, but it is still fine, because it never returns and therefore
                  // does not violate the type requirementsof the match expression.
                  false => continue,
              };
              acc += addition;
         }
         acc
     }
     println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}

