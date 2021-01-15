pub fn run() {
  let age: u8 = 17;
  let check_id: bool = false;
  let is_know_person: bool = true;
  // If else if else condition
  if age >= 18 && check_id || is_know_person {
    println!("Bartender: What do you want to drink?");
  }else if age < 18 && check_id {
    println!("Bartender: You can't drink.");
  }else {
    println!("Bartender: I'll need ID.");
  }
  // short hand
  let is_of_age = if age >= 18 {
    "You can drink."
  }else {
    "You can't"
  };
  println!("Bartender: {}", is_of_age);
}