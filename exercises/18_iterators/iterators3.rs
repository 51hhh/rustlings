#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // 示例: 42 / 0
    DivideByZero,
    // `i64` 类型的唯一情况: `i64::MIN / -1` 因为结果是 `i64::MAX + 1`
    IntegerOverflow,
    // 示例: 5 / 2 = 2.5
    NotDivisible,
}

// 待办事项: 如果 `a` 能被 `b` 整除，计算 `a` 除以 `b` 的结果。
// 否则，返回合适的错误。
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }
    if a % b != 0 {
        return Err(DivisionError::NotDivisible);
    }
    Ok(a / b)
}

// 待办事项: 添加正确的返回类型并完成函数体。
// 期望输出: `Ok([1, 11, 1426, 3])`
fn result_with_list() -> Result<[i64; 4], DivisionError> {
    let numbers = [27, 297, 38502, 81];
    let division_results: Result<[i64; 4], DivisionError> = numbers.into_iter()
       .map(|n| divide(n, 27))
       .collect::<Result<Vec<i64>, DivisionError>>()
       .map(|vec| vec.try_into().unwrap());
    division_results
}

// 待办事项: 添加正确的返回类型并完成函数体。
// 期望输出: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() -> [Result<i64, DivisionError>; 4] {
    let numbers = [27, 297, 38502, 81];
    let division_results_vec: Vec<Result<i64, DivisionError>> = numbers.into_iter()
       .map(|n| divide(n, 27))
       .collect();
    division_results_vec.try_into().unwrap()
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
