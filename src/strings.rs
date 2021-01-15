pub fn run() {
    let mut greet = String::from("Hello");
    greet.push(' ');
    greet.push_str("World");
    println!("{}", greet);
    println!("{}", greet.len());
}
