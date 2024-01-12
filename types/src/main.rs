fn main() {
    /*
       scalar

       int
       signed unsigned 8 - 64
       arch -> isize, usize depends on the architecture

       Decimal 98_222
       Hex 0xff
       Octal 0o77
       Binary 0b1111_0000
       Byte(u8 only) b'A'


    */
    let x = 2.0;
    let y: f32 = 3.0;

    println!("x: {} y: {}", x, y);

    /*
       character

       using Unicode Scalar

    */

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c: {} z: {} cat: {}", c, z, heart_eyed_cat);

    /*
      combined types

      tuple
      array

    */

    let t: (i32, f64, u8) = (500, 6.4, 1);

    println!("t.0: {} t.1: {} t.2: {}", t.0, t.1, t.2);

    let a = [1, 2, 3, 4, 5];
    let index = 3;

    let element = a[index];

    println!("The value of element is: {}", element);
}
