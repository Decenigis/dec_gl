use std::hash::{Hash, Hasher};
use impl_ops::*;
use std::ops;
use crate::math::Normalise;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Hash for Vec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl Normalise for Vec2 {
    fn normalise(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y).sqrt();
        Vec2::new(self.x / length, self.y / length)
    }
}

impl_op_ex!(+ |a: &Vec2, b: &Vec2| -> Vec2 { Vec2::new(a.x + b.x, a.y + b.y )});
impl_op_ex!(- |a: &Vec2, b: &Vec2| -> Vec2 { Vec2::new(a.x - b.x, a.y - b.y) });
impl_op_ex!(* |a: &Vec2, b: &Vec2| -> Vec2 { Vec2::new(a.x * b.x, a.y * b.y) });
impl_op_ex!(/ |a: &Vec2, b: &Vec2| -> Vec2 { Vec2::new(a.x / b.x, a.y / b.y) });

impl_op_ex!(* |a: &Vec2, b: &f32| -> Vec2 { Vec2::new(a.x * b, a.y * b) });
impl_op_ex!(/ |a: &Vec2, b: &f32| -> Vec2 { Vec2::new(a.x / b, a.y / b) });

impl_op_ex!(- |a: &Vec2| -> Vec2 { Vec2::new(-a.x, -a.y) });

impl_op_ex!(+= |a: &mut Vec2, b: &Vec2| { a.x += b.x; a.y += b.y; });
impl_op_ex!(-= |a: &mut Vec2, b: &Vec2| { a.x -= b.x; a.y -= b.y; });
impl_op_ex!(*= |a: &mut Vec2, b: &Vec2| { a.x *= b.x; a.y *= b.y; });
impl_op_ex!(/= |a: &mut Vec2, b: &Vec2| { a.x /= b.x; a.y /= b.y; });

impl_op_ex!(*= |a: &mut Vec2, b: &f32| { a.x *= b; a.y *= b; });
impl_op_ex!(/= |a: &mut Vec2, b: &f32| { a.x /= b; a.y /= b; });

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn as_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}



#[cfg(test)]
mod vec2_test {
    use crate::types::vec2::Vec2;

    #[test]
    fn addition_test() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        let expected = Vec2::new(4.0, 6.0);

        assert_eq!(a + b, expected);
    }

    #[test]
    fn subtraction_test() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        let expected = Vec2::new(-2.0, -2.0);

        assert_eq!(a - b, expected);
    }

    #[test]
    fn multiplication_test() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        let expected = Vec2::new(3.0, 8.0);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn division_test() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        let expected = Vec2::new(1.0/3.0, 2.0/4.0);

        assert_eq!(a / b, expected);
    }

    #[test]
    fn scalar_multiplication_test() {
        let a = Vec2::new(1.0, 2.0);
        let b = 3.0;

        let expected = Vec2::new(3.0, 6.0);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn scalar_division_test() {
        let a = Vec2::new(1.0, 2.0);
        let b = 3.0;

        let expected = Vec2::new(1.0/3.0, 2.0/3.0);

        assert_eq!(a / b, expected);
    }
    
    #[test]
    fn negation_test() {
        let a = Vec2::new(1.0, 2.0);
    
        let expected = Vec2::new(-1.0, -2.0);
    
        assert_eq!(-a, expected);
    }
    
    #[test]
    fn addition_assignment_test() {
        let mut a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
    
        let expected = Vec2::new(4.0, 6.0);
    
        a += b;
    
        assert_eq!(a, expected);
    }
    
    #[test]
    fn subtraction_assignment_test() {
        let mut a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
    
        let expected = Vec2::new(-2.0, -2.0);
    
        a -= b;
    
        assert_eq!(a, expected);
    }
    
    #[test]
    fn multiplication_assignment_test() {
        let mut a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
    
        let expected = Vec2::new(3.0, 8.0);
    
        a *= b;
    
        assert_eq!(a, expected);
    }
    
    #[test]
    fn division_assignment_test() {
        let mut a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
    
        let expected = Vec2::new(1.0 / 3.0, 2.0 / 4.0);
    
        a /= b;
    
        assert_eq!(a, expected);
    }
    
    #[test]
    fn scalar_multiplication_assignment_test() {
        let mut a = Vec2::new(1.0, 2.0);
        let b = 3.0;
    
        let expected = Vec2::new(3.0, 6.0);
    
        a *= b;
    
        assert_eq!(a, expected);
    }
    
    #[test]
    fn scalar_division_assignment_test() {
        let mut a = Vec2::new(1.0, 2.0);
        let b = 3.0;
    
        let expected = Vec2::new(1.0 / 3.0, 2.0 / 3.0);
    
        a /= b;
    
        assert_eq!(a, expected);
    }

    #[test]
    fn as_array_test() {
        let a = Vec2::new(1.0, 2.0);

        let expected = [1.0, 2.0];

        assert_eq!(a.as_array(), expected);
    }
}
