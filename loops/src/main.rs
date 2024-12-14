fn main() {
    try_break();

    loop_labels();
}

fn try_break() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("again!");

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

//  Loop Labels
fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count  = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
