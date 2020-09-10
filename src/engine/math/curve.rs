#![allow(dead_code)]

use serde::Deserialize;

trait Polynom {
    fn at(&self, at: f32) -> f32;
}
impl Polynom for () {
    fn at(&self, _: f32) -> f32 {
        0.0
    }
}
impl Polynom for f32 {
    fn at(&self, _: f32) -> f32 {
        *self
    }
}
impl Polynom for (f32, f32) {
    fn at(&self, at: f32) -> f32 {
        self.0 * at + self.1
    }
}
trait PolyTo<T> {
    fn to_polynom(self) -> T;
}
impl PolyTo<(f32, f32)> for ((f32, f32), (f32, f32)) {
    fn to_polynom(self) -> (f32, f32) {
        let ((x1, y1), (x2, y2)) = self;

        let slope = (y2 - y1) / (x2 - x1);
        let zero_value = y1 - slope * x1;

        (slope, zero_value)
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Curve {
    values: Vec<(f32, f32)>,
}

impl Curve {
    pub fn sample(&self, at: f32) -> f32 {
        match self.values.len() {
            0 => ().at(at),
            1 => (self.values[0].1).at(at),
            2 => (self.values[1], self.values[0]).to_polynom().at(at),
            _ => {
                if at <= self.values[0].0 {
                    (self.values[1], self.values[0]).to_polynom().at(at)
                } else if at >= self.values.last().unwrap().0 {
                    let last_index = self.values.len() - 1;
                    (self.values[last_index], self.values[last_index - 1])
                        .to_polynom()
                        .at(at)
                } else {
                    for (i, (x, _)) in self.values.iter().enumerate() {
                        if at <= *x {
                            return (self.values[i], self.values[i - 1]).to_polynom().at(at);
                        }
                    }
                    panic!("Unexpected end of algo");
                }
            }
        }
    }
}
