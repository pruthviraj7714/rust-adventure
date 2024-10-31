fn is_even(num : i32) ->  Result<bool,String> {
    if num % 2 == 1 {
        Err("Thy given number is odd".to_string())
    }else {
        Ok(num % 2 == 0)
    }
}

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(content)
}

fn main() {
    let data = [2,6,26,16,1,42];
    let slice = &data[1..4];

    println!("The length of slice is {}",slice.len());
    println!("{:?}",slice);

    if let Some(element) = data.get(4) {
        println!("The val at index 4 is {element}");
    }

    let (left, right) = data.split_at(2);
    println!("The left slice {:?} and right slice {:?}", left, right);

    println!("The chunks are as follows: ");
    for chunk in data.chunks(2) {
        println!("{:?}",chunk);
    }


    //Create and Print a Slice:
    // Given an array, create a slice that includes only the middle elements and print it.
    let arr = [1,2,3,4,5];
    let mid_slice = &arr[arr.len()/2];
    println!("{:?}",mid_slice);

    for chunk in arr.chunks(3) {
        println!("{:?}",chunk);
    }

    let num = 4;
    let num2 = 5;

    match is_even(num) {
        Ok(val) => println!("The num {} is even : {}",num,val),
        Err(e) => println!("{}",e)
    }

    match is_even(num2) {
        Ok(val) => println!("The num {} is even : {}",num,val),
        Err(e) => println!("{}",e)
    }

    let content = read_file("a.txt");
    match content {
        Ok(val) => println!("{}",val),
        Err(e) => println!("{}",e)
    }

    let data_arr = [1,5,6,6,1];
    let data_slice = &data_arr[..];
    println!("{:?}",data_slice);
    
}

