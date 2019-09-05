use std::{
  default::Default,
  error::Error,
  fmt::{
    Display,
    Formatter,
    self
  },
  marker::Sized,
  ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Neg
  }
};


pub mod square_matrix;


trait Matrix<T>: Sized + Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Neg {
  fn determinant(&self) -> T where T: num::Num + Default + ToString + Copy;
  fn inverse(&self) -> Result<Self, Box<dyn Error>>;
  fn invert(&mut self) -> Result<(), Box<dyn Error>>;
  fn transposed(&self) -> Self;
  fn transpose(&mut self) -> Self;
  fn conjugate(&self) -> Self;
  fn conjugate_self(&mut self) -> Self;
}