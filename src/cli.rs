use std::env;

pub fn  run(){
  let args:Vec<String> = env::args().collect();
  let cmd = args[1].clone();

if cmd == "hello"{
  println!("hello");
}else if cmd == "hi"{
  println!("hi")
}else{
  println!("command is wrong");
}


}