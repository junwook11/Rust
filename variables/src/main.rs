fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = x * 10;
    println!("The value of x is: {}", x);

    /*
        mut 과 shadowing의 차이

        mut은 가변성 변수를 만들어 값이 변하는 것을 허용
        shadowing은 새 변수를 선언하여 유형을 변경할 수 있음

        ex) 
        shadowing
        let spaces = "   ";
        let spaces = spaces.len(); => 가능

        mut
        let mut spaces = "   ";
        spaces = spaces.len(); => 불가능, 변수의 유형은 변경불가

     */
}
