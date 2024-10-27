//Lifetimes

fn longest(a: String, b: String) -> String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn longest2<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

struct User<'a> {
    name: &'a str,
}

fn main() {
    // let a = String::from("Tony");
    // let b = String::from("Stark");
    // let res = longest(a, b);
    // println!("{}", res);
    // let ans;
    // let ans2;

    // let a = String::from("tony");
    // {
    // let b = String::from("stark");
    // // ans = longest(a, b);
    // ans2 = longest2(&a, &b);
    // }
    // // println!("{}",ans);
    // println!("{}",ans2);

    let my_name = String::from("Hello");

    let user = User { name: &my_name };

    println!("the name of user is {}", user.name);
}
