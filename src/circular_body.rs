use std::f64::consts::PI;
use macroquad::color::WHITE;
use macroquad::math::Vec2;
use macroquad::shapes::draw_circle;
use crate::MASS_FACTOR;

pub struct CircularBody {
    mass: f64,
    radius: f64,
    pos: Vec2,
    v: Vec2,
    a: Vec2,
    pub destroy: bool,
}

impl CircularBody {
    pub fn new(mass: f64,radius: f64, pos: Vec2, vel: Vec2) -> Self {
        Self {
            mass,
            radius,
            pos,
            v: vel,
            a: Vec2::ZERO,
            destroy: false,
        }
    }
    pub fn get_impulse(&self) -> Vec2 {
        self.v * self.mass as f32
    }
    pub fn get_density(&self) -> f64 {
        self.mass / (PI * self.radius * self.radius)
    }
    pub fn get_mass(&self) -> f64 {
        self.mass
    }
    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }
    pub fn get_rad(&self) -> f64 {
        self.radius
    }
    pub fn absorb(&mut self, body_mass: f64, body_impulse: Vec2) {
        self.mass += body_mass;
        self.radius = ((self.mass/MASS_FACTOR)*3f64/(4f64*PI)).cbrt();
        let new_impulse = self.get_impulse() + body_impulse;
        self.v = new_impulse / self.mass as f32;
    }
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius as f32, WHITE);
    }
    pub fn update(&mut self) {
        self.v += self.a;
        self.pos += self.v;
        self.a = Vec2::ZERO;
    }
    pub fn apply_force(&mut self, force: Vec2) {
        self.a += force / self.mass as f32;
    }
    pub fn destroy(&mut self) {
        self.destroy = true;
    }
}