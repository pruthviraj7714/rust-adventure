//Iterator

// Write a function that takes a slice of integers and returns a vector containing the squares of all even numbers from the slice.

// Input: [1, 2, 3, 4, 5, 6]
// Output: [4, 16, 36]

fn square_of_even(v : &mut Vec<i32>) -> Vec<i32> {

    let iter1 = v.iter_mut();

    let v2 : Vec<i32> = iter1.filter(|x| **x % 2 == 0).map(|x| *x * *x).collect();

    v2
}

fn increment_ele_by_1(v : &mut Vec<i32>) {
    let mut iter = v.iter_mut();

    while let Some(val) = iter.next() {
        *val = *val + 1;
    }

}

// Problem 6: Sum of Positive Numbers
// Given an array of integers, write a function that returns the sum of all positive numbers.

// Input: [-3, 5, -1, 8, -2, 6]
// Output: 19
// Use iter(), filter(), and sum().

fn sum_of_pos_int(v: &Vec<i32>) -> i32 {
    v.iter()
     .filter(|&&x| x >= 0)  // Use `&&x` to dereference the reference twice
     .sum()  // Sum returns a single value
}

// Problem 7: Splitting Words
// Write a function that splits a sentence into individual words and returns them as a vector of strings.

// Input: "Rust is fun"
// Output: ["Rust", "is", "fun"]

fn split_word(str: &str) -> Vec<&str> {
    // let mut words_array = Vec::new();

    // for word in str.split_whitespace() {
    //     words_array.push(word);
    // }

    // words_array
    str.split_whitespace().collect()
}

// Problem 8: Find Maximum Using Iterators
// Write a function that finds the maximum element in an array of integers using an iterator.

// Input: [3, 1, 4, 1, 5, 9]
// Output: 9
// Use iter() and max().

fn max_from_vec(v: &Vec<i32>) -> i32 {
    match v.iter().max() {
        Some(&val) => val,  // Dereference the value since max() returns a reference
        None => -1          // Return -1 for empty vector
    }
}

// Problem 9: Group Elements by Even and Odd
// Write a function that takes a vector of integers and splits them into two vectors: one containing all even numbers and the other containing all odd numbers.

// Input: vec![1, 2, 3, 4, 5, 6]
// Output: ([2, 4, 6], [1, 3, 5])
// Use partition() to solve this.


fn group_by_even_odd(v : &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let iter = v.iter();
    iter.partition(|&x| x % 2 == 0)
}

// Problem 10: Iterator that Counts Vowels
// Write a function that takes a string and returns the number of vowels (a, e, i, o, u) using an iterator.

// Input: "hello world"
//Output: 3


fn count_vowels(str : &str) -> i32 {
    let mut count = 0;
    for char in str.chars() {
        if char == 'a' || char == 'e' || char == 'i' || char == 'o' || char == 'u' {
            count += 1;
        }
    }
    count
}


fn main() {
    // let mut vec = vec![1,2,3,4,5];

    // let mut iter = vec.iter();

    // for val in iter {
    //     println!("The Value {}",val);
    // }

    // let iter2 = vec.iter_mut();

    // for val in iter2 {
    //     *val = *val + 1;
    // }

    // while let Some(val) = iter.next() {
    //     print!("{} ",val);
    // }

    // println!("{:?}",vec);

    // let vec1 = vec![1,4,6,2,3,7];

    // let v1_iter = vec1.iter();

    // let iter2 = v1_iter.filter(|x| *x % 2 == 1).map(|x| x * 2);

    // let v2 : Vec<i32> = iter2.collect();

    // println!("{:?}",v2);

    // let mut vec2 = vec![1,2,3,4,5];
    // let v3 = square_of_even(&mut vec2);
    // println!("{:?}",v3);

    // let mut vec3 = vec![4,61,2,3];
    // increment_ele_by_1(&mut vec3);
    // println!("{:?}",vec3);

    // let mut vec = vec![3,2,6,1,2];
    // let sum = sum_of_pos_int(&mut vec);
    // println!("The sum is {}", sum);

    // let str = "Rust is Programming Language";
    // let word_vect = split_word(&str);
    // println!("{:?}",word_vect);


    // let vect = vec![4,6,11,61,2,12,63];
    // // println!("The max element is {}", max_from_vec(&vect));
    // println!("The even and odd partition : {:?}", group_by_even_odd(&vect));

    // let str = "Hello World";
    // println!("The count of character is {}", count_vowels(&str));

    //Closures

    //NO arguument
    let greet = || println!("Hello world");
    greet();

    //Single
    let double = |x| x * 2;
    println!("Double of 4 is {}",double(4));

    //Multiple
    let addition = |x, y| x + y;
    println!("The sum of 4 & 5 is {}",addition(4,5)); 

    //closures with iterator
    let numbers = vec![1,5,2,6,23];
    let vec2 : Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).map(|x| *x * *x).collect();
    println!("The square of even is {:?}",vec2);
}
