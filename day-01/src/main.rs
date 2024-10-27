fn main() {
    println!("This is Day 1 of Learning Rust!");

    for i in 1..5 {
        print!("{} ", i);
    }

    let answer = calculator('+', 10, 30);
    println!("The ans is {}", answer);

    println!("The digit sum for num 432 is {}", digit_sum(432));

    let ans = is_even(20);
    println!("{}", ans);
    let ans2 = fib(4);
    println!("{}", ans2);
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

fn fib(num: i32) -> i32 {
    let mut first: i32 = 0;
    let mut second: i32 = 1;

    if num == 0 {
        return 0;
    }

    if num == 1 {
        return 1;
    }

    for _i in 2..=num {
        let temp = second;
        second = first + second;
        first = temp;
    }

    return second;
}

fn calculator(operation: char, num1: i32, num2: i32) -> i32 {
    if operation == '+' {
        return num1 + num2;
    } else if operation == '-' {
        return num1 - num2;
    } else if operation == '/' {
        return num1 / num2;
    } else if operation == '*' {
        return num1 * num2;
    } else {
        return 0;
    }
}

fn digit_sum(num : i32) -> i32 {
    if num == 0 {
        return 0;
    }
    let mut temp = num;
    let mut sum = 0;
    while temp > 0 {
        let last_digit = temp % 10;
        sum+= last_digit;
        temp/= 10;
    }

    return sum;
}
