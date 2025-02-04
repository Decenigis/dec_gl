use std::hash::{Hash, Hasher};
use impl_ops::*;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Hash for IVec3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl_op_ex!(+ |a: &IVec3, b: &IVec3| -> IVec3 { IVec3::new(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(- |a: &IVec3, b: &IVec3| -> IVec3 { IVec3::new(a.x - b.x, a.y - b.y, a.z - b.z) });
impl_op_ex!(* |a: &IVec3, b: &IVec3| -> IVec3 { IVec3::new(a.x * b.x, a.y * b.y, a.z * b.z) });
impl_op_ex!(/ |a: &IVec3, b: &IVec3| -> IVec3 { IVec3::new(a.x / b.x, a.y / b.y, a.z / b.z) });

impl_op_ex!(* |a: &IVec3, b: &i32| -> IVec3 { IVec3::new(a.x * b, a.y * b, a.z * b) });
impl_op_ex!(/ |a: &IVec3, b: &i32| -> IVec3 { IVec3::new(a.x / b, a.y / b, a.z / b) });

impl_op_ex!(- |a: &IVec3| -> IVec3 { IVec3::new(-a.x, -a.y, -a.z) });

impl_op_ex!(+= |a: &mut IVec3, b: &IVec3| { a.x += b.x; a.y += b.y; a.z += b.z; });
impl_op_ex!(-= |a: &mut IVec3, b: &IVec3| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });
impl_op_ex!(*= |a: &mut IVec3, b: &IVec3| { a.x *= b.x; a.y *= b.y; a.z *= b.z; });
impl_op_ex!(/= |a: &mut IVec3, b: &IVec3| { a.x /= b.x; a.y /= b.y; a.z /= b.z; });

impl_op_ex!(*= |a: &mut IVec3, b: &i32| { a.x *= b; a.y *= b; a.z *= b; });
impl_op_ex!(/= |a: &mut IVec3, b: &i32| { a.x /= b; a.y /= b; a.z /= b; });

impl IVec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn as_array(&self) -> [i32; 3] {
        [self.x, self.y, self.z]
    }
}

pub fn ivec3(x: i32, y: i32, z: i32) -> IVec3 {
    IVec3::new(x, y, z)
}

#[cfg(test)]
mod ivec3_test {
    use super::IVec3;

    #[test]
    fn addition_test() {
        let a = IVec3::new(1, 2, 3);
        let b = IVec3::new(4, 5, 6);

        let expected = IVec3::new(5, 7, 9);

        assert_eq!(a + b, expected);
    }

    #[test]
    fn subtraction_test() {
        let a = IVec3::new(1, 2, 3);
        let b = IVec3::new(4, 5, 6);

        let expected = IVec3::new(-3, -3, -3);

        assert_eq!(a - b, expected);
    }

    #[test]
    fn multiplication_test() {
        let a = IVec3::new(1, 2, 3);
        let b = IVec3::new(4, 5, 6);

        let expected = IVec3::new(4, 10, 18);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn division_test() {
        let a = IVec3::new(1, 2, 3);
        let b = IVec3::new(4, 5, 6);

        let expected = IVec3::new(1 / 4, 2 / 5, 3 / 6);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn scalar_multiplication_test() {
        let a = IVec3::new(1, 2, 3);
        let b = 4;

        let expected = IVec3::new(4, 8, 12);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn scalar_division_test() {
        let a = IVec3::new(1, 2, 3);
        let b = 4;

        let expected = IVec3::new(1 / 4, 2 / 4, 3 / 4);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn negation_test() {
        let a = IVec3::new(1, 2, 3);

        let expected = IVec3::new(-1, -2, -3);

        assert_eq!(-a, expected);
    }

    #[test]
    fn addition_assignment_test() {
        let mut a = IVec3::new(1, 2, 3);
        let b = IVec3::new(4, 5, 6);

        let expected = IVec3::new(5, 7, 9);

        a += b;

        assert_eq!(a, expected);
    }

    #[test]
    fn subtraction_assignment_test() {
        let mut a = IVec3::new(1, 2, 3);
        let b = IVec3::new(4, 5, 6);

        let expected = IVec3::new(-3, -3, -3);

        a -= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn multiplication_assignment_test() {
        let mut a = IVec3::new(1, 2, 3);
        let b = IVec3::new(4, 5, 6);

        let expected = IVec3::new(4, 10, 18);

        a *= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn division_assignment_test() {
        let mut a = IVec3::new(1, 2, 3);
        let b = IVec3::new(4, 5, 6);

        let expected = IVec3::new(1 / 4, 2 / 5, 3 / 6);

        a /= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_multiplication_assignment_test() {
        let mut a = IVec3::new(1, 2, 3);
        let b = 4;

        let expected = IVec3::new(4, 8, 12);

        a *= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_division_assignment_test() {
        let mut a = IVec3::new(1, 2, 3);
        let b = 4;

        let expected = IVec3::new(1 / 4, 2 / 4, 3 / 4);

        a /= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn as_array_test() {
        let a = IVec3::new(1, 2, 3);

        let expected = [1, 2, 3];

        assert_eq!(a.as_array(), expected);
    }
}
