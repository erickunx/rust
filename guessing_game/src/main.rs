use std::io;    //by default rust has a lot of built in defined as the standard library which is the prelude, if
                    //a type you wnant is not included you can add that type into the scope by using
                    //the use statement
                //std means that it is from the standard library
                //io is a library that provides you with features that include the ability to
                    //accept user input

use rand::Rng;  //Rng defines methods thar random number generators implements and for a random number to be 
                    //generated this is needed
                // you will not know what traits to use adn which methods and functions to call
                    // from a crate so each crate has documentation with instructions for using them
                    // you can run "cargo doc --open" which will build documentation provided by
                    // all the dependencies locally and open it in your browser   

use std::cmp::Ordering;     //brings in the Ordering type into the scope which has variants of "Less", "Greater"
                                //, and "Equal" which are the three possible outcomes when
                                //comparing two values

//fn declares a new function 
//() means that there are no parameters
fn main() {
    println!("Guess the number!");      //println! is a macro that prints a string to the screen

    let secret_number = rand::thread_rng().gen_range(1..=100);      //rand::thread_rng is the specific random
                                                                        //number generator that is
                                                                        //used and is made possible
                                                                        //by using Rng trait that
                                                                        //was brought in by "use
                                                                        //rand:: Rng"
                                                                    //"gen_range" takes a range
                                                                        //expression as an argument and
                                                                        //generates a random number
                                                                        //in that range in the form
                                                                        //of "{start}..={end}" and
                                                                        //is inclusive on the lower
                                                                        //and upper bounds so
                                                                        //"1..=100" is numbers
                                                                        //between 1 and 100 
    
    println!("The secret number is: {secret_number}");      //prints the secret number {} allows for printing
                                                                //of variables

    println!("Please input your guess.");

    let mut guess = String::new();      //let is a keyword for creating a variable
                                        //mut means that the variable is mutable 
                                        //String::new() creates a new, empty instance of a string
                                        //all together, this line creates a mutable varianle that
                                        //is bound to a new empty instance of a String

    io::stdin()                         //these line can be written as
        .read_line(&mut guess)          //io::stdin().read_line(&mut guess).expect("failed to read li
        .expect("Failed to read line"); //but this is harder to read therefore it is split into three lines
    
    println!("You guessed: {}", guess); //current print out of what the guess is 

    //comparing section adding the use of Ordering demo of match construct

    match guess.cmp(&secret_number){                //"match" expression is used to what to do based on which variant of Ordering was returned, cmp compares the variable 
        Ordering::Less => println!("Too small!"),   //this is a arm that makes up the match expression this one in particular is when the guess is less than the secret
        Ordering::Greater => println!("Too big!"),  //this arm is when the guess is greater than the secret
        Ordering::Equal => println!("You win!"),    //this arm is when the guess is the same as the secret
    }   
}

