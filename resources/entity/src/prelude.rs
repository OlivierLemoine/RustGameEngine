// This file has been generated by a script, don't modify it!
#![allow(dead_code)]
pub use std::cell::RefCell;pub use std::rc::{Rc, Weak};pub use serde::Deserialize;use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};#[derive(Debug, Default, Copy, Clone)]pub struct Vector {pub x: f32,pub y: f32,}impl Vector {pub fn new(x: f32, y: f32) -> Self {Vector { x, y }}pub fn zero() -> Self {Vector::new(0.0, 0.0)}pub fn one() -> Self {Vector::new(1.0, 1.0)}pub fn up() -> Self {Vector::new(0.0, 1.0)}pub fn normalize(mut self) -> Self {let f = self.magnitude();self.x /= f;self.y /= f;self}pub fn abs(self) -> Self {Vector::new(self.x.abs(), self.y.abs())}pub fn dot_product(v1: &Vector, v2: &Vector) -> f32 {v1.x * v2.x + v1.y * v2.y}pub fn to_array(&self) -> [f32; 2] {[self.x, self.y]}pub fn magnitude_squared(&self) -> f32 {Vector::dot_product(self, self)}pub fn magnitude(&self) -> f32 {self.magnitude_squared().sqrt()}pub fn angle_radians(&self) -> f32 {self.y.atan2(self.x)}pub fn normal(&self) -> Self {let Vector { x, y } = *self;Vector::new(-y, x).normalize()}}impl From<[f32; 2]> for Vector {fn from(arr: [f32; 2]) -> Self {Vector::new(arr[0], arr[1])}}impl Sub for Vector {type Output = Vector;fn sub(mut self, other: Vector) -> Self::Output {self -= other;self}}impl SubAssign for Vector {fn sub_assign(&mut self, other: Vector) {self.x -= other.x;self.y -= other.y;}}impl Add for Vector {type Output = Vector;fn add(mut self, other: Vector) -> Self::Output {self += other;self}}impl AddAssign for Vector {fn add_assign(&mut self, other: Vector) {self.x += other.x;self.y += other.y;}}impl MulAssign for Vector {fn mul_assign(&mut self, other: Vector) {self.x *= other.x;self.y *= other.y;}}impl Mul for Vector {type Output = Vector;fn mul(mut self, other: Vector) -> Self::Output {self *= other;self}}impl MulAssign<f32> for Vector {fn mul_assign(&mut self, other: f32) {self.x *= other;self.y *= other;}}impl Mul<f32> for Vector {type Output = Vector;fn mul(mut self, other: f32) -> Self::Output {self *= other;self}}impl DivAssign<f32> for Vector {fn div_assign(&mut self, other: f32) {self.x /= other;self.y /= other;}}impl Div<f32> for Vector {type Output = Vector;fn div(mut self, other: f32) -> Self::Output {self /= other;self}}impl DivAssign<Vector> for Vector {fn div_assign(&mut self, other: Vector) {self.x /= other.x;self.y /= other.y;}}impl Div for Vector {type Output = Vector;fn div(mut self, other: Vector) -> Self::Output {self /= other;self}}#[derive(Debug, Default)]pub struct Camera {pub position: Vector,}#[derive(Debug)]pub struct Time {pub delta: std::time::Duration,}#[derive( Debug)]pub struct Object {pub transform: Option<Transform>,pub sprite: Option<Sprite>,pub rigidbody: Option<Rigidbody>,pub script: Option<Script>,pub parent: Option<Weak<RefCell<Object>>>,pub children: Vec<Rc<RefCell<Object>>>,pub custom: Option<Box<dyn std::any::Any>>,}impl Object {pub fn global_transform(&self) -> Option<Transform> {let mut res = self.transform.clone()?;if let Some(parent) = &self.parent {let parent_global_transform =parent.upgrade()?.try_borrow().ok()?.global_transform()?;res.position += parent_global_transform.position;res.scale *= parent_global_transform.scale;}Some(res)}}#[derive( Debug, Clone)]pub struct Transform {pub position: Vector,pub scale: Vector,}impl Default for Transform {fn default() -> Self {Transform {position: Vector::default(),scale: Vector::one(),}}}pub type OnClick = fn(&mut Object);pub type Update = fn(&mut Object, &mut Camera, &Time);#[derive( Default, Debug)]pub struct Script {pub lib: String,}#[derive( Debug)]pub struct Sprite {pub depth: u32,pub color: Option<[f32; 4]>,pub animations: std::collections::HashMap<String, Vec<usize>>,}fn one() -> f32 {1f32}#[derive( Debug)]pub struct Rigidbody {pub mass: f32,pub acceleration: Vector,pub speed: Vector,pub force: Vector,pub is_on_ground: bool,}impl Default for Rigidbody {fn default() -> Self {Rigidbody {mass: 1f32,acceleration: Vector::zero(),speed: Vector::zero(),force: Vector::zero(),is_on_ground: false,}}}