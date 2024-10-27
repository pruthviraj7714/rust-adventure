// enum TrafficLight {
//     RED,
//     YELLOW,
//     GREEN
// }

// fn light_duration(light : TrafficLight) {
//     match light {
//         TrafficLight::GREEN => println!("Stays for 60 sec"),
//         TrafficLight::RED => println!("Stays for 30 sec"),
//         TrafficLight::YELLOW => println!("Stays for 5 sec")
//     }
// }

// fn main() {
//     println!("This is day 3 of Learning of Rust");
//     let mut light = TrafficLight::GREEN;
//     light_duration(light);
//     light = TrafficLight::RED;
//     light_duration(light);

// }

// //Calculator with enums
// enum Operation {
//     Add(i32, i32),
//     Subtract(i32, i32),
//     Mult(i32, i32),
//     Divide(i32, i32),
// }

// fn calculate(op: Operation) -> i32 {
//     match op {
//         Operation::Add(a, b) => a + b,
//         Operation::Subtract(a, b) => a - b,
//         Operation::Mult(a, b) => a * b,
//         Operation::Divide(a, b) => a / b,
//     }
// }

// fn main() {
//     let a = 6;
//     let b = 4;

//     let mut ans = calculate(Operation::Add(a, b));
//     println!("{}", ans);
//     ans = calculate(Operation::Subtract(a, b));
//     println!("{}", ans);

//     ans = calculate(Operation::Mult(a, b));
//     println!("{}", ans);

//     ans = calculate(Operation::Divide(a, b));
//     println!("{}", ans);
// }

// //Message System
// enum Message {
//     Send(String),
//     Recieve(String),
//     Error(u32, String)
// }

// fn handle_message(msg : Message) {
//     match msg {
//         Message::Recieve(x) => println!("Message Recived : {}",x),
//         Message::Send(x) => println!("Message Sent : {}", x),
//         Message::Error(y,x) => println!("The error {} with status code {}",x, y)
//     }
// }

// fn main() {
//     let mut message = Message::Send(String::from("Hi there"));
//     handle_message(message);
//     message = Message::Recieve(String::from("Recived"));
//     handle_message(message);
//     message = Message::Error(500, String::from("Internal Server Error"));
//     handle_message(message);
// }

// //Shape Enum with Area Calculation
// enum Shape {
//     Circle(f32),
//     Rectangle(f32, f32)
// }

// fn area(shape : Shape) -> f32 {
//     match shape {
//         Shape::Circle(x) => 3.14 * x * x,
//         Shape::Rectangle(y,z ) => y * z
//     }
// }

// fn main() {
//     let c = Shape::Circle(6.43);
//     let r = Shape::Rectangle(5.5, 10.5);

//     println!("The area of circle is {:?}", area(c));
//     println!("The area of rectangle is {:?}", area(r));
// }

//options
// fn main() {
//     let index = get_first_a(String::from("mail"));
//     let index2 = get_first_a(String::from("test"));
    
//     match index {
//         Some(val) => println!("{}",val),
//         None => println!("Erorr")
//     }

//     match index2 {
//         Some(val) => println!("{}",val),
//         None => println!("Erorr")
//     }
// }

// fn get_first_a(str: String) -> Option<usize> {
//     for (index, char) in str.chars().enumerate() {
//         if char == 'a' {
//             return Some(index);
//         }
//     }
//     None 
// }

// use std::fs;


// fn main() {

//     let file_result = fs::read_to_string("index.txt");

//     match file_result {
//         Ok(data) => println!("{}",data),
//         Err(err) => println!("{}", err)
//     }


// }

use chrono::{Local,Utc};

fn main() {
    let now = Local::now();
    println!("Current Time is {}",now);

    let time_in_utc = Utc::now();
    println!("Current time acc to UTC : {}", time_in_utc);
}