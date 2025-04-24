// 在编译时，Rust 需要知道一个类型占用多少空间。这对于递归类型来说会成为问题，
// 因为一个递归类型的值可以将自身类型的另一个值作为其一部分。为了解决这个问题，
// 我们可以使用 `Box`——一种用于在堆上存储数据的智能指针，它还允许我们包装递归类型。
//
// 在这个练习中我们要实现的递归类型是 “cons 列表”，这是一种在函数式编程语言中经常出现的数据结构。
// cons 列表中的每个元素包含两个部分：当前元素的值和下一个元素。最后一个元素是一个称为 `Nil` 的值。


// 待办事项：在枚举定义中使用 `Box` 使代码能够编译。
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 待办事项：创建一个空的 cons 列表。
fn create_empty_list() -> List {
    List::Nil
}

// 待办事项：创建一个非空的 cons 列表。
fn create_non_empty_list() -> List {
    List::Cons(42, Box::new(List::Nil))
}

fn main() {
    println!("这是一个空的 cons 列表: {:?}", create_empty_list());
    println!(
        "这是一个非空的 cons 列表: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}


