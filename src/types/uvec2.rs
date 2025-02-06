use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use impl_ops::*;
use std::ops;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

impl Hash for UVec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Display for UVec2 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Index<usize> for UVec2 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds")
        }
    }
}

impl IndexMut<usize> for UVec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds")
        }
    }
}

impl_op_ex!(+ |a: &UVec2, b: &UVec2| -> UVec2 { UVec2::new(a.x + b.x, a.y + b.y) });
impl_op_ex!(- |a: &UVec2, b: &UVec2| -> UVec2 { UVec2::new(a.x.saturating_sub(b.x), a.y.saturating_sub(b.y)) });
impl_op_ex!(* |a: &UVec2, b: &UVec2| -> UVec2 { UVec2::new(a.x * b.x, a.y * b.y) });
impl_op_ex!(/ |a: &UVec2, b: &UVec2| -> UVec2 { UVec2::new(a.x / b.x, a.y / b.y) });

impl_op_ex!(* |a: &UVec2, b: &u32| -> UVec2 { UVec2::new(a.x * b, a.y * b) });
impl_op_ex!(/ |a: &UVec2, b: &u32| -> UVec2 { UVec2::new(a.x / b, a.y / b) });

impl_op_ex!(+= |a: &mut UVec2, b: &UVec2| { a.x += b.x; a.y += b.y; });
impl_op_ex!(-= |a: &mut UVec2, b: &UVec2| { a.x = a.x.saturating_sub(b.x); a.y = a.y.saturating_sub(b.y); });
impl_op_ex!(*= |a: &mut UVec2, b: &UVec2| { a.x *= b.x; a.y *= b.y; });
impl_op_ex!(/= |a: &mut UVec2, b: &UVec2| { a.x /= b.x; a.y /= b.y; });

impl_op_ex!(*= |a: &mut UVec2, b: &u32| { a.x *= b; a.y *= b; });
impl_op_ex!(/= |a: &mut UVec2, b: &u32| { a.x /= b; a.y /= b; });

impl UVec2 {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn as_array(&self) -> [u32; 2] {
        [self.x, self.y]
    }
}

pub fn uvec2(x: u32, y: u32) -> UVec2 {
    UVec2::new(x, y)
}

#[cfg(test)]
mod uvec2_test {
    use super::UVec2;

    #[test]
    fn addition_test() {
        let a = UVec2::new(1, 2);
        let b = UVec2::new(3, 4);

        let expected = UVec2::new(4, 6);

        assert_eq!(a + b, expected);
    }

    #[test]
    fn subtraction_test() {
        let a = UVec2::new(5, 6);
        let b = UVec2::new(3, 4);

        let expected = UVec2::new(2, 2);

        assert_eq!(a - b, expected);
    }

    #[test]
    fn multiplication_test() {
        let a = UVec2::new(1, 2);
        let b = UVec2::new(3, 4);

        let expected = UVec2::new(3, 8);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn division_test() {
        let a = UVec2::new(6, 8);
        let b = UVec2::new(3, 4);

        let expected = UVec2::new(2, 2);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn scalar_multiplication_test() {
        let a = UVec2::new(1, 2);
        let b = 3;

        let expected = UVec2::new(3, 6);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn scalar_division_test() {
        let a = UVec2::new(6, 9);
        let b = 3;

        let expected = UVec2::new(2, 3);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn addition_assignment_test() {
        let mut a = UVec2::new(1, 2);
        let b = UVec2::new(3, 4);

        let expected = UVec2::new(4, 6);

        a += b;

        assert_eq!(a, expected);
    }

    #[test]
    fn subtraction_assignment_test() {
        let mut a = UVec2::new(5, 6);
        let b = UVec2::new(3, 4);

        let expected = UVec2::new(2, 2);

        a -= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn multiplication_assignment_test() {
        let mut a = UVec2::new(1, 2);
        let b = UVec2::new(3, 4);

        let expected = UVec2::new(3, 8);

        a *= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn division_assignment_test() {
        let mut a = UVec2::new(6, 8);
        let b = UVec2::new(3, 4);

        let expected = UVec2::new(2, 2);

        a /= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_multiplication_assignment_test() {
        let mut a = UVec2::new(1, 2);
        let b = 3;

        let expected = UVec2::new(3, 6);

        a *= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_division_assignment_test() {
        let mut a = UVec2::new(6, 9);
        let b = 3;

        let expected = UVec2::new(2, 3);

        a /= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn as_array_test() {
        let a = UVec2::new(1, 2);

        let expected = [1, 2];

        assert_eq!(a.as_array(), expected);
    }
}