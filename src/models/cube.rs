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
const CUBE_VERTICES: [f32; 24] = [
    -0.5, -0.5,  0.5,
    0.5, -0.5,  0.5,
    0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5, -0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5,  0.5, -0.5,
    -0.5,  0.5, -0.5
];

// #[rustfmt::skip]
// const CUBE_VERTICES: [f32; 24] = [
//     4.0, 4.0, 6.0, 6.0, 4.0, 6.0, 6.0, 6.0, 6.0, 4.0, 6.0, 6.0, 4.0, 4.0, 4.0, 6.0, 4.0, 4.0, 6.0, 6.0, 4.0, 4.0, 6.0, 4.0
// ];

pub struct Cube {
    loc: Vec3,
    geom: [f32; 24],
}

fn get_vertices(loc: Vec3) -> [f32; 24] {
    let mut x:[f32; 24] = [0.0; 24];
    x[..24].clone_from_slice(&CUBE_VERTICES);
    println!("{:?}", x);
    let mut i = 0;
    while i < 24 {
        x[i] += loc[0];
        x[i+1] += loc[1];
        x[i+2] += loc[2];
        i += 3;
    }
    log::info!("get_vertices: {}", format!("{:?}", x));
    return x;
}

impl Cube {
    pub fn new(loc: Vec3) -> Self {
        log::info!("new cube");
        let g = get_vertices(loc);
        Self {
            loc,
            geom: g,
        }
    }

    pub fn geom(&self) -> [f32; 24] {
        return self.geom;
    }

    pub fn loc(&self) -> Vec3 {
        return self.loc;
    }
}
