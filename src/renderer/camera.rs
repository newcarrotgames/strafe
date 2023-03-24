use glam::Vec3;

pub struct Camera {
    pub pos: Vec3,
    pub target: Vec3,
    ang: f32,
}

const PI: f32 = std::f32::consts::PI;
const PI2: f32 = PI * 2.0;
const HAFPI: f32 = PI / 2.0;
const FORFPI: f32 = PI / 4.0;
const DEGRADS: f32 = PI / 180.0;

pub fn fmod(x: f32, y: f32) -> f32 {
    let mut val = x - (x / y).trunc() * y;
    if val < 0.0 {
        val += y;
    }
    return val;
}

impl Camera {
    pub fn new(spawn: [u32; 2]) -> Camera {
        let pos = Vec3::new(spawn[0] as f32 + 0.5, 0.0, spawn[1] as f32 + 0.5);
        let ang: f32 = 0.0;
        let x = ang.sin();
        let y = ang.cos();
        let target = pos + Vec3::new(x, 0.0, y);
        Camera { pos, target, ang }
    }

    pub fn turn(&mut self, ang: f32) {
        let mut new_ang: f32 = self.ang + ang * DEGRADS;
        if new_ang >= PI2 {
            new_ang -= PI2;
        }
        let x = new_ang.sin();
        let y = new_ang.cos();
        let target = self.pos + Vec3::new(x, 0.0, y);
        self.target = target;
        self.ang = new_ang;
        log::info!("ang: {}", self.ang);
    }

    pub fn walk(&mut self, dir: i8) {
        let mut xdir = 0;
        let mut ydir = 0;
        let a = fmod(self.ang, PI2); // clamp value from 0 to 2PI

        if a >= FORFPI && a < HAFPI + FORFPI { // todo: precompute these silly values
            xdir = 1 * dir;
        } else if a >= HAFPI + FORFPI && a < PI + FORFPI {
            ydir = -1 * dir;
        } else if a >= PI + FORFPI && a < PI + HAFPI + FORFPI {
            xdir = -1 * dir;
        } else {
            ydir = 1 * dir;
        }

        log::info!("a: {}, xdir: {}, ydir: {}", a, xdir, ydir);

        let old_target = self.target - self.pos;
        self.pos = self.pos + Vec3::new(xdir as f32, 0.0, ydir as f32); // y is z in RHC
        self.target = self.pos + old_target;
    }
}
