pub enum Judge {
  MissFast(f64),
  NearFast(f64),
  CriticalFast(f64),
  SCritical(f64),
  CriticalLate(f64),
  NearLate(f64),
  MissLate(f64)
}

const FPS: f64 = 60.;

const MISS_TIME: f64 = 13. / FPS;
const NEAR_TIME: f64 = 4. / FPS;
const CRITICAL_TIME: f64 = 2. / FPS;
const SCRITICAL_TIME: f64 = 1. / FPS;

impl Judge {
  pub fn new(delta_time: f64) -> Option<Self> {
    if delta_time <= -MISS_TIME{
      None
    } else if delta_time < -NEAR_TIME{
      Some(Self::MissFast(delta_time))
    } else if delta_time < -CRITICAL_TIME{
      Some(Self::NearFast(delta_time))
    } else if delta_time < -SCRITICAL_TIME{
      Some(Self::CriticalFast(delta_time))
    } else if -SCRITICAL_TIME <= delta_time && delta_time <= SCRITICAL_TIME{
      Some(Self::SCritical(delta_time))
    } else if delta_time > SCRITICAL_TIME{
      Some(Self::CriticalLate(delta_time))
    } else if delta_time > CRITICAL_TIME{
      Some(Self::NearLate(delta_time))
    } else if delta_time > NEAR_TIME{
      Some(Self::MissLate(delta_time))
    } else {
      None // unleachable
    }
  }
}