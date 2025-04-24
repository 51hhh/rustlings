fn current_favorite_color() -> String {
    "blue".to_string()  // 将&str转换为String
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}