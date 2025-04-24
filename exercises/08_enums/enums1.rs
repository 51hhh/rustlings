#[derive(Debug)]
enum Message {
    Resize,        // 调整大小消息
    Move,          // 移动消息
    Echo,          // 回显消息
    ChangeColor,   // 改变颜色消息
    Quit,          // 退出消息
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}