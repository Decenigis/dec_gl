use glm::{IVec2, Mat4};

pub struct UICamera {
    projection_matrix: Mat4,
}

impl UICamera {
    pub fn new (dimensions: IVec2, near_plane: f32, far_plane: f32) -> UICamera {
        const BLANK_MATRIX: Mat4 = Mat4 { 
            c0: glm::Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
            c1: glm::Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
            c2: glm::Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
            c3: glm::Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
        };

        let mut camera = UICamera { projection_matrix: BLANK_MATRIX };
        camera.update_matrix(dimensions, near_plane, far_plane);

        camera
    }

    pub fn update_matrix (&mut self, dimensions: IVec2, near_plane: f32, far_plane: f32) {
        self.projection_matrix[0][0] = 2.0 / dimensions.x as f32;
        self.projection_matrix[1][1] = -2.0 / dimensions.y as f32;
        self.projection_matrix[2][2] = 2.0 / (far_plane - near_plane);
        
        self.projection_matrix[3][0] = (0.0 + dimensions.x as f32) / (0.0 - dimensions.x as f32);
        self.projection_matrix[3][1] = -(0.0 + dimensions.x as f32) / (0.0 - dimensions.x as f32);
        self.projection_matrix[3][2] = -(far_plane+near_plane) / (far_plane - near_plane);
        self.projection_matrix[3][3] = 1.0;       
    }

    pub fn get_matrix (&self) -> Mat4 {
        self.projection_matrix
    }
}