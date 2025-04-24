// 在这个练习中，你将学习到迭代器所能提供的一些独特优势。

// 待办事项：完成 `capitalize_first` 函数。
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            // 将首字符转换为大写并收集为字符串
            let mut result = first.to_uppercase().collect::<String>();
            // 将剩余字符收集为字符串并追加到结果中
            result.push_str(&chars.collect::<String>());
            result
        }
    }
}

// 待办事项：将 `capitalize_first` 函数应用于字符串切片的切片。
// 返回一个字符串向量。
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 使用 iter 方法遍历切片，对每个元素应用 capitalize_first 函数，然后收集结果
    words.iter().map(|word| capitalize_first(word)).collect()
}

// 待办事项：再次将 `capitalize_first` 函数应用于字符串切片的切片。
// 返回一个单一字符串。
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // 使用 iter 方法遍历切片，对每个元素应用 capitalize_first 函数，然后收集结果
    words.iter().map(|word| capitalize_first(word)).collect()
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
