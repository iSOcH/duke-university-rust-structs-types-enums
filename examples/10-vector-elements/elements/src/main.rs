fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    println!("other_numbers before using it for append: {other_numbers:?}"); // [7, 8]
    v.append(&mut other_numbers); // moves elements from `other_numbers` which becomes empty
    println!("{:?}", v);
    println!("other_numbers after using it for append: {other_numbers:?}"); // empty

    // insert items at a given index
    // shifts element at `index` (and following) to the right
    v.insert(0, -1);
    println!("{:?}", v); // Output: [-1, 1, 2, 3, 4, 5, 6, 7, 8]

    // when only passing one value it appears we need to somehow duplicate it.
    // when using simply i32 (or similar primitive type) instead of implementing a generic method, the implementation
    // "just works" (can insert/push the "same value" twice) since it implements Copy and is thus automatically copied
    // for a generic method OTOH it becomes a "problem" that ownership is moved to insert/push, causing the argument
    // to no longer be available for the second call
    add_at_start_and_end_copy(&mut v, 13);
    add_at_start_and_end_clone(&mut v, 14);
    println!("{:?}", v);

    append_to_vector(&mut v, vec![-3, -4]);
    println!("After append_to_vector: {v:?}");
}

fn add_at_start_and_end_clone<T: Clone>(vec: &mut Vec<T>, value: T) -> () {
    vec.insert(0, value.clone());
    vec.push(value);
}

fn add_at_start_and_end_copy<T: Copy>(vec: &mut Vec<T>, value: T) -> () {
    vec.insert(0, value);
    vec.push(value);
}

fn append_to_vector<T, C>(vec: &mut Vec<T>, vec2: C) -> ()
    where C: IntoIterator<Item = T> {
    vec.extend(vec2);
}

/*
What are the different methods available for adding elements to a vector in Rust? How do they differ in terms of the types of elements they accept and the changes they make to the vector?
    Seen above

In the provided code, what would be the result of uncommenting and executing the line `println!("{:?}", v);` after each addition operation? How would the program output change?
    Already commented above...
 */