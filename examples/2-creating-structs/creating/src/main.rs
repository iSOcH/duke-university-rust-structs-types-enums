#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
    email: Option<String>,
    phone_number: Option<String>,
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let alfredo = Person{
        first_name: "Alfredo".to_string(),
        last_name: "Sanchez".to_string(),
        age: Some(23),
        email: "some-address@example.org".to_string().into(), // .into() alone does not error in RustRover, but it does not work since this needs both to_string and wrapping in Option
        phone_number: None
    };

    println!("The person's first name is: {}", alfredo.first_name);
    println!("The person's age is: {:?}", alfredo.age);

    println!("Person as a whole: {alfredo:?}");

    // It seems to not be supported to call functions in the format string, "Full name: {alfredo.full_name()}" does not compile
    println!("Full name: {full_name}", full_name=alfredo.full_name());
}

/*
What is a struct in Rust, and what purpose does it serve? How is it different from other data types in the language?
    It assembles multiple values of potentially different types. Compared to tuple, the properties are named.

In the provided code, why is the `#[derive(Debug)]` attribute used for the `Person` struct? What benefit does it provide?
    Generates an implementation for the Debug trait which allows formatting using {:?}

Challenge questions see above
*/