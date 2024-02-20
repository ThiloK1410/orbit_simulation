use std::f64::consts::PI;
use macroquad::math::Vec2;
use macroquad::window::{screen_height, screen_width};
use crate::circular_body::CircularBody;
use num_traits::pow::Pow;
use rand::Rng;
use crate::MASS_FACTOR;

const GRAVITATION: f64 = 0.00000000066743f64;
pub struct Engine {
    pub objects: Vec<CircularBody>
}

impl Engine {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }
    pub fn draw_all(&self) {
        for obj in &self.objects {
            obj.draw();
        }
    }

    pub fn update_all(&mut self) {
        //check for collision
        for i in 0..self.objects.len() {
            for j in 0..self.objects.len() {
                if i==j || self.objects[i].destroy {continue}
                let distance = self.objects[i].get_pos().distance(self.objects[j].get_pos());
                if distance <= self.objects[i].get_rad() as f32 {
                    let (body_mass, body_impulse) = (self.objects[j].get_mass(), self.objects[j].get_impulse());
                    self.objects[i].absorb(body_mass, body_impulse);
                    self.objects[j].destroy();
                }
            }
        }
        // removes all objects which are meant to be destroyed
        self.objects.retain(|body| !body.destroy);
        //apply gravitational pull
        for i in 0..self.objects.len() {
            for j in i..self.objects.len() {
                if i==j {continue}
                let force =
                    (GRAVITATION * self.objects[i].get_mass() * self.objects[j].get_mass() /
                        self.objects[i].get_pos().distance(self.objects[j].get_pos()).pow(2) as f64) as f32;
                let unit_direction = (self.objects[i].get_pos() - self.objects[j].get_pos())
                    .normalize();
                self.objects[i].apply_force(force*unit_direction*(-1f32));
                self.objects[j].apply_force(force*unit_direction);
            }
        }
        for obj in &mut self.objects {
            obj.update();
        }
    }

    pub fn spawn_object(&mut self, radius: f64, pos: Vec2, vel: Vec2) {
        self.objects.push(CircularBody::new(PI*radius.powf(3f64)*MASS_FACTOR*1.333f64, radius, pos, vel));
    }
    pub fn spawn_sample(&mut self, amount: usize, radius: f64, max_v: f32) {
        let mut rng = rand::thread_rng();
        for _ in 0..amount {
            let (x, y): (f32, f32) = (rng.gen::<f32>() * screen_width(), rng.gen::<f32>() * screen_height());
            let (dx, dy): (f32, f32) = ((rng.gen::<f32>()-0.5f32) * max_v, (rng.gen::<f32>()-0.5f32) * max_v);
            let pos = Vec2::new(x, y);
            let vel = Vec2::new(dx, dy);
            self.spawn_object(radius * (1f64 + rng.gen::<f64>()*2f64), pos, vel);
        }
    }
}