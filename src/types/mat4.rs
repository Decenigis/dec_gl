use std::fmt::{Display, Formatter};
use crate::types::vec4::{vec4, Vec4};
use std::hash::{Hash, Hasher};
use impl_ops::*;
use std::ops;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub c0: Vec4,
    pub c1: Vec4,
    pub c2: Vec4,
    pub c3: Vec4
}

impl Hash for Mat4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.c0.hash(state);
        self.c1.hash(state);
        self.c2.hash(state);
        self.c3.hash(state);
    }
}

impl Display for Mat4 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.c0, self.c1, self.c2, self.c3)
    }
}

impl Index<usize> for Mat4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.c0,
            1 => &self.c1,
            2 => &self.c2,
            3 => &self.c3,
            _ => panic!("Index out of bounds")
        }
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.c0,
            1 => &mut self.c1,
            2 => &mut self.c2,
            3 => &mut self.c3,
            _ => panic!("Index out of bounds")
        }
    }
}

impl_op_ex!(+ |a: &Mat4, b: &Mat4| -> Mat4 { Mat4::new(a.c0 + b.c0, a.c1 + b.c1, a.c2 + b.c2, a.c3 + b.c3) });
impl_op_ex!(- |a: &Mat4, b: &Mat4| -> Mat4 { Mat4::new(a.c0 - b.c0, a.c1 - b.c1, a.c2 - b.c2, a.c3 - b.c3) });
impl_op_ex!(* |a: &Mat4, b: &Mat4| -> Mat4 {
    let mut out = [0.0; 16];
    let a_array = a.as_array();
    let b_array = b.as_array();

    for i in 0..4 {
        for j in 0..4 {
            out[i * 4 + j] = a_array[0].as_array()[i] * b_array[j].as_array()[0]
                          + a_array[1].as_array()[i] * b_array[j].as_array()[1]
                          + a_array[2].as_array()[i] * b_array[j].as_array()[2]
                          + a_array[3].as_array()[i] * b_array[j].as_array()[3];
        }
    }

    Mat4::from(out)
});

impl From<glm::Mat4> for Mat4 {
    fn from(mat: glm::Mat4) -> Self {
        Mat4 {
            c0: vec4(mat.c0.x, mat.c0.y, mat.c0.z, mat.c0.w),
            c1: vec4(mat.c1.x, mat.c1.y, mat.c1.z, mat.c1.w),
            c2: vec4(mat.c2.x, mat.c2.y, mat.c2.z, mat.c2.w),
            c3: vec4(mat.c3.x, mat.c3.y, mat.c3.z, mat.c3.w)
        }
    }
}

impl From<[f32; 16]> for Mat4 {
    fn from(array: [f32; 16]) -> Self {
        Mat4 {
            c0: vec4(array[0], array[1], array[2], array[3]),
            c1: vec4(array[4], array[5], array[6], array[7]),
            c2: vec4(array[8], array[9], array[10], array[11]),
            c3: vec4(array[12], array[13], array[14], array[15])
        }
    }
}

impl Mat4 {
    pub fn new(c0: Vec4, c1: Vec4, c2: Vec4, c3: Vec4) -> Mat4 {
        Mat4 {
            c0,
            c1,
            c2,
            c3
        }
    }
    
    pub fn identity() -> Mat4 {
        Mat4::new(
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0, 1.0, 0.0, 0.0),
            vec4(0.0, 0.0, 1.0, 0.0),
            vec4(0.0, 0.0, 0.0, 1.0)
        )
    }

    pub fn as_array(&self) -> [Vec4; 4] {
        [
            self.c0,
            self.c1,
            self.c2,
            self.c3
        ]
    }
}

pub fn mat4(c0: Vec4, c1: Vec4, c2: Vec4, c3: Vec4) -> Mat4 {
    Mat4::new(c0, c1, c2, c3)
}


#[cfg(test)]
mod mat4_test {
    use crate::types::{vec4, Mat4};

    #[test]
    fn addition_test() {
        let a = Mat4::new(
            vec4(1.0, 2.0, 3.0, 4.0),
            vec4(5.0, 6.0, 7.0, 8.0),
            vec4(9.0, 10.0, 11.0, 12.0),
            vec4(13.0, 14.0, 15.0, 16.0)
        );
        let b = Mat4::new(
            vec4(17.0, 18.0, 19.0, 20.0),
            vec4(21.0, 22.0, 23.0, 24.0),
            vec4(25.0, 26.0, 27.0, 28.0),
            vec4(29.0, 30.0, 31.0, 32.0)
        );

        let expected = Mat4::new(
            vec4(18.0, 20.0, 22.0, 24.0),
            vec4(26.0, 28.0, 30.0, 32.0),
            vec4(34.0, 36.0, 38.0, 40.0),
            vec4(42.0, 44.0, 46.0, 48.0)
        );

        assert_eq!(a + b, expected);
    }

    #[test]
    fn subtraction_test() {
        let a = Mat4::new(
            vec4(1.0, 2.0, 3.0, 4.0),
            vec4(5.0, 6.0, 7.0, 8.0),
            vec4(9.0, 10.0, 11.0, 12.0),
            vec4(13.0, 14.0, 15.0, 16.0)
        );
        let b = Mat4::new(
            vec4(17.0, 18.0, 19.0, 20.0),
            vec4(21.0, 22.0, 23.0, 24.0),
            vec4(25.0, 26.0, 27.0, 28.0),
            vec4(29.0, 30.0, 31.0, 32.0)
        );

        let expected = Mat4::new(
            vec4(-16.0, -16.0, -16.0, -16.0),
            vec4(-16.0, -16.0, -16.0, -16.0),
            vec4(-16.0, -16.0, -16.0, -16.0),
            vec4(-16.0, -16.0, -16.0, -16.0)
        );

        assert_eq!(a - b, expected);
    }

    #[test]
    fn multiplication_test() {
        let a = Mat4::new(
            vec4(1.0, 2.0, 3.0, 4.0),
            vec4(5.0, 6.0, 7.0, 8.0),
            vec4(9.0, 10.0, 11.0, 12.0),
            vec4(13.0, 14.0, 15.0, 16.0)
        );

        let b = Mat4::new(
            vec4(17.0, 18.0, 19.0, 20.0),
            vec4(21.0, 22.0, 23.0, 24.0),
            vec4(25.0, 26.0, 27.0, 28.0),
            vec4(29.0, 30.0, 31.0, 32.0)
        );

        let expected = Mat4::new(
            vec4(538.0, 650.0, 762.0, 874.0),
            vec4(612.0, 740.0, 868.0, 996.0),
            vec4(686.0, 830.0, 974.0, 1118.0),
            vec4(760.0, 920.0, 1080.0, 1240.0),
        );

        let c = a * b;

        assert_eq!(expected, c);
    }
}
