use glam::Vec3;

pub struct Camera {
    pub pos: Vec3,
    pub target: Vec3,
    ang: f32,
}

const DEGRADS:f32 = std::f32::consts::PI / 180.0;

impl Camera {
    pub fn new() -> Camera {
        let pos = Vec3::new(3.0, 0.0, 3.0);
        let ang:f32 = 135.0 * DEGRADS;
        let x = ang.sin();
        let y = ang.cos();
        let target = pos + Vec3::new(x, 0.0, y);
        Camera {
            pos,
            target,
            ang, 
        }
    }

    pub fn turn(&mut self, ang: f32) {
        let new_ang:f32 = self.ang + ang * DEGRADS;
        let x = new_ang.sin();
        let y = new_ang.cos();
        let target = self.pos + Vec3::new(x, 0.0, y);
        self.target = target;
        self.ang = new_ang;
    }
}