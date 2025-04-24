fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");  // Moved this line outside the inner scope
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}