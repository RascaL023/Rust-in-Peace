use std::{
    io::{self, Write},
    process::exit,
};

fn get_index_by_target(arr: &[u32], target: &u32) -> Option<usize> {
    for (idx, &value) in arr.iter().enumerate() {
        if value == *target {
            return Some(idx);
        }
    }

    None
}

fn print_label(label: &str) {
    print!("{label}");
    io::stdout().flush().unwrap();
}

fn parse_to_int(inp: &str) -> Result<u32, String> {
    match inp.parse::<u32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("{inp} is not valid u32")),
    }
}

fn main() {
    let arr: [u32; 3] = [10, 20, 25];
    println!("Array: {:?}", arr);

    let target: u32 = {
        print_label("Input target: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        match parse_to_int(input) {
            Ok(num) => num,
            Err(msg) => {
                println!("Failed to parse input: {}", msg);
                exit(1)
            }
        }
    };

    if let Some(idx) = get_index_by_target(&arr, &target) {
        println!("Target {:?} ditemukkan di index {:?}", target, idx);
        return;
    }

    println!("Target {:?} tidak ditemukkan", target);
}
