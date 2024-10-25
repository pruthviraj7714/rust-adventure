//String & Slices

// fn find_first_word(str : &str) -> &str {
//     let mut space_index = 0;
//     for i in str.chars() {
//         if i == ' ' {
//             break;
//         }
//         space_index = space_index + 1;
//     }

//     return &str[0..space_index];
// }

// fn main() {
//     let mut str = String::from("Hello Worlds");
//     // let str2 = &str[0..5];
//     println!("{}",str);
//     println!("The first word is {}",find_first_word(&str));
// }

// Generics

pub trait Summury {
    fn summurize(&self) -> String;
    //default function
    fn test(&self) -> String {
        return String::from("Hi there");
    }
}

pub trait Fix {
    fn new(&self) -> String {
        return String::from("This is a new trait");
    }
}
struct User {
    name: String,
    age: i32,
}

impl Summury for User {
    fn summurize(&self) -> String {
        return format!("The age of {} is {}", self.name, self.age);
    }
}

impl Fix for User {}

fn main() {
    let user1 = User {
        name: String::from("Tony"),
        age: 43,
    };

    println!("{}", user1.summurize());
    println!("{}", user1.test());

    
    notify(user1);
    notify(2)
    
}

fn notify<T : Summury + Fix>(u: T) -> String {
    return String::from("test");
}