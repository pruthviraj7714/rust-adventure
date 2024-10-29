
// Struct-Based Book Library
// Create a Book struct with fields for title, author, and year of publication. Write functions to add 
//books to a collection and to search for books by title or author.
use std::vec;

#[derive(Debug)]
struct Book {
    title : String,
    author : String,
    year_of_pub : i32,
}


fn add_book(title : &str, author : &str, year : i32,books : &mut Vec<Book>) {
    let new_book = Book {
        title : title.to_string(),
        author : author.to_string(),
        year_of_pub : year
    };
    books.push(new_book);
}

fn search_book<'a>(books: &'a [Book], author: &'a str, title: &'a str) -> Option<&'a Book> {
    books.iter().find(|book| book.author == author && book.title == title)
}

//Palindrome Checker
fn is_palindrome(str : &str) -> bool {
    let chars : Vec<char> = str.chars().collect();
    
    let mut start: i32 = 0;
    let mut end: i32 = (chars.len() as i32)-1;


    while start <= end {
        if chars[start as usize] != chars[end as usize] {
            return false;
        }
        start+=1;
        end-=1;
    }
    true
}

//binary search
fn binary_search(arr : &Vec<i32>, key : i32) -> Option<i32> {
    let mut start = 0;
    let mut end = arr.len() - 1;

    while start <= end {
        let mid = start + (end - start) / 2;

        if arr[mid] == key {
            return Some(mid as i32);
        }else if arr[mid] > key {
            end = mid - 1;
        }else {
            start = mid + 1;
        }
    }
    None
}

fn addition_of_two_matrices(mat1 : &Vec<Vec<i32>>, mat2 : &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for i in mat1 {
        for j in i {
            sum += j;
        }
    }

    for i in mat2 {
        for j in i {
            sum += j;
        }
    }
    sum
}


fn main() {
    let mut books: Vec<Book> = Vec::new();
    add_book("test", "test", 2001, &mut books);
    add_book("test2", "test2", 2001, &mut books);
    add_book("test3", "test3", 2001, &mut books);
    // for book in books {
    //     println!("{:?}",book);
    // }


    let my_book = search_book(&books, "test2", "test3");

    match my_book {
        Some(val) => println!("Book is found : {:?}",val),
        None => println!("No Book found!")
    }

    println!("camel is palindrome : {}", is_palindrome("camel"));
    println!("rasar is palindrome : {}", is_palindrome("rasar"));

    let arr = vec![2,5,6,7,9,10,15];
    println!("The key 7 is at index {}",binary_search(&arr, 10).unwrap());

    let mat1 = vec![vec![4,3,1],vec![6,1,3],vec![9,3,5]];
    let mat2 = vec![vec![4,3,1],vec![6,1,3],vec![9,3,5]];

    let sum = addition_of_two_matrices(&mat1, &mat2);
    println!("The sum is {}",sum);

}