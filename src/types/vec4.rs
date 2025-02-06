use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use impl_ops::*;
use std::ops;
use std::ops::{Index, IndexMut};
use crate::math::Normalise;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Hash for Vec4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
        self.w.to_bits().hash(state);
    }
}

impl Normalise for Vec4 {
    fn normalise(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        Vec4::new(self.x / length, self.y / length, self.z / length, self.w / length)
    }
}

impl Display for Vec4 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds")
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds")
        }
    }
}

impl_op_ex!(+ |a: &Vec4, b: &Vec4| -> Vec4 { Vec4::new(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w) });
impl_op_ex!(- |a: &Vec4, b: &Vec4| -> Vec4 { Vec4::new(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w) });
impl_op_ex!(* |a: &Vec4, b: &Vec4| -> Vec4 { Vec4::new(a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w) });
impl_op_ex!(/ |a: &Vec4, b: &Vec4| -> Vec4 { Vec4::new(a.x / b.x, a.y / b.y, a.z / b.z, a.w / b.w) });

impl_op_ex!(* |a: &Vec4, b: &f32| -> Vec4 { Vec4::new(a.x * b, a.y * b, a.z * b, a.w * b) });
impl_op_ex!(/ |a: &Vec4, b: &f32| -> Vec4 { Vec4::new(a.x / b, a.y / b, a.z / b, a.w / b) });

impl_op_ex!(- |a: &Vec4| -> Vec4 { Vec4::new(-a.x, -a.y, -a.z, -a.w) });

impl_op_ex!(+= |a: &mut Vec4, b: &Vec4| { a.x += b.x; a.y += b.y; a.z += b.z; a.w += b.w; });
impl_op_ex!(-= |a: &mut Vec4, b: &Vec4| { a.x -= b.x; a.y -= b.y; a.z -= b.z; a.w -= b.w; });
impl_op_ex!(*= |a: &mut Vec4, b: &Vec4| { a.x *= b.x; a.y *= b.y; a.z *= b.z; a.w *= b.w; });
impl_op_ex!(/= |a: &mut Vec4, b: &Vec4| { a.x /= b.x; a.y /= b.y; a.z /= b.z; a.w /= b.w; });

impl_op_ex!(*= |a: &mut Vec4, b: &f32| { a.x *= b; a.y *= b; a.z *= b; a.w *= b; });
impl_op_ex!(/= |a: &mut Vec4, b: &f32| { a.x /= b; a.y /= b; a.z /= b; a.w /= b; });

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}

#[cfg(test)]
mod vec4_test {
    use super::Vec4;

    #[test]
    fn addition_test() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);

        let expected = Vec4::new(6.0, 8.0, 10.0, 12.0);

        assert_eq!(a + b, expected);
    }

    #[test]
    fn subtraction_test() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);

        let expected = Vec4::new(-4.0, -4.0, -4.0, -4.0);

        assert_eq!(a - b, expected);
    }

    #[test]
    fn multiplication_test() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);

        let expected = Vec4::new(5.0, 12.0, 21.0, 32.0);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn division_test() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);

        let expected = Vec4::new(1.0 / 5.0, 2.0 / 6.0, 3.0 / 7.0, 4.0 / 8.0);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn scalar_multiplication_test() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = 5.0;

        let expected = Vec4::new(5.0, 10.0, 15.0, 20.0);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn scalar_division_test() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = 5.0;

        let expected = Vec4::new(1.0 / 5.0, 2.0 / 5.0, 3.0 / 5.0, 4.0 / 5.0);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn negation_test() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);

        let expected = Vec4::new(-1.0, -2.0, -3.0, -4.0);

        assert_eq!(-a, expected);
    }

    #[test]
    fn addition_assignment_test() {
        let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);

        let expected = Vec4::new(6.0, 8.0, 10.0, 12.0);

        a += b;

        assert_eq!(a, expected);
    }

    #[test]
    fn subtraction_assignment_test() {
        let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);

        let expected = Vec4::new(-4.0, -4.0, -4.0, -4.0);

        a -= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn multiplication_assignment_test() {
        let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);

        let expected = Vec4::new(5.0, 12.0, 21.0, 32.0);

        a *= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn division_assignment_test() {
        let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);

        let expected = Vec4::new(1.0 / 5.0, 2.0 / 6.0, 3.0 / 7.0, 4.0 / 8.0);

        a /= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_multiplication_assignment_test() {
        let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = 5.0;

        let expected = Vec4::new(5.0, 10.0, 15.0, 20.0);

        a *= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_division_assignment_test() {
        let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = 5.0;

        let expected = Vec4::new(1.0 / 5.0, 2.0 / 5.0, 3.0 / 5.0, 4.0 / 5.0);

        a /= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn as_array_test() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);

        let expected = [1.0, 2.0, 3.0, 4.0];

        assert_eq!(a.as_array(), expected);
    }
}
