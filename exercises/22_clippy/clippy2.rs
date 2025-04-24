fn main() {
    let mut res = 42;
    let option = Some(12);

    // 使用 if let 来处理 Option 类型
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
