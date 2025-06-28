use inNex::graph::HMatrix::romat;
use srd;

impl HMatrix {
    pub fn new() -> Self {
        Self {
            matrix: [[0.0; 4]; 4],
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.matrix = [
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ];
    }

    pub fn set_rot_x(&mut self, x: f32) {
        self.matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, x.cos(), -x.sin(), 0.0],
            [0.0, x.sin(), x.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }

    pub fn set_rot_y(&mut self, y: f32) {
        self.matrix = [
            [y.cos(), 0.0, y.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-y.sin(), 0.0, y.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }

    pub fn set_rot_z(&mut self, z: f32) {
        self.matrix = [
            [z.cos(), -z.sin(), 0.0, 0.0],
            [z.sin(), z.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }

    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        self.matrix
    }
}

impl Position<f32> {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z };
        Self.mat = romat::set_position(x, y, z);
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_z(&self) -> f32 {
        self.z
    }

    pub fn rot(&self, rotMat: Rotation<f32>) {}
}
