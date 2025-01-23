#![allow(dead_code)]

use glm::{Vec3, vec3};



pub struct Camera {
    fov: f32,
    screen_aspect_ratio: f32,
    close_plane: f32,
    far_plane: f32,

    position: Vec3,
    pitch: f32,
    yaw: f32,
    roll: f32,

    view_matrix: glm::Mat4,
    projection_matrix: glm::Mat4,
    pv_matrix: glm::Mat4, //for efficiency it is good to only calculate these after an update so their last correct value is stored and only updated when the funny flags are set

    look_vector: Vec3,

    projection_matrix_dirty_flag: bool, // it is comical that the word dirty is used by real legitimate serious programmers
    view_matrix_dirty_flag: bool,
}

impl Camera {
    pub fn new(position: Vec3, fov: f32, screen_aspect_ratio: f32, close_plane: f32, far_plane: f32, pitch: f32, yaw: f32, roll: f32) -> Camera {
        const BLANK_MATRIX: glm::Mat4 = glm::Mat4 { 
            c0: glm::Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
            c1: glm::Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
            c2: glm::Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
            c3: glm::Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
        };


        Camera {
            fov: fov.to_radians(),
            screen_aspect_ratio: screen_aspect_ratio,
            close_plane: close_plane,
            far_plane: far_plane,
            
            position: position,
            pitch: pitch,
            yaw: yaw,
            roll: roll,
        
            projection_matrix: BLANK_MATRIX,
            view_matrix: BLANK_MATRIX,
            pv_matrix: BLANK_MATRIX,

            look_vector: vec3(0.0, 0.0, 0.0),
        
            projection_matrix_dirty_flag: true,
            view_matrix_dirty_flag: true
        }
    }


    fn try_update_matrices(&mut self) {
        if (self.projection_matrix_dirty_flag) || (self.view_matrix_dirty_flag) { //if either matrix needs an update
            self.try_update_projection_matrix(); //test if the projection is still valid 
            self.try_update_view_matrix(); //test if the view is still valid

            self.pv_matrix = self.projection_matrix * self.view_matrix; //store the combined value, for slight efficiency uptick when no changes happen
        }
    }

    fn try_update_projection_matrix(&mut self) {
        if self.projection_matrix_dirty_flag {
            self.projection_matrix = glm::ext::perspective(self.fov, self.screen_aspect_ratio, self.close_plane, self.far_plane);
            self.projection_matrix_dirty_flag = false;
        }
    }
        

    fn try_update_view_matrix(&mut self) {
        if self.view_matrix_dirty_flag {
            let up: Vec3 = glm::normalize(vec3(-glm::sin(self.roll) * glm::cos(self.yaw) * glm::cos(self.pitch), glm::cos(self.roll), glm::sin(self.roll) * glm::sin(self.yaw)* glm::cos(self.pitch)));
            // without roll this would just be (0.0, 1.0, 0.0) but the up vector is affected by all angles when roll is involved
            // this took me so long to figure out the trig for 


            self.look_vector = glm::normalize(
                vec3(
                    glm::cos(self.pitch) * glm::sin(self.yaw),
                    glm::sin(self.pitch),
                    glm::cos(self.pitch) * glm::cos(self.yaw),
                )
            );

            self.view_matrix = glm::ext::look_at(self.position, self.look_vector + self.position, up); //make the matrix

            self.view_matrix_dirty_flag = false;
        }
    }


    // setting stuff    


    pub fn set_aspect_ratio (&mut self, aspect: f32) {
        self.screen_aspect_ratio = aspect;
        self.projection_matrix_dirty_flag = true;
    }

    pub fn rotate (&mut self, pitch: f32, yaw :f32, roll: f32) {
        self.pitch = glm::clamp(self.pitch + pitch, -1.565, 1.565);
        self.yaw += yaw;
        self.roll += roll;

        self.view_matrix_dirty_flag = true;
    }

    pub fn set_rotation (&mut self, pitch: Option<f32>, yaw: Option<f32>, roll: Option<f32>) {
        match pitch {
            Some(pitch_value) => {
                self.pitch = pitch_value;
                self.view_matrix_dirty_flag = true;
            }
            None => {}
        }
        match yaw {
            Some(yaw_value) => {
                self.yaw = yaw_value;
                self.view_matrix_dirty_flag = true;
            }
            None => {}
        }
        match roll {
            Some(roll_value) => {
                self.roll = roll_value;
                self.view_matrix_dirty_flag = true;
            }
            None => {}
        }
    }

    pub fn translate (&mut self, vector: Vec3) {
        self.position = self.position + vector;
        self.view_matrix_dirty_flag = true;
    }

    pub fn set_position (&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        match x {
            Some(x_pos) => {
                self.position.x = x_pos;
                self.view_matrix_dirty_flag = true;
            }
            None => {}
        }
        match y {
            Some(y_pos) => {
                self.position.y = y_pos;
                self.view_matrix_dirty_flag = true;
            }
            None => {}
        }
        match z {
            Some(z_pos) => {
                self.position.z = z_pos;
                self.view_matrix_dirty_flag = true;
            }
            None => {}
        }
    }



    // Matrix and vector getters 

    pub fn get_projection_view_matrix (&mut self) -> glm::Mat4 {
        self.try_update_matrices();

        self.pv_matrix
    }
    
    pub fn get_projection_matrix (&mut self) -> glm::Mat4 {
        self.try_update_projection_matrix();

        self.projection_matrix
    }
    
    pub fn get_view_matrix (&mut self) -> glm::Mat4 {
        self.try_update_view_matrix();

        self.view_matrix
    }


    pub fn get_forward_vector (&mut self) -> Vec3 {
        self.try_update_view_matrix();
        self.look_vector
    }

    pub fn get_flat_forward_vector (&self) -> Vec3{
        glm::normalize(vec3(
            glm::sin(self.yaw),
            0.0,
            glm::cos(self.yaw),
        ))
    }

    pub fn get_flat_right_vector (&self) -> Vec3{
        glm::normalize(vec3(
            -glm::cos(self.yaw),
            0.0,
            glm::sin(self.yaw),
        ))
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn get_rotation(&self) -> Vec3 { //pitch, yaw, roll
        vec3(self.pitch, self.yaw, self.roll)
    }



    pub fn do_matrices_need_update(&self) -> bool {
        (self.projection_matrix_dirty_flag) || (self.view_matrix_dirty_flag) 
    }
}