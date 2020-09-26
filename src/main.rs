#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)] 
#![crate_name = "ACC"]
//#![allow(unused_brain)] 
/// cośkolwiek 
/// 
/// 
use std::fmt;
use rand::Rng;
fn main() {
    println!("Hello, world!");
    //println działa jak format, tekst jest drukowanyu do konsoli io::stout
    println!("{}  days",32);
    // bez podawania typu zmiennej, rust automatycznie nadaje naszej zmiennej typ i32

        // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    println!("My name is {0}, {1} {0} {} {} {0} {1}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"
    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    // struct Structure(i32);
    struct S {
        bar: i32
    }

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct {:#?} won't print...", Structure(3));
    // FIXME ^ Comment out this line.

// ZA CHOLERE
    impl fmt::Debug for S {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.debug_struct("S")
               .field("bar", &self.bar)
               .finish()
        }
    }
    

    println!("This will work: {:?} ", S{bar: 3});

    let pi = 3.141592;
    println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);
    println!(" {} with 3 fractional digits: {pi:.prec$} ","the pi num", prec = 4, pi=pi);


    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(2,10));

    println!("roll the dice");

    use rand::distributions::{Distribution, Uniform};

    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the dice; {}", throw);
        if throw == 6 {
            break;
        }

    use rand_distr::{Distribution, Normal, NormalError};

    
    



    }
    

}




