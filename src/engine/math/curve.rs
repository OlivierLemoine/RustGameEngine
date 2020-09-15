#![allow(dead_code)]

use super::Vector;
use serde::Deserialize;

trait Polynom {
    fn at(&self, at: f32) -> f32;
}
impl Polynom for (f32, f32) {
    #[inline]
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
pub struct Curve<T>(Vec<T>);
impl Curve<(f32, f32)> {
    pub fn sample(&self, at: f32) -> f32 {
        match self.0.len() {
            0 => 0.0,
            1 => self.0[0].1,
            2 => (self.0[1], self.0[0]).to_polynom().at(at),
            _ => {
                if at <= self.0[0].0 {
                    (self.0[1], self.0[0]).to_polynom().at(at)
                } else if at >= self.0.last().unwrap().0 {
                    let last_index = self.0.len() - 1;
                    (self.0[last_index], self.0[last_index - 1])
                        .to_polynom()
                        .at(at)
                } else {
                    for (i, (x, _)) in self.0.iter().enumerate() {
                        if at <= *x {
                            return (self.0[i], self.0[i - 1]).to_polynom().at(at);
                        }
                    }
                    panic!("Unexpected end of algo");
                }
            }
        }
    }
}
impl Curve<(f32, Vector)> {
    pub fn sample(&self, at: f32) -> Vector {
        match self.0.len() {
            0 => Vector::zero(),
            1 => self.0[0].1,
            2 => Vector::new(
                ((self.0[1].0, self.0[1].1.x), (self.0[0].0, self.0[0].1.x))
                    .to_polynom()
                    .at(at),
                ((self.0[1].0, self.0[1].1.y), (self.0[0].0, self.0[0].1.y))
                    .to_polynom()
                    .at(at),
            ),
            _ => {
                if at <= self.0[0].0 {
                    Vector::new(
                        ((self.0[1].0, self.0[1].1.x), (self.0[0].0, self.0[0].1.x))
                            .to_polynom()
                            .at(at),
                        ((self.0[1].0, self.0[1].1.y), (self.0[0].0, self.0[0].1.y))
                            .to_polynom()
                            .at(at),
                    )
                } else if at >= self.0.last().unwrap().0 {
                    let last_index = self.0.len() - 1;
                    Vector::new(
                        (
                            (self.0[last_index].0, self.0[last_index].1.x),
                            (self.0[last_index - 1].0, self.0[last_index - 1].1.x),
                        )
                            .to_polynom()
                            .at(at),
                        (
                            (self.0[last_index].0, self.0[last_index].1.y),
                            (self.0[last_index - 1].0, self.0[last_index - 1].1.y),
                        )
                            .to_polynom()
                            .at(at),
                    )
                } else {
                    for (i, (x, _)) in self.0.iter().enumerate() {
                        if at <= *x {
                            return Vector::new(
                                (
                                    (self.0[i].0, self.0[i].1.x),
                                    (self.0[i - 1].0, self.0[i - 1].1.x),
                                )
                                    .to_polynom()
                                    .at(at),
                                (
                                    (self.0[i].0, self.0[i].1.y),
                                    (self.0[i - 1].0, self.0[i - 1].1.y),
                                )
                                    .to_polynom()
                                    .at(at),
                            );
                        }
                    }
                    panic!("Unexpected end of algo");
                }
            }
        }
    }
}
