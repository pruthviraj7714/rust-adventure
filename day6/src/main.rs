//Vectors

use std::collections::HashMap;

fn even_filter(v: &mut Vec<i32>) {
    let mut i = 0;

    while i < v.len() {
        if v[i] % 2 != 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    }
}

fn sum_of_elements(v : Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in v {
        sum+=i;
    }

    return sum;
}

fn maximum_element(v : Vec<i32>) -> i32 {
    let mut max_element = v[0];

    for i in v {
        if i > max_element {
            max_element = i;
        }
    }

    return max_element;
}

fn reverse_vector(v: &mut Vec<i32>) {
    if v.is_empty() {
        return;
    }

    let mut first = 0;
    let mut last = v.len()-1;

    while first < last {
        // let temp = v[first];
        // v[first] = v[last];
        // v[last] = temp;
        v.swap(first,last);
        first+=1;
        last-=1;
    }
}

fn occurence_of_target(v : Vec<i32>, target : i32) -> i32 {
    let mut no_of_occ = 0;

    for i in v {
        if i == target {
            no_of_occ += 1;
        }
    }
    return no_of_occ;
}

//Concatenate Two Vectors
fn concat_two_vectors(v1 : Vec<i32>, v2 : Vec<i32>) -> Vec<i32> {
    // let mut new_vec = Vec::new();

    // for i in v1 {
    //     new_vec.push(i);
    // }

    // for j in v2 {
    //     new_vec.push(j);
    // }

    // return new_vec;
    let mut new_vec = Vec::with_capacity(v1.len() + v2.len()); // Pre-allocate capacity.

    new_vec.extend(v1); // Extend new_vec with all elements from v1.
    new_vec.extend(v2); // Extend new_vec with all elements from v2.

    new_vec
}

fn remove_duplicates(v : Vec<i32>) -> Vec<i32> {
    let mut new_vector = Vec::new();

    for i in v {
        if new_vector.contains(&i) {
            continue;
        }else {
            new_vector.push(i);
        }
    }

    return new_vector;
}

// Problem 1: Word Frequency Counter
// Write a function that takes a string of text and returns a HashMap where the keys are words, and the values are the number of times each word appears.


fn word_freq_counter(text : &str) -> HashMap<&str,i32> {
    let mut freq_map = HashMap::new();

    for word in text.split_whitespace() {
        if freq_map.contains_key(word) {
            let is_already_added = freq_map.get(word);
            match is_already_added {
                Some(val) => freq_map.insert(word, val+1),
                None => continue
            };
        }else {
            freq_map.insert(word, 1);
        }
    }

    return freq_map;
}

fn group_by_value(v : &Vec<i32>) -> HashMap<i32, Vec<usize>> {
    let mut group_map: HashMap<i32, Vec<usize>> = HashMap::new();

    for (index, value) in v.iter().enumerate() {
       group_map.entry(*value).or_insert(Vec::new()).push(index);
    }

    return group_map;
}

 fn char_freq(str : &String) -> HashMap<char,i32> {
    let mut char_freq_map = HashMap::new();

    for char in str.chars() {
        let counter = char_freq_map.entry(char).or_insert(0);
        *counter+=1
    }

    char_freq_map
 }

fn main() {
    let mut vec = vec![1, 23, 3, 4,23];
    // even_filter(&mut vec);
    // println!("{:?}", vec);

    // println!("The sum of elements : {}", sum_of_elements(vec.clone()));
    // println!("The max element in array : {}", maximum_element(vec.clone()));

    // reverse_vector(&mut vec);
    // println!("The Reversed vector is : {:?}", vec);

    // println!("The 23 is occurenece : {}",occurence_of_target(vec, 23));

    // let mut vec2 = vec![12,16];
    // println!("The Concatinated vector is {:?}", concat_two_vectors(vec, vec2));


    // println!("The Duplicated removed : {:?}", remove_duplicates(vec));

    //Hashmap

    // let mut users = HashMap::new();

    // users.insert(String::from("tony"), 43);
    // users.insert(String::from("stark"), 86);

    // let first_value = users.get("tony");

    // match first_value {
    //     Some(val) => println!("{}",val),
    //     None => println!("No key with this name exists")
    // }

    // let text = "hello world hello rust rust rust";
    // let result = word_freq_counter(&text);
    // println!("{:?}",result);

    // Problem 2: Grouping by Value
    // Given a vector of integers, return a HashMap where the keys are the integers and the values are vectors of indices where each integer appears in the original vector.

    // let vector = vec![1, 2, 1, 3, 2, 1];
    // println!("{:?}",group_by_value(&vector));

    // Problem 3: Character Frequency
    // Write a function that takes a string and returns a HashMap of the frequency of each character (excluding spaces).

    let str = String::from("aaaaabb");
    println!("{:?}",char_freq(&str));

}
