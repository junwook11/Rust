use std::io;

fn main() {
    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Failed to read line");

    match number.trim().parse::<u32>(){
        Ok(num) => {
            if num < 5 {
                println!("condition was true");
            } else {
                println!("condition was false");
            }
        },
        Err(_) => {
            println!("Err");
        }
    };

    let a = [10, 20, 30, 40, 50];

    for (index,element) in a.iter().enumerate() {
        println!("the index is: {} value is: {}", index,element);
    }
}
