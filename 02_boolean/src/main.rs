fn main() {
    let is_male: bool = false;
    let is_above_18: bool = true;

    if is_male {
        println!("You are a male");
    } else {
        println!("You are a female");
    }
    if is_male && is_above_18 {
        println!("You are a male and above 18");
    } else {
        println!("You are not a male or above 18");
    }
    
}

