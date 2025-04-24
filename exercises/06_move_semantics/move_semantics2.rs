fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(88);
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let mut vec0 = vec![22, 44, 66];
        let vec1 = vec0.clone(); // Clone vec0 to keep it accessible

        let mut vec2 = vec1;
        fill_vec(&mut vec2);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec2, [22, 44, 66, 88]);
    }
}