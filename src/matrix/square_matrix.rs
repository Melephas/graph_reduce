use std::{
  default::Default,
  fmt::{
    Display,
    Formatter,
    self
  }
};

use num::Num;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct SquareMatrix<T: Num + Default + ToString + Copy> {
  size: usize,
  data: Vec<T>
}

impl <T: Num + Default + ToString + Copy> SquareMatrix<T> {
  pub fn new() -> SquareMatrix<T> {
    SquareMatrix::default()
  }

  pub fn with_size(size: usize) -> SquareMatrix<T> {
    let mut mat = SquareMatrix::new();
    mat.resize(size);
    mat
  }

  pub fn get(&self, x: usize, y: usize) -> Option<&T> {
    self.data.get((y * self.size) + x)
  }

  pub fn set(&mut self, x: usize, y: usize, value: T) -> Option<T> {
    let index = (y * self.size) + x;

    if index > self.data.len() {
      return None;
    }

    let old = self.data[index];

    self.data[index] = value;

    Some(old)
  }

  pub fn resize(&mut self, size: usize) -> Option<usize> {
    let old_size = self.size;

    self.size = size;
    self.data.resize_with(size * size, Default::default);

    Some(old_size)
  }
}

impl <T: Num + Default + ToString + Copy> Display for SquareMatrix<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let mut repr = String::default();

    for y in 0..self.size {
      for x in 0..self.size {
        repr.push_str(&self.get(x, y).unwrap().to_string());
        if x == self.size - 1 {
          repr.push('\n');
        } else {
          repr.push(' ');
        }
      }
    }

    write!(f, "{}", repr)
  }
}





#[cfg(test)]
mod tests {
  #[test]
  fn display_test() {
    let mut mat = super::SquareMatrix::<i64>::with_size(1);
    mat.set(0, 0, 10);

    assert_eq!(10, *mat.get(0, 0).unwrap());
  }
}