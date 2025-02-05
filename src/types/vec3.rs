use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use impl_ops::*;
use std::ops;
use crate::math::Normalise;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Hash for Vec3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
    }
}

impl Normalise for Vec3 {
    fn normalise(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z) });
impl_op_ex!(* |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z) });
impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x / b.x, a.y / b.y, a.z / b.z) });

impl_op_ex!(* |a: &Vec3, b: &f32| -> Vec3 { Vec3::new(a.x * b, a.y * b, a.z * b) });
impl_op_ex!(/ |a: &Vec3, b: &f32| -> Vec3 { Vec3::new(a.x / b, a.y / b, a.z / b) });

impl_op_ex!(- |a: &Vec3| -> Vec3 { Vec3::new(-a.x, -a.y, -a.z) });

impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| { a.x += b.x; a.y += b.y; a.z += b.z; });
impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });
impl_op_ex!(*= |a: &mut Vec3, b: &Vec3| { a.x *= b.x; a.y *= b.y; a.z *= b.z; });
impl_op_ex!(/= |a: &mut Vec3, b: &Vec3| { a.x /= b.x; a.y /= b.y; a.z /= b.z; });

impl_op_ex!(*= |a: &mut Vec3, b: &f32| { a.x *= b; a.y *= b; a.z *= b; });
impl_op_ex!(/= |a: &mut Vec3, b: &f32| { a.x /= b; a.y /= b; a.z /= b; });

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}



#[cfg(test)]
mod vec3_test {
    use super::Vec3;

    #[test]
    fn addition_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let expected = Vec3::new(5.0, 7.0, 9.0);

        assert_eq!(a + b, expected);
    }

    #[test]
    fn subtraction_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let expected = Vec3::new(-3.0, -3.0, -3.0);

        assert_eq!(a - b, expected);
    }

    #[test]
    fn multiplication_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let expected = Vec3::new(4.0, 10.0, 18.0);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn division_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let expected = Vec3::new(1.0 / 4.0, 2.0 / 5.0, 3.0 / 6.0);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn scalar_multiplication_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 4.0;

        let expected = Vec3::new(4.0, 8.0, 12.0);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn scalar_division_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 4.0;

        let expected = Vec3::new(1.0 / 4.0, 2.0 / 4.0, 3.0 / 4.0);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn negation_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);

        let expected = Vec3::new(-1.0, -2.0, -3.0);

        assert_eq!(-a, expected);
    }

    #[test]
    fn addition_assignment_test() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let expected = Vec3::new(5.0, 7.0, 9.0);

        a += b;

        assert_eq!(a, expected);
    }

    #[test]
    fn subtraction_assignment_test() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let expected = Vec3::new(-3.0, -3.0, -3.0);

        a -= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn multiplication_assignment_test() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let expected = Vec3::new(4.0, 10.0, 18.0);

        a *= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn division_assignment_test() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let expected = Vec3::new(1.0 / 4.0, 2.0 / 5.0, 3.0 / 6.0);

        a /= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_multiplication_assignment_test() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 4.0;

        let expected = Vec3::new(4.0, 8.0, 12.0);

        a *= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_division_assignment_test() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 4.0;

        let expected = Vec3::new(1.0 / 4.0, 2.0 / 4.0, 3.0 / 4.0);

        a /= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn as_array_test() {
        let a = Vec3::new(1.0, 2.0, 3.0);

        let expected = [1.0, 2.0, 3.0];

        assert_eq!(a.as_array(), expected);
    }
}
