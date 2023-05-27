use std::{cmp::Ordering, io};
use test_rust_crate::{SomeTrait, hashmaps::build_scores_table};

fn foo(x: i32) -> Result<i32, String> {
    if x == 0 {
        Err("zero".into())
    } else if x > 3 {
        Ok(x + 1)
    } else {
        Ok(x - 1)
    }
}

fn bar() {
    let t = build_scores_table("string".to_string());
    t.is_empty();
}

fn main() {
    println!("Guess the number!");
    let unused_var = vec![1, 2, 3, 4, 5]
        .into_iter()
        .map(|x| foo(x - 5))
        .collect::<Result<Vec<_>, _>>();
    let secret_number = 4;

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

        println!("You guessed: {guess}");

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
