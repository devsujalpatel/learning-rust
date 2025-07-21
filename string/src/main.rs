fn main() {
   let ax: &str = "Sujal Patel"; // can change space at runtime 

   println!("{}", ax);

   let char1  = ax.chars().nth(0);

   println!("{}", char1.unwrap());

}

// to change

// string -> slower, hard
// number (intergers, floats) -> easy, faster
// boolean -> easy, faster