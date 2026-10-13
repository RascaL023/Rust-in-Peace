mod user;

use std::io::{self, Write};

use crate::user::User;

fn main() {
    let name = {
        let mut input = String::new();
        print_label_for_input("Name: ");
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };

    let username = {
        let mut input = String::new();
        print_label_for_input("Username: ");
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };

    let password = {
        let mut input = String::new();
        print_label_for_input("Password: ");
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };

    let user = User::new(
        name,
        username,
        password,
    );

    if user.is_empty() {
        println!("User field must be filled");
        return;
    }

    println!("Name: {}", user.name);
    println!("Username: {}", user.username);
}

fn print_label_for_input(label: &str) {
    print!("{label}");
    io::stdout().flush().unwrap();
}
