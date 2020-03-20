//here i'll add some comments not much changed here

/*here is a block comment, wow this is crAZY fun
ha my comment just goes on and on 
and on and on*/

///okay im told this is a doc comment and will be used for the html docs, very cool
fn main() {
    println!("Hello, world!");
    println!("I am person of this world!");
    formatted();
}


///Exercise 1.2 formatted print https://doc.rust-lang.org/stable/rust-by-example/hello/print.html
fn formatted(){
    //as i understand it the {} when using the println! macro is replaced by any argument that 
    //followes it and that argument is stringified (made to a string)
    //if i don't use a suffix for 12 it will become a i32, by adding a suffix it will become whatever 
    //i say it is
    println!("so here is the argument, {}", 12);
    
    //another cool thing that can be done
    println!("{0}, this is {1}. {1}, this is {0}. Lets not talk out in the open, {2} might be listening.", "Alice", "Bob", "Eve" );
    
    //just dirrectly copying the example from `rust-by-example`, apperently one can name arguments and 
    //have them print which is super cool!
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");
        
    //in a {} one can specify speciall formating after a `:`
    println!("{} of {:b} people know binary, the other half doesen't", 1, 2);
    
    //I can right-align text with a specified with. this will use spaces to right-align
    println!("{number:>width$}", number=1, width=21);
    
    //I can right-align text with a specified with. this will use 0s to right-align
    println!("{number:>0width$}", number=1, width=21);
    
    //fixed example
    println!("My name is {0}, {1} {0}", "Bond", "James");
    
    //printing the number of decimal places of pi by defining it in the println! macro
    let pi = 3.142592;
    println!("here is some pi with a precision of {prec}: {number:.prec$}", prec=5, number=pi);
}
