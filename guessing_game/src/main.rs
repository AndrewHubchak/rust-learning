extern crate std; // always done implicitly
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

//fn main()
// is equal
//fn main() -> ()
//empty tuple is returned by the compiler if we did not specify something else
//explicitly
fn main() {
    println!("Guess the number. You can input you number now:");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // method gen_range is inclusive on the lower bound
    // but exclusive on the upper bound 
    // so [0, 101)

    loop {
        let ref mut guess = String::new();
        // v1: let mut guess = String::new();
        // v2: let mut guess = &mut String::new();
        // v3: let ref mut guess = String::new();
        //
        // v2 and v3 are eqvivalent
        // this is a rust feature called `references`
        //
        // !Note
        // !References are one the the rust key features
        //
        // with v1 you should call read_line as
        // io::stdin().read_line(&mut guess)
        //
        // let is immutable by default
        // it is called 'variable binding' instead of just variabled as in other languages
        // it accepts a `pattern` on the right side
        //
        // String is a growable UTF-8 encoded bit of text
        //
        // ::new() was called instead of .new()
        // because this is an `associated function` of a particular type.
        // That is to say, it's associated with String type itself, rather that a 
        // particular instance of the String type. In C++ this would be a `static member function`,
        // aka `static method`

        io::stdin().read_line(guess)
         .ok()
         .expect("Read has failed");
        
        // io.stdin() or std::io::stdin()
        // This particular `associated function` returns a handle to the standard input for your
        // terminal
        //
        // .read_line(&mut guess) is used to get input from $USER
        // read_line is a method and not a `associated function`
        //
        //
        // if I don't use
        // .ok()
        // .expect("Read has failed")
        //
        // Rust compiler will give a warning.
        // We should handle all errors properly in rust,
        // but as this application is not very critical,
        // we can use .ok().expect() just want to crash on error
        // and currently for us this is okay

        // some variable shadowing here :)
        let guess : u32 = match guess.to_string().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your number is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Your number is to small"),
            Ordering::Greater   => println!("Your number is to big"),
            Ordering::Equal     => { 
                println!("You win!!! Hooray!!!");
                break;
            }
        }
    }
}
