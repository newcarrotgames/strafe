use rand::rngs::ThreadRng;
use rand::Rng;

const LEVEL_WIDTH: i32 = 64;
const LEVEL_HEIGHT: i32 = 64;

pub struct Level {
    rng: ThreadRng,
    pub data: [[u32; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
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
            rng: rand::thread_rng(),
            data: [[0; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
            spawn: [1, 1],
        }
    }

    pub fn set_value(&mut self, x: u32, y: u32, val: u32) {
        self.data[y as usize][x as usize] = val;
    }

    pub fn build(&mut self) {
        // place first room with player spawn
        let avg_size = (LEVEL_WIDTH + LEVEL_HEIGHT) / 2;
        let min = avg_size / 9;
        let max = avg_size / 3;
        let new_room_width = self.rng.gen_range(min..max);
        let new_room_height = self.rng.gen_range(min..max);
        let new_room_x = self.rng.gen_range(2..(LEVEL_WIDTH - new_room_width - 2));
        let new_room_y = self.rng.gen_range(2..(LEVEL_HEIGHT - new_room_height - 2));

        // build room walls
        for y in 0..new_room_height {
            for x in 0..new_room_width {
                if y == 0 || x == 0 || y == new_room_height - 1 || x == new_room_width - 1 {
                    self.set_value((new_room_x + x) as u32, (new_room_y + y) as u32, 40);
                }
            }
        }

        // find new spawn
        let mut spawn_x = (new_room_x + self.rng.gen_range(2..new_room_width-2)) as f32;
        let mut spawn_y = (new_room_y + self.rng.gen_range(2..new_room_height-2)) as f32;
        // start in the middle of the cell
        spawn_x += 0.5;
        spawn_y += 0.5;
        self.spawn = [spawn_x as u32, spawn_y as u32];
    }
}
