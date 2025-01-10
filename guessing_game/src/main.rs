use std::io;    //by default rust has a lot of built in defined as the standard library which is the prelude, if
                //a type you wnant is not included you can add that type into the scope by using
                //the use statement
                //std means that it is from the standard library
                //io is a library that provides you with features that include the ability to
                //accept user input

//fn declares a new function 
//() means that there are no parameters
fn main() {
    println!("Guess the number!");      //println! is a macro that prints a string to the screen

    println!("Please input your guess.");

    let mut guess = String::new();      //let is a keyword for creating a variable
                                        //mut means that the variable is mutable 
                                        //String::new() creates a new, empty instance of a string
                                        //all together, this line creates a mutable varianle that
                                        //is bound to a new empty instance of a String

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        //above line can be written as
        //io::stdin().read_line(&mut guess).expect("failed to read line);
        //but this is harder to read therefore it is split into three lines above
    println!("You guessed: {}", guess); //current print out of what the guess is 

}

