#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }

    fn from_email(email: String) -> Option<Self> {
        let user_and_uri = email.split_once('@');
        let option_of_user = user_and_uri.map(|(username, uri)| User::new(username.to_string(), email.to_string(), uri.to_string()));
        option_of_user
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn update_uri(&mut self, uri: String) { self.uri = uri; }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);

    let user_from_email = User::from_email(new_user.email);
    match user_from_email {
        Some(mut user) => {
            user.update_uri("https://alfredodeza.com".to_string());
            println!("From email: {user:?}");
        },
        None => println!("user_from_email returned None"),
    }
}

/*
What is an associated function in Rust, and how does it differ from regular methods? In what scenarios would you choose to use an associated function instead of a regular method?
    Very similar to "static" functions in languages like C#. A function which can be called without having an instance of the type. Often used to create instances (factory function).

In the provided code, why is the associated function `new` used to create a new instance of the `User` struct? What advantages does it offer compared to creating a struct instance directly?
    It defaults `active` to true, removing the need to specify it to create a `User`.
 */