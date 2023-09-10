#[allow(unused)]

use crate::traits::note::Note;
use crate::judge::Judge;

pub struct BTChipNote {
  pub time: f64,
  pub is_judged: bool,
}

impl Note for BTChipNote {
  fn judge(&mut self, time: f64) -> Option<Judge>{
    Judge::new(time - self.time)
  }

  fn is_judged(&self) -> bool {
      self.is_judged
  }
}

impl BTChipNote {
  pub fn new(time: f64) -> Self {
    Self {
      time,
      is_judged: false,
    }
  }
}

pub struct BTLongNote {
  pub time: f64,
  pub time_end: f64,
  pub is_judged: bool,
  judge_times: usize,
}

impl Note for BTLongNote {
  fn judge(&mut self, time: f64) -> Option<Judge>{
    todo!();

    let j = match Judge::new(time - self.time) {
      Some(Judge::CriticalFast(_)) => Some(Judge::SCritical(0.0)),
      Some(Judge::SCritical(_)) => Some(Judge::SCritical(0.0)),
      Some(Judge::CriticalLate(_)) => Some(Judge::SCritical(0.0)),
      _ => None,
    };

    if j.is_none() {
      None
    } else {
      self.judge_times -= 1;
      if self.judge_times == 0 {
        self.is_judged = true;
      };
      j
    }
  }

  fn is_judged(&self) -> bool {
      self.is_judged
  }
}

impl BTLongNote {
  pub fn new(time: f64, time_end: f64) -> Self {
    todo!()
  }
}

pub struct FXChipNote{
  pub time: f64,
  pub is_judged: bool,
}

impl Note for FXChipNote {
  fn judge(&mut self, time: f64) -> Option<Judge>{
    Judge::new(time - self.time)
  }

  fn is_judged(&self) -> bool {
      self.is_judged
  }
}

impl FXChipNote {
  pub fn new(time: f64) -> Self {
    Self {
      time,
      is_judged: false,
    }
  }
}

pub struct FXLongNote {
  pub time: f64,
  pub time_end: f64,
  pub is_judged: bool,
  judge_times: usize,
}

impl Note for FXLongNote {
    fn judge(&mut self, time: f64) -> Option<Judge>{
    todo!();

    let j = match Judge::new(time - self.time) {
      Some(Judge::CriticalFast(_)) => Some(Judge::SCritical(0.0)),
      Some(Judge::SCritical(_)) => Some(Judge::SCritical(0.0)),
      Some(Judge::CriticalLate(_)) => Some(Judge::SCritical(0.0)),
      _ => None,
    };

    if j.is_none() {
      None
    } else {
      self.judge_times -= 1;
      if self.judge_times == 0 {
        self.is_judged = true;
      };
      j
    }
  }

  fn is_judged(&self) -> bool {
      self.is_judged
  }
}

impl FXLongNote {
  fn new(time: f64, time_end: f64) -> Self {
    todo!()
  }
}