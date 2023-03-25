pub struct Level {
    pub data: [[u32; 64]; 64],
    pub spawn: [u32; 2],
}

    // for (y, row) in FIRST_ROOM.iter().enumerate() {
    //     for (x, _) in row.iter().enumerate() {
    //         if FIRST_ROOM[y][x] > 0 {
    //             cubes.push(Cube::new(Vec3::new(x as f32, 1.0, y as f32), 1));   // ceiling
    //             cubes.push(Cube::new(Vec3::new(x as f32, 0.0, y as f32), FIRST_ROOM[y][x]));
    //             cubes.push(Cube::new(Vec3::new(x as f32, -1.0, y as f32), 1)); // floor
    //         }
    //     }
    // }

impl Level {
    pub fn new() -> Self {
        Self {
            data: [[0; 64]; 64],
            spawn: [1, 1],
        }
    }

    pub fn build(&self) {
        // place first room with player spawn
        
    }
}
