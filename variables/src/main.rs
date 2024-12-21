fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}")
    }
    println!("The value of x is; {x}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The spaces size is: {spaces}");

    // addition
    let sum = 5 + 10;
    println!("The sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    print!("The difference is: {difference}");
    // multiplication
    let product = 4 * 30;
    print!("The product is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    print!("The quotient is: {quotient}");
    let truncated = -5 / 3; 
    print!("The truncated is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    print!("The remainder is: {remainder}");

    let t = true;
    print!("The t is: {t}");
    let f: bool = false;
    print!("The f is: {f}");

    let c = 'z';
    print!("The c is: {c}");
    let z: char  = 'ℤ';
    print!("The z is: {z}");
    let heart_eyed_cat = '😻';
    print!("The heart_eyed_cat is: {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    print!("The value of x is: {x}");
    print!("The value of y is: {y}");
    print!("The value of z is: {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    print!("The value of five_hundred is: {five_hundred}");

    let six_point_four = x.1;
    print!("The value of six_point_four is: {six_point_four}");

    let one = x.2;
    print!("The value of one is: {one}");

    let mut x: (i32, i32)  = (1, 2);

    x.0 = 0;
    x.1 += 5;

    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {a:?}")

}
