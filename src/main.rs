// use std::io;
// use rand::Rng;
//
// fn main() {
//     enter_number();
//     // return_numbers();
//     println!("Exit!");
//     // plus_numbers();
// }
//
// fn enter_number() {
//     println!("Guess the number!");
//
//     let secret_number = rand::thread_rng().gen_range(1..101);
//
//     println!("The secret number is: {}", secret_number);
//
//     println!("Please input your guess");
//
//     let mut guess = String::new();
//
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
//
//     println!("You guessed: {}", guess);
// }
//
// fn return_numbers() {
//     let x = 5;
//     let y = 10;
//
//     println!("x = {} and y = {}", x, y)
// }
// //
// // fn plus_numbers() {
// //     let mut x = 23.4;
// //     let mut y = 14.5;
// //     return x + y;
// // }
// // fn stroka(txt) {
// //     let mut foo = txt;
// //     println!(foo);
// // }

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
