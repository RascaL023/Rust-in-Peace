// use std::io::{self, Write};
//
// fn mutability() {
//     // Default variable Rust itu immutable
//     let x = 20;
//     // x = 10; -> Bakal error
//     println!("Isi x: {x}");
//
//     let mut y = 20; // `mut` -> artinya mutable, bisa diubah nantinya
//     println!("Isi y awal: {y}");
//
//     y = 10;
//     println!("Isi y akhir: {y}");
// }
//
// fn shadowing() {
//     let x = 5;
//     let x = x * 2;
//
//     {
//         let x = x - 5;
//         println!("Nilai x di dalam scope: {x}")
//     }
//
//     println!("Nilai x di luar scope: {x}")
// }
//
// // fn simple_io() {
// //     print!("Input name: ");
// //     default stdout Rust itu Line Buffering, jika ada newline maka flush terjadi, karena tidak
// //     ada newline, maka output tertahan di buffer hingga ada trigger yang memaksa flus
// //
// //     let mut name = String::new();
// //     io::stdin().read_line(&mut name).unwrap();
// //
// //     println!("Name: {name}");
// // }
//
// fn print_label(label: &str) {
//     print!("{label}");
//     io::stdout().flush().unwrap();
// }
//
// fn simple_io() {
//     let mut name = String::new();
//
//     print_label("Input name: ");
//     io::stdin().read_line(&mut name).unwrap();
//     name = name.trim().to_string();
//
//     print_label("Input age: ");
//     let mut age_input = String::new();
//     io::stdin().read_line(&mut age_input).unwrap();
//     let age: i32 = age_input.trim().parse().unwrap();
//
//     println!("Name  : {name}");
//     println!("Age   : {age}");
// }
//
// fn main() {
//     println!("\nMutability");
//     mutability();
//
//     println!("\nShadowing");
//     shadowing();
//
//     println!("\nSimple IO");
//     simple_io();
// }


// use std::{io::{self, Write}};

// ============================= ASSIGNMENT
// fn print_flush(label: &str) {
//     print!("{label}");
//     io::stdout().flush().unwrap();
// }

fn count_scores(scores: &[u32]) -> (u32, u32, u32) {
    let mut total: u32 = 0;
    let mut max: u32 = scores[0];

    for score in scores {
        total += score;
        if max < *score {
            max = *score
        }
    }

    (total, total / scores.len() as u32, max)
}

fn get_grade_by_average(average: u32) -> &'static str {
    let mut grade = "D";
    if average >= 90 {
        grade = "A";
    } else if average >= 80 {
        grade = "B";
    } else if average >= 70 {
        grade = "C";
    }

    grade
}

fn main() {
    // let mut name = String::new();
    // print_flush("Name: ");
    // io::stdin().read_line(&mut name).unwrap();
    // let name = name.trim();
    let name = String::from("   Siapa Aja   ");
    let name = name.trim();

    // let mut initials = String::new();
    // print_flush("Initials: ");
    // io::stdin().read_line(&mut initials).unwrap();
    // let initials = initials.trim();

    // let (size, chars) = name.char_indices();
    // let initials = String::new();
    // let initials = initials.add(&name[0..1]);
    // let initials = initials.add(".");
    // let initials = initials.add(&name[5..6]);
    let mut initials = String::new();
    initials.push_str(&name[0..1]);
    initials.push_str(".");
    let space = name.find(" ").unwrap();
    initials.push_str(&name[space+1..space+2]);

    let scores: [u32; 6] = [70, 85, 90, 64, 97, 88];
    let (total, average, max) = count_scores(&scores);
    let grade: &str = get_grade_by_average(average);

    println!("Name: {name}");
    println!("Initials: {initials}");
    println!("Scores: {:?}", scores);
    println!("Total: {total} | Average: {average} | Max: {max}");
    println!("Grade: {grade}");

    let summary = format!("Summary: {name} scored {average} on average ({grade}) with {} subjects", scores.len());
    println!("{summary}");
}
