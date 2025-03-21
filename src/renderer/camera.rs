use glam::Vec3;

pub struct Camera {
    pub pos: Vec3,
    pub target: Vec3,
    ang: f32,
    dir: i8,
}

const PI: f32 = std::f32::consts::PI;
const FORFPI: f32 = PI / 4.0;

impl Camera {
    pub fn new(spawn: [u32; 2]) -> Camera {
        let pos = Vec3::new(spawn[0] as f32, 0.0, spawn[1] as f32);
        let ang: f32 = 0.0;
        let x = ang.sin();
        let y = ang.cos();
        let target = pos + Vec3::new(x, 0.0, y);
        Camera {
            pos,
            target,
            ang,
            dir: 0,
        }
    }

    pub fn turn(&mut self, dir: i8) {
        // dir == 1 (+45°), dir == -1 (-45°)
        self.dir += dir;
        if self.dir > 7 {
            self.dir = 0;
        } else if self.dir < 0 {
            self.dir = 7;
        }

        let new_ang = self.dir as f32 * FORFPI;
        let x = new_ang.sin();
        let y = new_ang.cos();
        let target = self.pos + Vec3::new(x, 0.0, y);
        self.target = target;
        self.ang = new_ang;

        log::info!("ang: {}, dir: {}", self.ang, self.dir);
    }

    pub fn walk(&mut self, dir: i8) {
        let mut xdir = 0;
        let mut ydir = 0;
        match self.dir {
            0 => ydir = dir,
            1 => {
                xdir = dir;
                ydir = dir
            }
            2 => xdir = dir,
            3 => {
                xdir = dir;
                ydir = -dir
            }
            4 => ydir = -dir,
            5 => {
                xdir = -dir;
                ydir = -dir
            }
            6 => xdir = -dir,
            7 => {
                xdir = -dir;
                ydir = dir
            }
            _ => {
                log::error!("self.dir invalid: {}", self.dir);
            }
        }
        log::info!(
            "dir: {}, ang: {}, xdir: {}, ydir: {}",
            self.dir,
            self.ang,
            xdir,
            ydir
        );
        let old_target = self.target - self.pos;
        self.pos = self.pos + Vec3::new(xdir as f32, 0.0, ydir as f32); // y is z in RHC
        self.target = self.pos + old_target;
    }
}
