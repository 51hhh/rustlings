// 本练习探索了 `Cow`（写时克隆）智能指针。它可以封装并提供对借用数据的不可变访问，
// 并且在需要进行变异或获取所有权时延迟克隆数据。该类型旨在通过 `Borrow` 特征处理一般的借用数据。

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // 如果尚未拥有数据，则克隆为一个向量
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // 由于 `input` 需要被变异，因此会发生克隆
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // 由于 `input` 不需要被变异，因此不会发生克隆
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Borrowed(_)));
    }

    #[test]
    fn owned_no_mutation() {
        // 我们也可以不使用 `&` 传递 `vec`，这样 `Cow` 就直接拥有它。
        // 在这种情况下，不会发生变异（所有数字已经是绝对值），因此也不会克隆。
        // 但结果仍然是被拥有的，因为它从未被借用或变异。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn owned_mutation() {
        // 当然，如果确实发生了变异（并非所有数字都是绝对值），情况也是如此。
        // 在这种情况下，`abs_all` 函数中对 `to_mut()` 的调用将返回对与之前相同数据的引用。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }
}


