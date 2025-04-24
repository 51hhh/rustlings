// 在这个练习中，我们有一个名为 `numbers` 的 `u32` 类型的 `Vec`，其中的值范围是 0 到 99。
// 我们希望同时在 8 个不同的线程中使用这组数字。每个线程将获取每隔 8 个值且带有偏移量的数字的总和。
//
// 第一个线程（偏移量为 0），将计算 0, 8, 16, … 的总和
// 第二个线程（偏移量为 1），将计算 1, 9, 17, … 的总和
// 第三个线程（偏移量为 2），将计算 2, 10, 18, … 的总和
// …
// 第八个线程（偏移量为 7），将计算 7, 15, 23, … 的总和
//
// 每个线程应该拥有一个指向数字向量的引用计数指针。但是 `Rc` 不是线程安全的。因此，我们需要使用 `Arc`。
//
// 不要被线程的创建和连接方式所干扰。我们将在后面关于线程的练习中练习这些内容。


// 不要更改下面的行。
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // 待办事项：使用 `Arc` 定义 `shared_numbers`。
    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // 待办事项：使用 `shared_numbers` 定义 `child_numbers`。
        let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}


