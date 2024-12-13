fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    for number in numbers {
        println!("The number is: {}", number);
    }

    // Handling empty vectors
    let empty_numbers: Vec<i32> = vec![];
    if empty_numbers.is_empty() {
        println!("The vector is empty");
    } else {
        for number in empty_numbers {
            println!("The number is: {}", number);
        }
    }
}