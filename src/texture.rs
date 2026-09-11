use crate::framebuffer::rgb;

#[derive(Clone, Copy, Debug)]
pub enum CubeFace {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

pub struct Texture;

impl Texture {
    pub fn rubik(face: CubeFace, u: f32, v: f32) -> u32 {
        let u = u.clamp(0.0, 0.999999);
        let v = v.clamp(0.0, 0.999999);

        let scaled_u = u * 3.0;
        let scaled_v = v * 3.0;

        let local_u = scaled_u.fract();
        let local_v = scaled_v.fract();

        let border = 0.08;

        if local_u < border || local_u > 1.0 - border || local_v < border || local_v > 1.0 - border
        {
            return rgb(8, 8, 8);
        }

        match face {
            CubeFace::Front => rgb(210, 25, 25),
            CubeFace::Back => rgb(255, 100, 15),
            CubeFace::Left => rgb(20, 165, 70),
            CubeFace::Right => rgb(25, 80, 210),
            CubeFace::Top => rgb(235, 235, 230),
            CubeFace::Bottom => rgb(235, 205, 20),
        }
    }
}
