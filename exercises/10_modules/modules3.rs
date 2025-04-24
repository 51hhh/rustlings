// 你可以使用 `use` 关键字将来自任何模块（特别是标准库中的模块）的模块路径引入到你的作用域中。

// 待办事项：将 `std::time` 模块中的 `SystemTime` 和 `UNIX_EPOCH` 引入到你的作用域中。
// 如果你能用一行代码完成，会有额外的风格加分！
// use ???;

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC 距离现在是 {} 秒前!", n.as_secs()),
        Err(_) => panic!("系统时间在 UNIX 纪元之前!"),
    }
}
