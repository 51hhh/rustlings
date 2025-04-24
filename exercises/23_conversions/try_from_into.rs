#![allow(clippy::useless_vec)]
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// 我们将使用这个错误类型进行 TryFrom 转换
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // 切片长度不正确
    BadLen,
    // 整数转换错误
    IntConversion,
}

// 元组实现
// 正确的 RGB 颜色值必须是 0..=255 范围内的整数
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (r, g, b) = tuple;
        if r < 0 || r > 255 || g < 0 || g > 255 || b < 0 || b > 255 {
            return Err(IntoColorError::IntConversion);
        }
        Ok(Color {
            red: r as u8,
            green: g as u8,
            blue: b as u8,
        })
    }
}

// 数组实现
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let (r, g, b) = (arr[0], arr[1], arr[2]);
        if r < 0 || r > 255 || g < 0 || g > 255 || b < 0 || b > 255 {
            return Err(IntoColorError::IntConversion);
        }
        Ok(Color {
            red: r as u8,
            green: g as u8,
            blue: b as u8,
        })
    }
}

// 切片实现
// 这个实现需要检查切片长度
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        let (r, g, b) = (slice[0], slice[1], slice[2]);
        if r < 0 || r > 255 || g < 0 || g > 255 || b < 0 || b > 255 {
            return Err(IntoColorError::IntConversion);
        }
        Ok(Color {
            red: r as u8,
            green: g as u8,
            blue: b as u8,
        })
    }
}

fn main() {
    // 使用 try_from 函数
    let c1 = Color::try_from((183, 65, 14));
    println!("{c1:?}");

    // 由于为 Color 实现了 TryFrom，我们可以使用 TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{c2:?}");

    let v = vec![183, 65, 14];
    // 对于切片，我们应该使用 try_from 函数
    let c3 = Color::try_from(&v[..]);
    println!("{c3:?}");

    // 或者将切片放在圆括号内并使用 try_into
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{c4:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use IntoColorError::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(Color::try_from((256, 1000, 10000)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(Color::try_from((-1, -10, -256)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_sum() {
        assert_eq!(Color::try_from((-1, 255, 255)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(BadLen));
    }

    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(BadLen));
    }
}


