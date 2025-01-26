use std::ops::Add;

fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn main() {
    let vec : Vec<i32> = vec![1, 2, 3, 4, 5];
    // get_item(3);

    // Retrieve a value at a specific index
    let mut third_value = vec[2];
    println!("The third value in the vector is: {}", third_value);

    // this does not mutate the vector (because i32 is Copy?)
    third_value = -3;

    // Retrieve the last value
    let last_value = vec.last().unwrap();
    println!("The last value in the vector is: {}", last_value);

    // implement a sum function
    let sum = sum(&vec);
    println!("Sum is: {sum}");

    // Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }

    let vec_of_box = vec![Box::new(1), Box::new(2)];
    let mut second_of_box = &vec_of_box[1]; // cannot use simply `vec_of_box[1]`, error "cannot move"
    second_of_box = &Box::new(3);
    println!("vec_of_box = {vec_of_box:?}");
}

// from https://users.rust-lang.org/t/writing-a-truly-generic-sum-function-and-why-is-it-missing-from-the-stdlib/101590/2
// a bit confused why the elements do not get moved / why it does not work when the argument is &[T]
fn sum<T, I, S>(iterable: I) -> S
where
    I: IntoIterator<Item = T>,
    T: Add<T, Output = S>,
    S: Add<T, Output = S>,
    S: Default,
{
    let mut result = S::default();
    for elem in iterable {
        result = result + elem;
    }
    result
}

/*
What are the different methods of retrieving values from a vector in Rust? How do they differ in terms of error handling and potential panics?
    Using the index-operator (vec[idx]) returns a T or panics whereas vec.get(idx) returns an Option<T>. There is also .first() and .last() which return Option<T>

In the provided code, what would happen if you uncommented and executed the line `println!("The third value in the vector is: {}", third_value);`? How would the program output change?
    Would output "The third value in the vector is: 3"
 */