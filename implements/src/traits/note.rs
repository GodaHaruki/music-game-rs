use crate::judge::Judge;

pub trait Note {
  fn judge(&mut self, time: f64) -> Option<Judge>;
  fn is_judged(&self) -> bool;
}

