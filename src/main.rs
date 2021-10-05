use std::io;

fn main() {
   match_that_hoe(89);
}


// Conditionals 
fn _this_or_that() {

    let a = true;
    let b = false;

    let c : f64 = 89.9;

    let d = 'c';

    
    // Simple tuple
    let tup = (d, c, a);

     // Tuple values accessed via indeces
     let first = tup.0;
     let second = tup.1;
 
     // destructured tuple
     let (_x, _y, _z) = tup;
 

    if a {
        println!("That right");
    }

    if b {
        println!("That tight");
    }

    if tup.2 {
        println!("You just messed with a tuple with the corresponding values {} {} {}", first, second, tup.2);
    }

}

fn _slice_that_hoe() {
    // Array elements are of the same type
    // They are of fixed sized upon initialization
    let _new_array = [1.01, 2.2, 99.90];
    
    // Array element accessed by index
    let _first_array_element = _new_array[0];

    //Slices - contiguous subset of data in another data structure
    let _first_slice = &_new_array[0..2];

    println!("This is the second element of my first slice {}", _first_slice[1]);
}

// Function with no return value
fn _next_birthday(name: &str, current_age: u8)  {
    let next_age = current_age + 1;
    println!("Hi {}, on your next birthday, you'll be {} years old", name, next_age);
}

// Function with a return value
fn _square(number : i32) -> i32 {
    number * number
}

// Exploring control flow
fn _discount(day_of_month: u8) {
    // Conditional variable assignment-ish
    let amount = if day_of_month % 2 == 0 {
        50
    } else {
        10
    };

    println!("Your discount is {}", amount);
}

// Looping
fn _right_word() {

    loop{
        println!("What is the secret word?");
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");

        if word.trim() == "rust" {
            break;
        }
    }

    println!("You know the secret word! Please proceed");
}

fn _right_word_while() {

    let mut word = String::new();
    while word.trim() != "rust" {
        println!("What is the secret word?");
        word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");   
    }

    println!("You know the secret word! Please proceed");
}

fn _for_looper() {
    for i in 1..11 {
        println!("Serving {}", i);
    }
}

// Pattern Matching
fn match_that_hoe(number : u8) {

    match number {
        1 => println!("That's a single fucking number"),
        2 => println!("We're talking double digits here"),
        3 => println!("Tripple that bitch up"),
        _ => println!("I don't know what the fuck you're talking about")
    }
} 