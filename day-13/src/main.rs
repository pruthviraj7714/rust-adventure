use core::str;
use std::cell::RefCell;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct MyStruct {
    number : i32,
    message : String
}

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct1 {
    number : i32,
    message : String
}

fn append_something(s : &mut String) {
    s.push_str(" World");

}

fn describe(str : &String) -> i32 {
    str.len() as i32
}

fn capitalize(str : &mut String) {
    *str = str.to_uppercase()
}

fn increment(r : &RefCell<i32>) {
    *r.borrow_mut()+=1
}

fn add_name<'a>(name : &'a str, names : &RefCell<Vec<&'a str>>) {
    names.borrow_mut().push(name);
}

fn display_names(names : &RefCell<Vec<&str>>) {
    for i in names.borrow().iter() {
        print!("{} ",i);
    }
}


fn display_and_modify(a : &i32) {
    println!("{}",a);
}

fn display_slice(a : &[i32]) {
    a.iter().for_each(|x| {
        print!("{} ",x);
    });
}

fn concat_byte_slices(a : &[u8], b : &[u8]) -> Vec<u8> {
    let mut slice = Vec::with_capacity(a.len() + b.len());
    slice.extend_from_slice(a);
    slice.extend_from_slice(b);
    slice
}

fn main() {
    // let original = MyStruct {
    //     number : 43,
    //     message : String::from("test")
    // };

    // let serialize = original.try_to_vec().expect("Failed to serialize");

    // println!("{:?}",serialize);

    // let deserialize = MyStruct::try_from_slice(&serialize).expect("Failed to deserialize");

    // println!("{:?}",deserialize);

    let original = MyStruct1 {
        number : 40,
        message : "Hello".to_string()
    };

    let serialized = serde_json::to_string(&original).expect("Failed to serialize");

    // println!("{}",serialized);

    let deserialize : MyStruct1 = serde_json::from_str(&serialized).expect("Failed to deserialize");

    // println!("{:?}",deserialize);

    // let mut data = String::from("Hi there");
    // append_something(&mut data);
    // println!("{}",data);

    let data = RefCell::new(3);

    // let data_ref = data.borrow();

    // println!("{}",*data_ref);

    *data.borrow_mut() += 10;
    // println!("{:?}",data.borrow());

    let mut str = String::from("Hello");
    // println!("The lenght of str is {}",describe(&str));
    capitalize(&mut str);
    // println!("{}",str);

    // let current_count = RefCell::new(0);
    // increment(&current_count);
    // println!("{}",current_count.borrow());
    // increment(&current_count);
    // println!("{}",current_count.borrow());
    // increment(&current_count);
    // println!("{}",current_count.borrow());
    // increment(&current_count);
    // println!("{}",current_count.borrow());


    let names = RefCell::new(vec!["Raman","Raj","Ram"]);
    // display_names(&names);

    add_name("Sara", &names);
    // display_names(&names);

    let mut a = 40;
    display_and_modify(&a);

    let b = &mut a; 
    // println!("b: {}", b);  


    //Slices
    let arr = vec![1,2,3,4];
    let arr_slice = &arr[0..2];
    // println!("{:?}",arr_slice);

    let mut arr = vec![1,2,3,4,5];
    let  arr_slice2: &mut [i32] = &mut arr[1..5];
    arr_slice2[0] = 20;
    // println!("{:?}",arr_slice2);

    let vector = vec![1,3,5,66,21];
    // display_slice(&vector[0..3]);

    let data = vec![0, 1, 2, 3, 4, 5];

    let byte_slice : &[u8] = &data[0..4];

    // println!("{:?}",byte_slice);

    let text = "hello";
    let string_byte_slice : &[u8] = text.as_bytes();
    // println!("{:?}",string_byte_slice);

    let string_from_bytes = str::from_utf8(string_byte_slice).expect("Invalid UTF8");
    // println!("{}",string_from_bytes);

    let bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
    let number = u32::from_be_bytes(bytes);
    // println!("Parsed number: {}", number);

    let mut data = vec![1,5,1,51,5];
    let slice : &mut [i32] = &mut data[0..2];
    slice[0] = 15;
    // println!("{:?}",slice);

    let test: Vec<u8> = vec![4,16,6,72,1];
    let slice1: &[u8] = &test[1..5];
    let slice2: &[u8] = &test[3..4];
    let result: Vec<u8> = concat_byte_slices(&slice1, &slice2);
    println!("{:?}",result);

}

