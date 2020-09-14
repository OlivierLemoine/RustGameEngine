// This file has been generated by a script, don't modify it!
#![allow(dead_code)]
pub use std::cell::RefCell;
pub use std::rc::{Rc, Weak};
pub use serde::Deserialize;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Default, Copy, Clone, Deserialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
impl Vector {
    pub const fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }
    pub const fn zero() -> Self {
        Vector::new(0.0, 0.0)
    }
    pub const fn one() -> Self {
        Vector::new(1.0, 1.0)
    }
    pub const fn up() -> Self {
        Vector::new(0.0, 1.0)
    }
    pub fn normalize(mut self) -> Self {
        let f = self.magnitude();
        self.x /= f;
        self.y /= f;
        self
    }
    pub fn abs(self) -> Self {
        Vector::new(self.x.abs(), self.y.abs())
    }
    pub fn dot_product(v1: &Vector, v2: &Vector) -> f32 {
        v1.x * v2.x + v1.y * v2.y
    }
    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
    pub fn magnitude_squared(&self) -> f32 {
        Vector::dot_product(self, self)
    }
    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }
    pub fn angle_radians(&self) -> f32 {
        self.y.atan2(self.x)
    }
    pub fn normal(&self) -> Self {
        let Vector { x, y } = *self;
        Vector::new(-y, x).normalize()
    }
}

impl From<[f32; 2]> for Vector {
    fn from(arr: [f32; 2]) -> Self {
        Vector::new(arr[0], arr[1])
    }
}
impl Sub for Vector {
    type Output = Vector;

    fn sub(mut self, other: Vector) -> Self::Output {
        self -= other;
        self
    }
}
impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl Add for Vector {
    type Output = Vector;

    fn add(mut self, other: Vector) -> Self::Output {
        self += other;
        self
    }
}
impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl MulAssign for Vector {
    fn mul_assign(&mut self, other: Vector) {
        self.x *= other.x;
        self.y *= other.y;
    }
}
impl Mul for Vector {
    type Output = Vector;
    fn mul(mut self, other: Vector) -> Self::Output {
        self *= other;
        self
    }
}
impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
}
impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(mut self, other: f32) -> Self::Output {
        self *= other;
        self
    }
}
impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
    }
}
impl Div<f32> for Vector {
    type Output = Vector;
    fn div(mut self, other: f32) -> Self::Output {
        self /= other;
        self
    }
}
impl DivAssign<Vector> for Vector {
    fn div_assign(&mut self, other: Vector) {
        self.x /= other.x;
        self.y /= other.y;
    }
}
impl Div for Vector {
    type Output = Vector;
    fn div(mut self, other: Vector) -> Self::Output {
        self /= other;
        self
    }
}


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

pub const NULL_CAMERA: Camera = Camera {
    position: Vector::zero(),
};

#[derive(Debug, Default)]
pub struct Camera {
    pub position: Vector,
}

#[derive(Debug)]
pub struct Time {
    pub delta: std::time::Duration,
    pub start: std::time::Instant,
}

#[derive(Deserialize, Debug)]
pub struct Object {
    #[serde(default)]
    pub ui: bool,
    pub transform: Option<Transform>,
    pub sprite: Option<Sprite>,
    pub rigidbody: Option<Rigidbody>,
    pub script: Option<Script>,
    #[serde(skip)]
    pub parent: Option<Weak<RefCell<Object>>>,
    #[serde(skip)]
    pub children: Vec<Rc<RefCell<Object>>>,
    #[serde(skip)]
    pub custom: Option<Box<dyn std::any::Any>>,
}
impl Object {
    pub fn global_transform(&self) -> Result<Transform, Box<dyn std::error::Error>> {
        let mut res = self
            .transform
            .clone()
            .ok_or("Object does not have a transform")?;

        if self.parent.has_transform() {
            if let Some(parent) = &self.parent {
                let parent_global_transform = parent
                    .upgrade()
                    .ok_or("Could not upgrade to parent")?
                    .try_borrow()?
                    .global_transform()?;
                res.position = (res.position * parent_global_transform.scale)
                    + parent_global_transform.position;
                res.scale *= parent_global_transform.scale;
            }
        }

        Ok(res)
    }
}
pub trait ComponentChecker {
    fn has_transform(&self) -> bool;
    fn has_sprite(&self) -> bool;
    fn has_rigidbody(&self) -> bool;
    fn has_script(&self) -> bool;
    fn has_parent(&self) -> bool;
}
impl ComponentChecker for Object {
    fn has_transform(&self) -> bool {
        self.transform.is_some()
    }
    fn has_sprite(&self) -> bool {
        self.sprite.is_some()
    }
    fn has_rigidbody(&self) -> bool {
        self.rigidbody.is_some()
    }
    fn has_script(&self) -> bool {
        self.script.is_some()
    }
    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}

macro_rules! impl_component_checker_opt_weak_refcell_obj {
    ($($f:ident),*) => {
        impl ComponentChecker for Option<Weak<RefCell<Object>>> {
            $(
                fn $f(&self) -> bool {
                    self.as_ref()
                    .map(|parent| {
                        parent
                            .upgrade()
                            .map(|v| v.try_borrow().ok().map(|v| v.$f()))
                    })
                    .flatten()
                    .flatten()
                    .unwrap_or(false)
                }
            )*
        }
    };
}
macro_rules! impl_component_checker_rc_refcell_obj {
    ($($f:ident),*) => {
        impl ComponentChecker for Rc<RefCell<Object>> {
            $(
                fn $f(&self) -> bool {
                    self.try_borrow()
                        .map(|v| v.$f())
                        .unwrap_or(false)
                }
            )*
        }
    };
}
macro_rules! impl_all_component_checker {
    ($($f:ident),*) => {
        impl_component_checker_opt_weak_refcell_obj!{$($f),*}
        impl_component_checker_rc_refcell_obj!{$($f),*}
    };
}

impl_all_component_checker! {has_transform, has_sprite, has_rigidbody, has_script, has_parent}

#[derive(Deserialize, Debug, Clone)]
pub struct Transform {
    #[serde(default)]
    pub position: Vector,
    #[serde(default = "Vector::one")]
    pub scale: Vector,
}
impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Vector::default(),
            scale: Vector::one(),
        }
    }
}

pub type ParseCustomObject = fn(&str) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>>;
pub type OnClick = fn(&mut Object);
pub type Update = fn(&mut Object, &mut Camera, &Time);

#[derive(Deserialize, Default, Debug)]
pub struct Script {
    #[serde(default)]
    pub lib: String,
}

#[derive(Deserialize, Debug)]
pub struct Sprite {
    pub depth: u32,
    pub text: Option<String>,
    pub color: Option<[f32; 4]>,
    #[serde(skip)]
    pub animations: std::collections::HashMap<String, Vec<usize>>,
}

fn one() -> f32 {
    1f32
}

#[derive(Deserialize, Debug)]
pub struct Rigidbody {
    #[serde(default = "one")]
    pub mass: f32,
    #[serde(default)]
    pub acceleration: Vector,
    #[serde(default)]
    pub speed: Vector,
    #[serde(default)]
    pub force: Vector,

    #[serde(skip)]
    pub is_on_ground: bool,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Rigidbody {
            mass: 1f32,
            acceleration: Vector::zero(),
            speed: Vector::zero(),
            force: Vector::zero(),

            is_on_ground: false,
        }
    }
}
