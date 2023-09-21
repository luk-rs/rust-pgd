use num_traits::Num;

pub fn add<T>(x: T, y: T) -> T
    where T: Num {
    x + y
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use approx::{assert_relative_eq, RelativeEq};
    use crate::calculator::add;
    use num_traits::{PrimInt, Float};

    fn test_typed_add_i<T>(x: T, y: T, expected: T)
        where T: PrimInt + Debug {
        let result = add(x, y);
        assert_eq!(result,expected)
    }

    fn test_typed_add_f<T>(x: T, y: T, expected: T)
        where T: Float + RelativeEq<T> + Debug {
        let result = add(x, y);
        assert_relative_eq!(result,expected)
    }

    #[test]
    fn add_i8s() {
        test_typed_add_i::<i8>(1, 2, 3);
    }

    #[test]
    fn add_i16s() {
        test_typed_add_i::<i16>(1, 2, 3);
    }

    #[test]
    fn add_i32s() {
        test_typed_add_i::<i32>(1, 2, 3);
    }

    #[test]
    fn add_i64s() {
        test_typed_add_i::<i64>(1, 2, 3);
    }

    #[test]
    fn add_i128s() {
        test_typed_add_i::<i128>(1, 2, 3);
    }

    #[test]
    fn add_u8s() {
        test_typed_add_i::<u8>(1, 2, 3);
    }

    #[test]
    fn add_u16s() {
        test_typed_add_i::<u16>(1, 2, 3);
    }

    #[test]
    fn add_u32s() {
        test_typed_add_i::<u32>(1, 2, 3);
    }

    #[test]
    fn add_u64s() {
        test_typed_add_i::<u64>(1, 2, 3);
    }

    #[test]
    fn add_f32s() {
        test_typed_add_f::<f32>(1.1, 2.2, 3.3);
    }

    #[test]
    fn add_f64s() {
        test_typed_add_f::<f64>(1.1, 2.2, 3.3);
    }

}
