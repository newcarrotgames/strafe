use glam::Vec3;

// #[rustfmt::skip]
// const CUBE_COLORS: [f32; 24] = [
//     0.0, 0.0, 0.0,
//     1.0, 0.0, 0.0,
//     1.0, 1.0, 0.0,
//     0.0, 1.0, 0.0,
//     0.0, 0.0, 1.0,
//     1.0, 0.0, 1.0,
//     1.0, 1.0, 1.0,
//     0.0, 1.0, 1.0
// ];

// #[rustfmt::skip]
// const CUBE_INDICES: [i32; 36] = [
//     0, 1, 2,
//     2, 3, 0,
//     1, 5, 6,
//     6, 2, 1,
//     7, 6, 5,
//     5, 4, 7,
//     4, 0, 3,
//     3, 7, 4,
//     4, 5, 1,
//     1, 0, 4,
//     3, 2, 6,
//     6, 7, 3
// ];

#[rustfmt::skip]
const TEXTURE_COORDS: [f32; 8] = [
    0.0, 1.0,
    1.0, 1.0,
    1.0, 0.0,
    0.0, 0.0
];

#[rustfmt::skip]
const CUBE_VERTICES: [f32; 72] = [
    -0.5, -0.5,  0.5,
     0.5, -0.5,  0.5,
     0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
     0.5,  0.5,  0.5,
     0.5,  0.5, -0.5,
    -0.5,  0.5, -0.5,
     0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5,  0.5, -0.5,
     0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5,
     0.5, -0.5, -0.5,
     0.5, -0.5,  0.5,
    -0.5, -0.5,  0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5,  0.5, -0.5,
     0.5, -0.5,  0.5,
     0.5, -0.5, -0.5,
     0.5,  0.5, -0.5,
     0.5,  0.5,  0.5,
];

pub struct Cube {
    loc: Vec3,
    geom: [f32; 144],
    texture_id: u32,
}

// interleave cube verts with texture coords
#[rustfmt::skip]
fn get_vertices(loc: Vec3, tex_id: u32) -> [f32; 144] {
    let mut x:[f32; 144] = [0.0; 144];
    let mut i = 0;
    while i < 24 {
        let xi = i * 6;
        let vi = i * 3;
        let ti = (i % 4) * 2;

        // cube vertices
        x[xi]   = CUBE_VERTICES[vi]   + loc[0];
        x[xi+1] = CUBE_VERTICES[vi+1] + loc[1];
        x[xi+2] = CUBE_VERTICES[vi+2] + loc[2];

        // texture coordinates
        x[xi+3] = TEXTURE_COORDS[ti];
        x[xi+4] = TEXTURE_COORDS[ti+1];

        // texture array index
        x[xi+5] = tex_id as f32;

        i += 1;

        // log::info!("x: {}, {}, {}, {}, {}", x[xi], x[xi + 1], x[xi + 2], x[xi + 3], x[xi + 4]);
    }
    log::debug!("get_vertices: {}", format!("{:?}", x));
    return x;
}

impl Cube {
    pub fn new(loc: Vec3, texture_id: u32) -> Self {
        log::debug!("new cube");
        let g = get_vertices(loc, texture_id);
        Self {
            loc,
            geom: g,
            texture_id,
        }
    }

    pub fn geom(&self) -> [f32; 144] {
        return self.geom;
    }

    pub fn loc(&self) -> Vec3 {
        return self.loc;
    }
}
