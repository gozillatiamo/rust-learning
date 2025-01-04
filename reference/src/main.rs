use bigdecimal::BigDecimal;
use std::str::FromStr;

fn main() {
    println!("{}", to_real(1254590000));

    let mut x: Box<i32> = Box::new(1);
    println!("Box<i32> x is: {x}");

    let a: i32 = *x;
    println!("i32 a is: {a}");

    *x += 1;
    println!("*x += 1 is: {}", x);

    let r1: &Box<i32> = &x;
    println!("r1 is: {}", r1);

    let b: i32 = **r1;
    println!("i32 b is: {}", b);

    let r2: &i32 = &*x;
    println!("r2 is: {}", r2);

    let c: i32 = *r2;
    println!("i32 c is: {}", c);
    // let first = &name[0];

    let mut a = [0, 1, 2, 3];
    let x = &mut a[1] as *mut i32;
    let y = &a[2] as *const i32;
    unsafe {
        *x += *y;
    } // DO NOT DO THIS unless you know what you're doing!
    println!("{}", x);
}

fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i];
    *n = v[i - 1]
}

pub const CONVERT_RATIO: i64 = 1000000;

pub fn to_real(amount: i64) -> f64 {
    let amount_decimal = BigDecimal::from(amount);
    let convert_ratio_decimal = BigDecimal::from(CONVERT_RATIO);
    let result = amount_decimal / convert_ratio_decimal;
    // Ensures the result is rounded to 2 decimal places to maintain currency precision
    let rounded_result = result.with_scale(2);
    f64::from_str(&rounded_result.to_string()).unwrap()
}
