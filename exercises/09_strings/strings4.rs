fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    string_slice("blue");  // string literal is &str
    
    string("red".to_string());  // to_string() creates String
    
    string(String::from("hi"));  // String::from creates String
    
    string("rust is fun!".to_owned());  // to_owned() creates String
    
    string("nice weather".into());  // into() converts to String
    
    string(format!("Interpolation {}", "Station"));  // format! creates String
    
    string_slice(&String::from("abc")[0..1]);  // string slice is &str
    
    string_slice(" hello there ".trim());  // trim() returns &str
    
    string("Happy Monday!".replace("Mon", "Tues"));  // replace() creates String
    
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());  // to_lowercase() creates String
}