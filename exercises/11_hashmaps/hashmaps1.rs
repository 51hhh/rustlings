use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // 声明哈希表
    let mut basket = HashMap::new();

    // 已经给你两个香蕉了
    basket.insert(String::from("banana"), 2);

    // 往篮子里添加更多水果
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("mango"), 1);

    basket
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}


