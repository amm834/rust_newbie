pub fn run(){

// Print Line
println!("I' newbie in Rust.");

// Basic Formatting 
println!("{}",1); // String literal {} need

// Arguments

println!("{} is a student.","Aung Myat Moe");

// Arguments Passing
println!("{} is a {}.","Aung Myat Moe","Web Dev");

//Posistional Arguments
println!("{0} is a {1} and {0} love {2}.","Aung Myat Moe","Web Dev","Programming and Physics");

// Mamed arguments
println!("My name is {name} and age is {age}.",name="Aung Myat Moe",age=17);

// Tuple or Place Holder Trait
println!("Binary Type : {:b} , Hex Type : {:x} , Ocatal Type : {:o} .",10,10,10);

// Debug

println!("{:?}",(1,"Hello World!",true));

//Basic Math

println!("10 + 10 = {}",10+10);

}