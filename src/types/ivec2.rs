use std::hash::{Hash, Hasher};
use impl_ops::*;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl Hash for IVec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl_op_ex!(+ |a: &IVec2, b: &IVec2| -> IVec2 { IVec2::new(a.x + b.x, a.y + b.y) });
impl_op_ex!(- |a: &IVec2, b: &IVec2| -> IVec2 { IVec2::new(a.x - b.x, a.y - b.y) });
impl_op_ex!(* |a: &IVec2, b: &IVec2| -> IVec2 { IVec2::new(a.x * b.x, a.y * b.y) });
impl_op_ex!(/ |a: &IVec2, b: &IVec2| -> IVec2 { IVec2::new(a.x / b.x, a.y / b.y) });

impl_op_ex!(* |a: &IVec2, b: &i32| -> IVec2 { IVec2::new(a.x * b, a.y * b) });
impl_op_ex!(/ |a: &IVec2, b: &i32| -> IVec2 { IVec2::new(a.x / b, a.y / b) });

impl_op_ex!(- |a: &IVec2| -> IVec2 { IVec2::new(-a.x, -a.y) });

impl_op_ex!(+= |a: &mut IVec2, b: &IVec2| { a.x += b.x; a.y += b.y; });
impl_op_ex!(-= |a: &mut IVec2, b: &IVec2| { a.x -= b.x; a.y -= b.y; });
impl_op_ex!(*= |a: &mut IVec2, b: &IVec2| { a.x *= b.x; a.y *= b.y; });
impl_op_ex!(/= |a: &mut IVec2, b: &IVec2| { a.x /= b.x; a.y /= b.y; });

impl_op_ex!(*= |a: &mut IVec2, b: &i32| { a.x *= b; a.y *= b; });
impl_op_ex!(/= |a: &mut IVec2, b: &i32| { a.x /= b; a.y /= b; });

impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn as_array(&self) -> [i32; 2] {
        [self.x, self.y]
    }
}

pub fn ivec2(x: i32, y: i32) -> IVec2 {
    IVec2::new(x, y)
}



#[cfg(test)]
mod ivec2_test {
    use super::IVec2;

    #[test]
    fn addition_test() {
        let a = IVec2::new(1, 2);
        let b = IVec2::new(3, 4);

        let expected = IVec2::new(4, 6);

        assert_eq!(a + b, expected);
    }

    #[test]
    fn subtraction_test() {
        let a = IVec2::new(1, 2);
        let b = IVec2::new(3, 4);

        let expected = IVec2::new(-2, -2);

        assert_eq!(a - b, expected);
    }

    #[test]
    fn multiplication_test() {
        let a = IVec2::new(1, 2);
        let b = IVec2::new(3, 4);

        let expected = IVec2::new(3, 8);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn division_test() {
        let a = IVec2::new(1, 2);
        let b = IVec2::new(3, 4);

        let expected = IVec2::new(1 / 3, 2 / 4);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn scalar_multiplication_test() {
        let a = IVec2::new(1, 2);
        let b = 3;

        let expected = IVec2::new(3, 6);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn scalar_division_test() {
        let a = IVec2::new(1, 2);
        let b = 3;

        let expected = IVec2::new(1 / 3, 2 / 3);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn negation_test() {
        let a = IVec2::new(1, 2);

        let expected = IVec2::new(-1, -2);

        assert_eq!(-a, expected);
    }

    #[test]
    fn addition_assignment_test() {
        let mut a = IVec2::new(1, 2);
        let b = IVec2::new(3, 4);

        let expected = IVec2::new(4, 6);

        a += b;

        assert_eq!(a, expected);
    }

    #[test]
    fn subtraction_assignment_test() {
        let mut a = IVec2::new(1, 2);
        let b = IVec2::new(3, 4);

        let expected = IVec2::new(-2, -2);

        a -= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn multiplication_assignment_test() {
        let mut a = IVec2::new(1, 2);
        let b = IVec2::new(3, 4);

        let expected = IVec2::new(3, 8);

        a *= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn division_assignment_test() {
        let mut a = IVec2::new(1, 2);
        let b = IVec2::new(3, 4);

        let expected = IVec2::new(1 / 3, 2 / 4);

        a /= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_multiplication_assignment_test() {
        let mut a = IVec2::new(1, 2);
        let b = 3;

        let expected = IVec2::new(3, 6);

        a *= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_division_assignment_test() {
        let mut a = IVec2::new(1, 2);
        let b = 3;

        let expected = IVec2::new(1 / 3, 2 / 3);

        a /= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn as_array_test() {
        let a = IVec2::new(1, 2);

        let expected = [1, 2];

        assert_eq!(a.as_array(), expected);
    }
}
