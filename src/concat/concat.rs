fn by_moving() {
    let hello = "Hello ".to_string();
    let world = "world!";

    let hello_world = hello + world;
    println!("{}", hello_world);
}

fn by_cloning() {
    let hello = "Hello ".to_string();
    let world = "world!";

    let hello_world = hello.clone() + world;
    println!("{}", hello_world);
}

fn by_mutating() {
    let mut hello = "Hello ".to_string();
    let world = "world!";

    hello.push_str(world);
    println!("{}", hello);
}

// Borrowing
fn pluralize(s: &String) -> String {
    // let mut p = s.clone();
    // p.push_str("s");
    // p
    s.to_owned() + "s"
}

fn main() {
    by_moving();
    by_cloning();
    by_mutating();
    let pig = String::from("pig");
    println!("Prual `pig` is {}", pluralize(&pig));
    println!("pig is still {}", pig);
}