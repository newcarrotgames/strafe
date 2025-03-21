use rand::rngs::ThreadRng;
use rand::Rng;

const LEVEL_WIDTH: i32 = 64;
const LEVEL_HEIGHT: i32 = 64;

/// Wall tile
const TILE_WALL: u32 = 40;
/// Floor tile
const TILE_FLOOR: u32 = 0;

/// Simple rectangle struct for convenience
#[derive(Copy, Clone)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    /// Creates a new Rect from an (x, y) origin, width (w), and height (h).
    fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + w - 1,
            y2: y + h - 1,
        }
    }

    /// Returns true if this rectangle intersects another.
    fn intersects(&self, other: &Rect) -> bool {
        !(self.x2 < other.x1
            || self.x1 > other.x2
            || self.y2 < other.y1
            || self.y1 > other.y2)
    }

    /// Returns the center of this rectangle as (x, y).
    fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }
}

pub struct Level {
    rng: ThreadRng,
    pub data: [[u32; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
    pub spawn: [u32; 2],
}

impl Level {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            data: [[0; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
            spawn: [1, 1],
        }
    }

    /// Sets the tile at (x, y) to `val`.
    pub fn set_value(&mut self, x: u32, y: u32, val: u32) {
        self.data[y as usize][x as usize] = val;
    }

    /// A helper to fill the map with a certain tile.
    fn fill(&mut self, val: u32) {
        for y in 0..LEVEL_HEIGHT {
            for x in 0..LEVEL_WIDTH {
                self.set_value(x as u32, y as u32, val);
            }
        }
    }

    /// Build a corridor in a horizontal line.
    fn carve_horizontal_corridor(&mut self, x1: i32, x2: i32, y: i32) {
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        for x in min_x..=max_x {
            self.set_value(x as u32, y as u32, TILE_FLOOR);
        }
    }

    /// Build a corridor in a vertical line.
    fn carve_vertical_corridor(&mut self, y1: i32, y2: i32, x: i32) {
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);
        for y in min_y..=max_y {
            self.set_value(x as u32, y as u32, TILE_FLOOR);
        }
    }

    /// Carve out a rectangular room: walls around the perimeter, floors inside.
    fn carve_room(&mut self, rect: &Rect) {
        for y in rect.y1..=rect.y2 {
            for x in rect.x1..=rect.x2 {
                // If on the border of the room, keep a wall
                if x == rect.x1 || x == rect.x2 || y == rect.y1 || y == rect.y2 {
                    self.set_value(x as u32, y as u32, TILE_WALL);
                } else {
                    // Otherwise, floor
                    self.set_value(x as u32, y as u32, TILE_FLOOR);
                }
            }
        }
    }

    pub fn build(&mut self) {
        // 1) Fill entire map with walls
        self.fill(TILE_WALL);

        // 2) Repeatedly attempt to place random rooms
        let max_rooms = 10;       // up to 10 rooms
        let min_size = 5;         // room min dimension
        let max_size = 12;        // room max dimension
        let mut rooms = Vec::new();

        for _ in 0..50 { // Up to 50 attempts to place rooms
            let w = self.rng.gen_range(min_size..=max_size);
            let h = self.rng.gen_range(min_size..=max_size);
            let x = self.rng.gen_range(1..(LEVEL_WIDTH - w - 1));
            let y = self.rng.gen_range(1..(LEVEL_HEIGHT - h - 1));
            let new_room = Rect::new(x, y, w, h);

            // Check for intersection with existing rooms
            let mut failed = false;
            for other_room in &rooms {
                if new_room.intersects(other_room) {
                    failed = true;
                    break;
                }
            }
            if !failed {
                // Carve out this new room
                self.carve_room(&new_room);
                rooms.push(new_room);
                if rooms.len() >= max_rooms {
                    break;
                }
            }
        }

        // 3) Connect rooms with corridors
        // We'll link each room to the previous one with an L‐shaped corridor
        for i in 1..rooms.len() {
            let (prev_x, prev_y) = rooms[i - 1].center();
            let (new_x, new_y)   = rooms[i].center();

            // Randomly decide if we carve horizontally first or vertically first
            if self.rng.gen_bool(0.5) {
                // carve horizontally, then vertically
                self.carve_horizontal_corridor(prev_x, new_x, prev_y);
                self.carve_vertical_corridor(prev_y, new_y, new_x);
            } else {
                // carve vertically, then horizontally
                self.carve_vertical_corridor(prev_y, new_y, prev_x);
                self.carve_horizontal_corridor(prev_x, new_x, new_y);
            }
        }

        // 4) Place spawn in the first room if any exist
        if !rooms.is_empty() {
            let (spawn_x, spawn_y) = rooms[0].center();
            self.spawn = [spawn_x as u32, spawn_y as u32];
        }
    }
}
