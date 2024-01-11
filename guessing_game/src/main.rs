extern crate rand;
/*  
    외부 crate인 rand를 가져옴. 
    Cargo.toml의 dependencies 부분에 추가 후 cargo build
*/

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();//new()는 비어있는 변수를 만들 때 사용
    
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        //expect() -> Err일 때 넘겨준 메세지를 출력
    
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
