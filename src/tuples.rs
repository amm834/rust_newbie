pub fn run() {
  let person: (&str, i8, &str) = ("Aung Myat Moe", 17, "Student");
  println!("{} is a {} years old {}.",person.0,person.1,person.2);
}