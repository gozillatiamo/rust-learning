pub mod hosting;

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}

fn update_vec() {
    let mut v = Vec::new();
    let s = String::from("Hello ");
    v.push(s);
    v[0].push_str("world");
    println!("original: {}", s);
    println!("new: {}", v[0]);
}
