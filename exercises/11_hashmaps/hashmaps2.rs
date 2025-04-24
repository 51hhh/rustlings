use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // 只插入篮子里没有的水果
        if !basket.contains_key(&fruit) {
            basket.insert(fruit, 1);
        }
    }
}

fn main() {
    // 可以在这里进行实验性代码
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试代码保持不变...
}