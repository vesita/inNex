struct HMatrix {
    matrix: [[f32; 4]; 4],
}

struct Position {
    x: f32,
    y: f32,
    z: f32,
    mat: HMatrix,
}

struct Rotation {
    rox: f32,
    roy: f32,
    roz: f32,
    mat: HMatrix,
}
