use std::hash::{Hash, Hasher};
use impl_ops::*;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IVec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl Hash for IVec4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
        self.w.hash(state);
    }
}

impl_op_ex!(+ |a: &IVec4, b: &IVec4| -> IVec4 { IVec4::new(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w) });
impl_op_ex!(- |a: &IVec4, b: &IVec4| -> IVec4 { IVec4::new(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w) });
impl_op_ex!(* |a: &IVec4, b: &IVec4| -> IVec4 { IVec4::new(a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w) });
impl_op_ex!(/ |a: &IVec4, b: &IVec4| -> IVec4 { IVec4::new(a.x / b.x, a.y / b.y, a.z / b.z, a.w / b.w) });

impl_op_ex!(* |a: &IVec4, b: &i32| -> IVec4 { IVec4::new(a.x * b, a.y * b, a.z * b, a.w * b) });
impl_op_ex!(/ |a: &IVec4, b: &i32| -> IVec4 { IVec4::new(a.x / b, a.y / b, a.z / b, a.w / b) });

impl_op_ex!(- |a: &IVec4| -> IVec4 { IVec4::new(-a.x, -a.y, -a.z, -a.w) });

impl_op_ex!(+= |a: &mut IVec4, b: &IVec4| { a.x += b.x; a.y += b.y; a.z += b.z; a.w += b.w; });
impl_op_ex!(-= |a: &mut IVec4, b: &IVec4| { a.x -= b.x; a.y -= b.y; a.z -= b.z; a.w -= b.w; });
impl_op_ex!(*= |a: &mut IVec4, b: &IVec4| { a.x *= b.x; a.y *= b.y; a.z *= b.z; a.w *= b.w; });
impl_op_ex!(/= |a: &mut IVec4, b: &IVec4| { a.x /= b.x; a.y /= b.y; a.z /= b.z; a.w /= b.w; });

impl_op_ex!(*= |a: &mut IVec4, b: &i32| { a.x *= b; a.y *= b; a.z *= b; a.w *= b; });
impl_op_ex!(/= |a: &mut IVec4, b: &i32| { a.x /= b; a.y /= b; a.z /= b; a.w /= b; });

impl IVec4 {
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self { x, y, z, w }
    }

    pub fn as_array(&self) -> [i32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

pub fn ivec4(x: i32, y: i32, z: i32, w: i32) -> IVec4 {
    IVec4::new(x, y, z, w)
}

#[cfg(test)]
mod ivec4_test {
    use super::IVec4;

    #[test]
    fn addition_test() {
        let a = IVec4::new(1, 2, 3, 4);
        let b = IVec4::new(5, 6, 7, 8);

        let expected = IVec4::new(6, 8, 10, 12);

        assert_eq!(a + b, expected);
    }

    #[test]
    fn subtraction_test() {
        let a = IVec4::new(1, 2, 3, 4);
        let b = IVec4::new(5, 6, 7, 8);

        let expected = IVec4::new(-4, -4, -4, -4);

        assert_eq!(a - b, expected);
    }

    #[test]
    fn multiplication_test() {
        let a = IVec4::new(1, 2, 3, 4);
        let b = IVec4::new(5, 6, 7, 8);

        let expected = IVec4::new(5, 12, 21, 32);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn division_test() {
        let a = IVec4::new(1, 2, 3, 4);
        let b = IVec4::new(5, 6, 7, 8);

        let expected = IVec4::new(1 / 5, 2 / 6, 3 / 7, 4 / 8);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn scalar_multiplication_test() {
        let a = IVec4::new(1, 2, 3, 4);
        let b = 5;

        let expected = IVec4::new(5, 10, 15, 20);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn scalar_division_test() {
        let a = IVec4::new(1, 2, 3, 4);
        let b = 5;

        let expected = IVec4::new(1 / 5, 2 / 5, 3 / 5, 4 / 5);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn negation_test() {
        let a = IVec4::new(1, 2, 3, 4);

        let expected = IVec4::new(-1, -2, -3, -4);

        assert_eq!(-a, expected);
    }

    #[test]
    fn addition_assignment_test() {
        let mut a = IVec4::new(1, 2, 3, 4);
        let b = IVec4::new(5, 6, 7, 8);

        let expected = IVec4::new(6, 8, 10, 12);

        a += b;

        assert_eq!(a, expected);
    }

    #[test]
    fn subtraction_assignment_test() {
        let mut a = IVec4::new(1, 2, 3, 4);
        let b = IVec4::new(5, 6, 7, 8);

        let expected = IVec4::new(-4, -4, -4, -4);

        a -= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn multiplication_assignment_test() {
        let mut a = IVec4::new(1, 2, 3, 4);
        let b = IVec4::new(5, 6, 7, 8);

        let expected = IVec4::new(5, 12, 21, 32);

        a *= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn division_assignment_test() {
        let mut a = IVec4::new(1, 2, 3, 4);
        let b = IVec4::new(5, 6, 7, 8);

        let expected = IVec4::new(1 / 5, 2 / 6, 3 / 7, 4 / 8);

        a /= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_multiplication_assignment_test() {
        let mut a = IVec4::new(1, 2, 3, 4);
        let b = 5;

        let expected = IVec4::new(5, 10, 15, 20);

        a *= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_division_assignment_test() {
        let mut a = IVec4::new(1, 2, 3, 4);
        let b = 5;

        let expected = IVec4::new(1 / 5, 2 / 5, 3 / 5, 4 / 5);

        a /= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn as_array_test() {
        let a = IVec4::new(1, 2, 3, 4);

        let expected = [1, 2, 3, 4];

        assert_eq!(a.as_array(), expected);
    }
}
