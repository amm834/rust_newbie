pub fn run() {

//i32 is default
println!("Max of i32 = {}",std::i32::MAX);
println!("Max of i64 = {}",std::i64::MAX);
//println!("Max of f64 = {}",std::f64::MAX);
println!("Max of u8 = {}",std::u8::MAX);

let x:i32 = 100;
let y:i64 = 200;
let z:f64 = 2.5;
let is_active:bool = true;
let is_true:bool = 6>5;
let emoji = "\u{1F600}";
let a1 = 'a';

println!("{:?}",(x,y,z,is_active,is_true,emoji,a1));
 
}