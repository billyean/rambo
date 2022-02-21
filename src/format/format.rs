use std::f32::consts::PI;

fn main() {
    let colour = "red";
    let favourite = format!("My favourite colour is {}", colour);
    println!("{}", favourite);

    let hello = "hello ";
    let world = "world!";
    let hello_world = format!("{}{}", hello, world);
    println!("{}", hello_world);

    let favourite_num = format!("My favourite number is {}", PI);
    println!("{}", favourite_num); 

    let no_no_no_stop = format!("{0}, {0}, {0}, {1}!", "no", "stop");
    println!("{}", no_no_no_stop);

    let introduction = format!(
        "My name is {surname}, {forename} {surname}",
        surname="Yan",
        forename="Tristan"
        );
    println!("{}", introduction)
}