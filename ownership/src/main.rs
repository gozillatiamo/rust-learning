fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");

    let b = Box::new(0);
    let b2 = b;
    println!("{}", b);
    move_a_box(b2);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn move_a_box(_b: Box<i32>) {}
