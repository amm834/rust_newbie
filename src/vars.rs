pub fn run() {
    // Variables are immutble like const in JS also define() in PHP

    let name = "Aung Myat Moe";
    let mut age = 16;
    println!("{} is {} years old in 2019.", name, age);

    age = 17;
    println!("{} is {} years old in 2020.", name, age);

    //Constant
    const INT: i32 = 1;
    println!("Int : {}", INT);

    // Assign Multiple Variables
    let (my_name, my_age) = ("Aung Myat Moe", 18);
    println!("{} is {} years old in 2020.", my_name, my_age);
}
