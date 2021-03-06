extern crate cgmath;
extern crate num_traits;

use std::collections::VecDeque;
use cgmath::prelude::*;
use cgmath::{BaseFloat, Matrix4, Matrix3, Vector3, Point3};

pub struct MatrixStack<T: BaseFloat + num_traits::Float> {
  current: Matrix4<T>,
  stack: VecDeque<Matrix4<T>>,
}

impl<T: BaseFloat + num_traits::Float> MatrixStack<T> {
  pub fn new() -> MatrixStack<T> {
    MatrixStack {
      current: Matrix4::identity(),
      stack: VecDeque::new(),
    }
  }

  pub fn transform(&mut self, transformation: Matrix4<T>) {
    self.current = self.current * transformation;
  }

  pub fn rotate(&mut self, rotation: Matrix3<T>) {
    self.current = self.current * Matrix4::from(rotation);
  }

  pub fn push(&mut self) {
    self.stack.push_back(self.current);
  }

  pub fn pop(&mut self) -> Matrix4<T> {
    if let Some(stack_top) = self.stack.pop_back() {
      self.current = stack_top;
    } else {
      // At the bottom of the stack you find the identity matrix
      self.current = Matrix4::identity();
    }
    return self.current;
  }

  pub fn transform_vector(& self, target: Vector3<T>) -> Vector3<T> { (self.current * target.extend(<T as cgmath::Zero>::zero())).truncate() }

  pub fn transform_point(& self, target: Point3<T>) -> Point3<T> { Point3::from_homogeneous(self.current * target.to_homogeneous()) }

  /// Transforms the point, but only with rotation and scale, no translation
  pub fn transform_point_no_translate(& self, target: Point3<T>) -> Point3<T> {
    let mut vec_version = target.to_homogeneous();
    // Set to 0.0 so that translation doesn't apply
    vec_version.w = <T as cgmath::Zero>::zero();
    Point3::from_homogeneous(self.current * vec_version)
  }

  pub fn get_matrix(& self) -> Matrix4<T> { self.current }

  pub fn origin(& self) -> Point3<T> { Point3::from_vec(self.current.w.truncate()) }
}
