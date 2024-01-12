fn main() {
    println!("Hello, world!");

    another_function();

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {} y is: {}", x,y);

    println!("return is: {}", returning_function(10));
}

fn another_function() {
    println!("Another function.");
}

fn returning_function(x: i32) -> i32{//must put return type
    x + 5
}