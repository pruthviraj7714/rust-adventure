#[derive(Debug, Clone)]
struct User {
    first_name : String,
    last_name : String,
    age : i32,
    username : String
}
struct  Rectangle {
    width : i32,
    height : i32
}
fn main() {
    //String
    let mut s = String::from("Hello");
    s.push_str(" world");
    println!("{}",s);

    //&str
    let s = "Hello world";
    println!("{}",s);

    //ownership
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}",s1); //This will give error as ownership is transfered to s2

    println!("{}",s2);

    let user: User = User {
        first_name : String::from("Tony"),
        last_name : String::from("Stark"),
        age : 32,
        username : String::from("tony")
    };

    let user2: User = User {
        username : String::from("spiderman"),
        ..user.clone()
    };

    println!("The username of user is {}", user.username);
    println!("The first name of user is {}", user.last_name);
    println!("The username of user2 is {}", user2.username);

    impl Rectangle {
        fn area(&self) -> i32 {
            self.height * self.width
        }
    }

    let rect = Rectangle {
        height : 30,
        width : 30
    };

    println!("The are of rectangle is {}", rect.area());


}
