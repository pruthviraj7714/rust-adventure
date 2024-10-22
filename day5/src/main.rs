
This is Practice day

1. Variables and Mutability

Task:
Declare an immutable variable that holds your name as a string.
Declare a mutable variable that holds your age.
Print both the name and age.
Update the age and print the new value.

fn main() {

    let name = String::from("tony");
    let mut age = 12;

    println!("The age of {} is {}",name, age);

    age = 15;

    println!("Now the age is {}", age);

}


2. Functions and Control Flow

Task:
Write a function that takes a number as input and returns whether the number is even or odd.
Call this function from main() and print the result.

fn is_even(num : i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

fn main() {
    println!("is 4 is even number? => {}", is_even(4));
}

3. Loops and Iteration
Task:
Write a program that uses a while loop to print the numbers from 1 to 10.

fn main() {
    let mut i = 1;
    while i <= 10 {
        print!("{} ", i);
        i += 1;
    }
}


4. Loops with Conditional Control
Task:

Write a program that uses a for loop to print the numbers from 1 to 20, but skips numbers divisible by 3.

fn main() {
    for i in 1..=20 {
        if i % 3 == 0 {
            continue;
        } else {
            print!("{} ", i);
        }
    }
}

5. Error Handling with Result
Task:

Write a function that attempts to open a file and read its contents.
If the file doesn't exist or cannot be opened, handle the error by printing a custom error message.

use std::{fs::File, io::Read};

fn read_file_content(file_name: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file_content("a.txt") {
        Ok(content) => println!("{}", content),
        Err(err) => println!("Error: {}", err),
    }
}


6. Simple Calculator
Task:

Write a basic calculator program that allows the user to input two numbers and choose an operation (addition, subtraction, multiplication, division).
Print the result of the operation.

enum Operation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multifply(i32, i32),
    Divide(i32, i32)
}

fn main() {
    let operation = Operation::Multifply(4,10);

    let result = match operation {
        Operation::Add(x,y ) => x + y,
        Operation::Subtract(x,y  ) => x-y,
        Operation::Multifply(x,y ) => x * y,
        Operation::Divide(x,y, ) => x / y
    };
    print!("{}",result);
}


7. Fibonacci Sequence
Task:

Write a function that returns the nth Fibonacci number.
Use a loop to calculate it and call the function from main() to display the result.


fn fib(number : i32) -> i32 {
    if number == 0 {
        return 0;
    }else if number == 1 {
        return 1;
    }

    let mut first = 0;
    let mut second = 1;

    for _ in 1..number {
        let temp = second;
        second = first + second;
        first = temp;
    }

    return second;
}
fn main() {
    let ans = fib(4);
    println!("{}",ans);
}

9. Guess the Number Game
Task:

Write a number-guessing game where the program randomly selects a number between 1 and 100.
The player has to guess the number, and the program gives hints like "Too high" or "Too low" after each guess.
The game should end when the player guesses the correct number.

use rand::Rng;          // Import random number generator
use std::io;            // Import input/output handling
use std::cmp::Ordering; // Import ordering comparison

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // Prompt the player for a guess
        println!("Please input your guess.");

        let mut guess = String::new();

        // Read the player's guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess string to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


11. Factorial Calculation
Task:

Write a function that calculates the factorial of a given number using recursion.
Call this function from main() and print the result for a given input.

fn fact(number : i32) -> i32 {
    if number == 0 {
        return 0;
    }else if number == 1 {
        return 1;
    }
    return fact(number-1) * number;
}

fn main() {
    let ans = fact(5);
    println!("{}",ans);

}


12. Reverse a String
Task:

Write a function that takes a string as input and returns the string reversed.
Call this function from main() and print the reversed string.


fn reverse_string(str : &str) -> String {
    let mut reversed_string = String::new();

    for c in str.chars().rev() {
        reversed_string.push(c);
    }

    return reversed_string;
}

fn main() {
    println!("The Reverse of string 'apple' is {}", reverse_string("apple"));

}   


14. Prime Number Check
Task:

Write a function that checks whether a given number is prime or not.
Call this function from main() and print whether the number is prime or not.


fn is_prime(number : i32) -> bool {
    if number == 0 || number == 1 {
        return  false;
    }

    for i in 2..=((number as f64).sqrt()) as i32 {
        if number % i == 0 {
            return false;
        }
    }

    return true;
}
fn main()  {
    let is_it_prime = is_prime(12);
    println!("The 43 is prime => {}", is_it_prime);

}

