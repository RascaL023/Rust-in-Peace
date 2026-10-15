use std::io::{self, Write};

#[derive(Debug)]
enum AppStatus {
    Starting,
    Running,
    Stopped,
}

fn print_status(status: AppStatus) {
    match status {
        AppStatus::Starting => println!("Aplikasi sedang starting"),
        AppStatus::Running => println!("Aplikasi sedang berjalan"),
        AppStatus::Stopped => println!("Aplikasi berhenti"),
    }
}

fn menu() -> AppStatus {
    println!("Select Menu: ");
    println!("1. {:?}", AppStatus::Starting);
    println!("2. {:?}", AppStatus::Running);
    println!("3. {:?}", AppStatus::Stopped);
    print!("Input: "); io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: u8 = input.trim().parse().unwrap();

    if input == 1 { AppStatus::Starting }
    else if input == 2 { AppStatus::Running }
    else { AppStatus::Stopped }
}

fn main() {
    let status = menu();
    print_status(status);
}
